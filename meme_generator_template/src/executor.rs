//! Template execution engine.
//!
//! Coordinates resource loading, element processing, frame iteration,
//! and layer composition to produce the final image/GIF.

use std::collections::HashMap;

use skia_safe::{
    Canvas, Color, FontStyle, IRect, ISize, Image, Paint, Point, Rect, textlayout::TextAlign,
};

use meme_generator_core::{error::Error, meme::OptionValue};
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::{
        FrameAlign, GifEncoder, GifInfo, encode_png, make_gif_or_combined_gif, make_png_or_gif,
    },
    image::{Fit, ImageExt},
    text::TextParams,
    tools::{color_from_str, load_image, new_paint, new_stroke_paint, new_surface},
};

use crate::{
    error::{ContextExt, TemplateError},
    expr::{
        FontSizeInfo, TextDrawInfo, Value, eval_size, eval_str, eval_value_expr, eval_yaml_value,
        interpolate,
    },
    schema::*,
};

// ── Context ──

type Ctx = HashMap<String, Value>;

fn build_context(
    input_images: &[InputImage],
    texts: &[String],
    options: &HashMap<String, OptionValue>,
) -> Ctx {
    let mut ctx = Ctx::new();

    let (image_values, name_values): (Vec<Value>, Vec<Value>) = input_images
        .iter()
        .map(|img| {
            let image = Value::Image(img.image.clone());
            let name = Value::String(img.name.clone());
            (image, name)
        })
        .unzip();
    let text_values: Vec<Value> = texts.iter().map(|t| Value::String(t.clone())).collect();

    let mut input_map = HashMap::new();
    input_map.insert("images".to_string(), Value::Array(image_values));
    input_map.insert("names".to_string(), Value::Array(name_values));
    input_map.insert("texts".to_string(), Value::Array(text_values));
    ctx.insert("input".to_string(), Value::Map(input_map));

    let mut opts_map = HashMap::new();
    for (k, v) in options {
        let val = match v {
            OptionValue::Boolean(b) => Value::Bool(*b),
            OptionValue::String(s) => Value::String(s.clone()),
            OptionValue::Integer(i) => Value::Number(*i as f64),
            OptionValue::Float(f) => Value::Number(*f as f64),
        };
        opts_map.insert(k.clone(), val);
    }
    ctx.insert("options".to_string(), Value::Map(opts_map));

    ctx
}

fn set_resources(ctx: &mut Ctx, resources: &HashMap<String, Value>) {
    ctx.insert("resources".to_string(), Value::Map(resources.clone()));
}

fn set_elements(ctx: &mut Ctx, elements: &HashMap<String, Value>) {
    let existing = ctx
        .entry("elements".to_string())
        .or_insert_with(|| Value::Map(HashMap::new()));
    if let Value::Map(map) = existing {
        for (k, v) in elements {
            map.insert(k.clone(), v.clone());
        }
    }
}

fn set_frame_vars(ctx: &mut Ctx, frame_index: usize, frames: &FramesDef) -> Result<(), Error> {
    let mut frame_map = HashMap::new();
    frame_map.insert("index".to_string(), Value::Number(frame_index as f64));

    for (name, var_def) in &frames.vars {
        let value =
            resolve_frame_var(var_def, frame_index).context(format!("frame_var '{name}'"))?;
        frame_map.insert(name.clone(), value);
    }

    ctx.insert("frames".to_string(), Value::Map(frame_map));
    Ok(())
}

fn resolve_frame_var(var_def: &FrameVarDef, frame_index: usize) -> Result<Value, Error> {
    let yaml_val = var_def.values.get(frame_index).ok_or_else(|| {
        TemplateError::FrameVar(format!(
            "Frame var values index {frame_index} out of bounds"
        ))
    })?;

    parse_typed_yaml(yaml_val, &var_def.var_type.base, &var_def.var_type.dims)
}

/// Recursively parse a YAML value according to declared type and array dims.
fn parse_typed_yaml(
    yaml: &yaml_serde::Value,
    base: &FrameVarBase,
    dims: &[Option<usize>],
) -> Result<Value, Error> {
    if let Some((&expected_len, rest)) = dims.split_first() {
        // Expect a YAML sequence, recurse with remaining dims
        let arr = yaml
            .as_sequence()
            .ok_or_else(|| TemplateError::FrameVar(format!("Expected array, got {yaml:?}")))?;
        if let Some(n) = expected_len {
            if arr.len() != n {
                return Err(TemplateError::FrameVar(format!(
                    "Expected array of length {n}, got {}",
                    arr.len()
                ))
                .into());
            }
        }
        let values = arr
            .iter()
            .map(|v| parse_typed_yaml(v, base, rest))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Value::Array(values))
    } else {
        match base {
            FrameVarBase::Number => {
                let n = yaml.as_f64().ok_or_else(|| {
                    TemplateError::FrameVar(format!("Expected number, got {yaml:?}"))
                })?;
                Ok(Value::Number(n))
            }
            FrameVarBase::String => {
                let s = yaml.as_str().ok_or_else(|| {
                    TemplateError::FrameVar(format!("Expected string, got {yaml:?}"))
                })?;
                Ok(Value::String(s.to_string()))
            }
            FrameVarBase::Bool => {
                let b = yaml.as_bool().ok_or_else(|| {
                    TemplateError::FrameVar(format!("Expected bool, got {yaml:?}"))
                })?;
                Ok(Value::Bool(b))
            }
        }
    }
}

// ── Resource loading ──

fn load_resources(
    defs: &indexmap::IndexMap<String, ResourceDef>,
    ctx: &Ctx,
) -> Result<HashMap<String, Value>, Error> {
    let mut resources = HashMap::new();
    for (name, def) in defs {
        let value = (|| -> Result<Value, Error> {
            match def {
                ResourceDef::Single(path_template) => {
                    let path = interpolate(path_template, ctx)?;
                    let img = load_image(&path)?;
                    Ok(Value::Image(img))
                }
                ResourceDef::Sequence {
                    pattern,
                    count,
                    start,
                } => {
                    let mut images = Vec::new();
                    for i in *start..(*start + *count) {
                        let mut seq_ctx = ctx.clone();
                        seq_ctx.insert("index".to_string(), Value::Number(i as f64));
                        let path = interpolate(pattern, &seq_ctx)?;
                        let img = load_image(&path)?;
                        images.push(Value::Image(img));
                    }
                    Ok(Value::Array(images))
                }
            }
        })()
        .context(format!("resource '{name}'"))?;
        resources.insert(name.clone(), value);
    }
    Ok(resources)
}

// ── Element processing ──

fn process_elements(
    defs: &indexmap::IndexMap<String, ElementDef>,
    ctx: &mut Ctx,
    scope_filter: &Scope,
) -> Result<HashMap<String, Value>, Error> {
    let mut results = HashMap::new();
    for (name, def) in defs {
        let (elem_scope, condition) = match def {
            ElementDef::Image(d) => (&d.scope, d.condition.as_deref()),
            ElementDef::Text(d) => (&d.scope, d.condition.as_deref()),
            ElementDef::Canvas(d) => (&d.scope, d.condition.as_deref()),
        };
        if elem_scope != scope_filter {
            continue;
        }
        if let Some(cond) = condition {
            let val = eval_str(cond, ctx)
                .and_then(|v| v.as_bool().map_err(Into::into))
                .context(format!("element '{name}' condition"))?;
            if !val {
                continue;
            }
        }
        let value = process_element(def, ctx).context(format!("element '{name}'"))?;
        // Update elements in context so later elements can reference earlier ones
        {
            let elements = ctx
                .entry("elements".to_string())
                .or_insert_with(|| Value::Map(HashMap::new()));
            if let Value::Map(map) = elements {
                map.insert(name.clone(), value.clone());
            }
        }
        results.insert(name.clone(), value);
    }
    Ok(results)
}

fn process_element(def: &ElementDef, ctx: &Ctx) -> Result<Value, Error> {
    match def {
        ElementDef::Image(d) => process_image_element(d, ctx),
        ElementDef::Text(d) => process_text_element(d, ctx),
        ElementDef::Canvas(d) => process_canvas_element(d, ctx),
    }
}

fn process_image_element(def: &ImageElementDef, ctx: &Ctx) -> Result<Value, Error> {
    let mut img = resolve_image_ref(&def.from, ctx)?;
    img = apply_operations(&img, &def.operations, ctx)?;
    Ok(Value::Image(img))
}

fn process_text_element(def: &TextElementDef, ctx: &Ctx) -> Result<Value, Error> {
    let text = interpolate(&def.text, ctx)?;
    let text_params = build_text_params(def, ctx)?;

    let font_size = match &def.font_size {
        FontSizeDef::Fixed(size_expr) => {
            let size = eval_value_expr(size_expr, ctx)?.as_f32()?;
            FontSizeInfo::Fixed(size)
        }
        FontSizeDef::Range(min, max) => FontSizeInfo::Range(*min as f32, *max as f32),
    };

    let bound = if let Some(bound_expr) = &def.bound {
        let bw = eval_value_expr(&bound_expr[0], ctx)?.as_f32()?;
        let bh = eval_value_expr(&bound_expr[1], ctx)?.as_f32()?;
        Some((bw, bh))
    } else {
        None
    };

    if matches!(font_size, FontSizeInfo::Range(_, _)) && bound.is_none() {
        return Err(TemplateError::Element(
            "font_size range requires 'bound' to be set".to_string(),
        )
        .into());
    }

    Ok(Value::Text(TextDrawInfo {
        text,
        text_params,
        font_size,
        bound,
        bbcode: def.bbcode,
    }))
}

fn process_canvas_element(def: &CanvasElementDef, ctx: &Ctx) -> Result<Value, Error> {
    let (w, h) = eval_size(&def.size, ctx)?;
    let mut surface = new_surface((w, h));
    let canvas = surface.canvas();

    if let Some(bg) = &def.background {
        let resolved = interpolate(bg, ctx)?;
        let color = parse_color(&resolved);
        canvas.clear(color);
    }

    // Composite layers on this off-screen canvas
    compose_layers(&def.layers, canvas, ctx)?;

    let mut img = surface.image_snapshot();
    img = apply_operations(&img, &def.operations, ctx)?;
    Ok(Value::Image(img))
}

fn build_text_params(def: &TextElementDef, ctx: &Ctx) -> Result<TextParams, Error> {
    let mut params = TextParams::default();

    if let Some(color_str) = &def.color {
        let resolved = interpolate(color_str, ctx)?;
        params.paint = new_paint(parse_color(&resolved));
    }

    if let Some(stroke) = &def.stroke {
        let resolved = interpolate(&stroke.color, ctx)?;
        let color = parse_color(&resolved);
        params.stroke_paint = Some(new_stroke_paint(color, stroke.width));
    }

    if let Some(families) = &def.font_families {
        params.font_families = families.clone();
    }

    if let Some(style_str) = &def.font_style {
        params.font_style = parse_font_style(style_str)?;
    }

    if let Some(align_str) = &def.text_align {
        params.text_align = parse_text_align(align_str)?;
    }

    Ok(params)
}

fn parse_font_style(s: &str) -> Result<FontStyle, Error> {
    match s {
        "normal" => Ok(FontStyle::normal()),
        "bold" => Ok(FontStyle::bold()),
        "italic" => Ok(FontStyle::italic()),
        "bold_italic" => Ok(FontStyle::bold_italic()),
        _ => Err(TemplateError::Config(format!("Unknown font_style: '{s}'")).into()),
    }
}

fn parse_text_align(s: &str) -> Result<TextAlign, Error> {
    match s {
        "left" => Ok(TextAlign::Left),
        "center" => Ok(TextAlign::Center),
        "right" => Ok(TextAlign::Right),
        _ => Err(TemplateError::Config(format!("Unknown text_align: '{s}'")).into()),
    }
}

fn parse_color(s: &str) -> Color {
    color_from_str(s)
}

// ── Reference resolution ──

fn resolve_image_ref(ref_path: &str, ctx: &Ctx) -> Result<Image, Error> {
    let val = eval_str(ref_path, ctx)?;
    Ok(val.as_image().cloned()?)
}

fn resolve_layer_ref(ref_path: &str, ctx: &Ctx) -> Result<Value, Error> {
    Ok(eval_str(ref_path, ctx)?)
}

// ── Operations ──

fn apply_operations(img: &Image, ops: &[OperationDef], ctx: &Ctx) -> Result<Image, Error> {
    let mut current = img.clone();
    for op in ops {
        let op_ctx = format!("operation '{}'", op.name);
        // Check condition
        if let Some(cond) = &op.condition {
            let val = eval_str(cond, ctx).context(&op_ctx)?;
            if !val.as_bool().context(&op_ctx)? {
                continue;
            }
        }
        current = apply_single_operation(&current, &op.name, &op.args, ctx).context(&op_ctx)?;
    }
    Ok(current)
}

fn apply_single_operation(
    img: &Image,
    name: &str,
    args: &Option<yaml_serde::Value>,
    ctx: &Ctx,
) -> Result<Image, Error> {
    match name {
        // No-args operations
        "square" => Ok(img.square()),
        "circle" => Ok(img.circle()),
        "grayscale" => Ok(img.grayscale()),
        "invert" => Ok(img.invert()),
        "flip_horizontal" => Ok(img.flip_horizontal()),
        "flip_vertical" => Ok(img.flip_vertical()),

        // Single arg
        "resize_width" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.resize_width(v.as_i32()?))
        }
        "resize_height" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.resize_height(v.as_i32()?))
        }
        "rotate" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.rotate(v.as_f32()?))
        }
        "rotate_crop" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.rotate_crop(v.as_f32()?))
        }
        "round_corner" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.round_corner(v.as_f32()?))
        }
        "gaussian_blur" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.gaussian_blur(v.as_f32()?))
        }
        "transparency" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.transparency(v.as_f32()?))
        }
        "brightness" => {
            let v = eval_op_arg(args, ctx)?;
            Ok(img.brightness(v.as_f32()?))
        }
        "colorize" => {
            let raw = args.as_ref().and_then(|v| v.as_str()).ok_or_else(|| {
                TemplateError::Operation("colorize requires a color string".to_string())
            })?;
            let resolved = interpolate(raw, ctx)?;
            let color = parse_color(&resolved);
            Ok(img.colorize(color))
        }

        // Array args
        "resize_exact" => {
            let v = eval_op_arg(args, ctx)?;
            let arr = v.as_array()?;
            if arr.len() != 2 {
                return Err(
                    TemplateError::Operation("resize_exact requires [w, h]".to_string()).into(),
                );
            }
            Ok(img.resize_exact(ISize::new(arr[0].as_i32()?, arr[1].as_i32()?)))
        }
        "crop" => {
            let v = eval_op_arg(args, ctx)?;
            let arr = v.as_array()?;
            if arr.len() != 4 {
                return Err(
                    TemplateError::Operation("crop requires [l, t, r, b]".to_string()).into(),
                );
            }
            let rect = IRect::from_ltrb(
                arr[0].as_i32()?,
                arr[1].as_i32()?,
                arr[2].as_i32()?,
                arr[3].as_i32()?,
            );
            Ok(img.crop(&rect))
        }
        "perspective" => {
            let v = eval_op_arg(args, ctx)?;
            let arr = v.as_array()?;
            if arr.len() != 4 {
                return Err(
                    TemplateError::Operation("perspective requires 4 points".to_string()).into(),
                );
            }
            let mut points: [Point; 4] = [Point::default(); 4];
            for (i, p) in arr.iter().enumerate() {
                let pt = p.as_array()?;
                if pt.len() != 2 {
                    return Err(TemplateError::Operation(
                        "Each perspective point must be [x, y]".to_string(),
                    )
                    .into());
                }
                points[i] = Point::new(pt[0].as_f32()?, pt[1].as_f32()?);
            }
            Ok(img.perspective(&points))
        }

        // Object args
        "resize_fit" | "resize_bound" => {
            let yaml = args
                .as_ref()
                .ok_or_else(|| TemplateError::Operation(format!("{name} requires arguments")))?;
            let map = yaml.as_mapping().ok_or_else(|| {
                TemplateError::Operation(format!("{name} requires {{size, fit}}"))
            })?;

            let size_yaml = map
                .get(yaml_serde::Value::String("size".to_string()))
                .ok_or_else(|| TemplateError::Operation(format!("{name} requires 'size'")))?;
            let size_val = eval_yaml_value(size_yaml, ctx)?;
            let size_arr = size_val.as_array()?;
            let size = ISize::new(size_arr[0].as_i32()?, size_arr[1].as_i32()?);

            let fit_yaml = map
                .get(yaml_serde::Value::String("fit".to_string()))
                .ok_or_else(|| TemplateError::Operation(format!("{name} requires 'fit'")))?;
            let fit_str = fit_yaml
                .as_str()
                .ok_or_else(|| TemplateError::Operation("fit must be a string".to_string()))?;
            let fit = match fit_str {
                "contain" => Fit::Contain,
                "cover" => Fit::Cover,
                _ => {
                    return Err(
                        TemplateError::Operation(format!("Unknown fit: '{fit_str}'")).into(),
                    );
                }
            };

            if name == "resize_fit" {
                Ok(img.resize_fit(size, fit))
            } else {
                Ok(img.resize_bound(size, fit))
            }
        }

        _ => Err(TemplateError::Operation(format!("Unknown operation: '{name}'")).into()),
    }
}

fn eval_op_arg(args: &Option<yaml_serde::Value>, ctx: &Ctx) -> Result<Value, Error> {
    let yaml = args
        .as_ref()
        .ok_or_else(|| TemplateError::Operation("Operation requires arguments".to_string()))?;
    eval_yaml_value(yaml, ctx).map_err(Into::into)
}

// ── Layer composition ──

fn compose_layers(layers: &[LayerDef], canvas: &Canvas, ctx: &Ctx) -> Result<(), Error> {
    for (i, layer) in layers.iter().enumerate() {
        (|| -> Result<(), Error> {
            // Check condition
            if let Some(cond) = &layer.condition {
                let val = eval_str(cond, ctx)?;
                if !val.as_bool()? {
                    return Ok(());
                }
            }

            let element = resolve_layer_ref(&layer.use_ref, ctx)?;

            let (x, y) = if let Some(pos) = &layer.position {
                let val = eval_value_expr(pos, ctx)?;
                let arr = val.as_array()?;
                (arr[0].as_f32()?, arr[1].as_f32()?)
            } else {
                (0.0, 0.0)
            };

            match &element {
                Value::Text(info) => {
                    draw_text_on_canvas(canvas, info, x, y)?;
                }
                Value::Image(img) => {
                    if let Some(opacity_expr) = &layer.opacity {
                        let alpha = eval_value_expr(opacity_expr, ctx)?.as_f32()?;
                        let mut paint = Paint::default();
                        paint.set_alpha_f(alpha);
                        canvas.draw_image(img, Point::new(x, y), Some(&paint));
                    } else {
                        canvas.draw_image(img, Point::new(x, y), None);
                    }
                }
                other => {
                    return Err(TemplateError::Layer(format!(
                        "Layer 'use' must reference an image or text element, got {other:?}"
                    ))
                    .into());
                }
            }
            Ok(())
        })()
        .context(format!("layer #{i} (use: {})", layer.use_ref))?;
    }
    Ok(())
}

fn draw_text_on_canvas(canvas: &Canvas, info: &TextDrawInfo, x: f32, y: f32) -> Result<(), Error> {
    match (&info.font_size, &info.bound) {
        (FontSizeInfo::Range(min, max), Some((bw, bh))) => {
            let rect = Rect::from_xywh(x, y, *bw, *bh);
            if info.bbcode {
                canvas.draw_bbcode_text_area_auto_font_size(
                    rect,
                    &info.text,
                    *min,
                    *max,
                    info.text_params.clone(),
                )?;
            } else {
                canvas.draw_text_area_auto_font_size(
                    rect,
                    &info.text,
                    *min,
                    *max,
                    info.text_params.clone(),
                )?;
            }
        }
        (FontSizeInfo::Fixed(font_size), Some((bw, bh))) => {
            let rect = Rect::from_xywh(x, y, *bw, *bh);
            if info.bbcode {
                canvas.draw_bbcode_text_area(
                    rect,
                    &info.text,
                    *font_size,
                    info.text_params.clone(),
                )?;
            } else {
                canvas.draw_text_area(rect, &info.text, *font_size, info.text_params.clone())?;
            }
        }
        (FontSizeInfo::Fixed(font_size), None) => {
            let origin = Point::new(x, y);
            if info.bbcode {
                canvas.draw_bbcode_text(origin, &info.text, *font_size, info.text_params.clone());
            } else {
                canvas.draw_text(origin, &info.text, *font_size, info.text_params.clone());
            }
        }
        (FontSizeInfo::Range(_, _), None) => {
            return Err(TemplateError::Layer(
                "font_size range requires 'bound' to be set".to_string(),
            )
            .into());
        }
    }
    Ok(())
}

// ── Frame rendering ──

fn render_frame(template: &Template, ctx: &mut Ctx) -> Result<Image, Error> {
    let (w, h) = eval_size(&template.canvas.size, ctx).context("canvas")?;

    // Inject canvas size into context
    let mut canvas_map = HashMap::new();
    canvas_map.insert("width".to_string(), Value::Number(w as f64));
    canvas_map.insert("height".to_string(), Value::Number(h as f64));
    canvas_map.insert(
        "size".to_string(),
        Value::Array(vec![Value::Number(w as f64), Value::Number(h as f64)]),
    );
    ctx.insert("canvas".to_string(), Value::Map(canvas_map));

    let mut surface = new_surface((w, h));
    let canvas = surface.canvas();

    if let Some(bg) = &template.canvas.background {
        let resolved = interpolate(bg, ctx)?;
        let color = parse_color(&resolved);
        canvas.clear(color);
    }

    compose_layers(&template.layers, canvas, ctx)?;
    Ok(surface.image_snapshot())
}

// ── Main execution entry point ──

pub fn execute_template(
    template: &Template,
    images: Vec<InputImage>,
    texts: Vec<String>,
    options: &HashMap<String, OptionValue>,
) -> Result<Vec<u8>, Error> {
    // Build base context
    let mut ctx = build_context(&images, &texts, options);

    // Load resources
    let resources = load_resources(&template.resources, &ctx)?;
    set_resources(&mut ctx, &resources);

    // Initialize elements map in context
    ctx.insert("elements".to_string(), Value::Map(HashMap::new()));

    // Process static elements
    let static_elements = process_elements(&template.elements, &mut ctx, &Scope::Static)?;
    set_elements(&mut ctx, &static_elements);

    // Determine execution mode
    match (&template.frames, template.config.gif_input) {
        // GIF with fixed frames + GIF input support
        (Some(frames), true) => {
            let target_info = GifInfo {
                frame_num: frames.count as u32,
                duration: frames.duration,
            };
            let align = parse_frame_align(frames.align.as_deref());

            make_gif_or_combined_gif(
                images,
                |frame_idx, frame_images| {
                    let mut frame_ctx = ctx.clone();

                    // Update input images with per-frame decomposed images
                    update_input_images(&mut frame_ctx, &frame_images);

                    // Set frame variables
                    set_frame_vars(&mut frame_ctx, frame_idx, frames)?;

                    // Process frame-scoped elements
                    let dynamic_elements =
                        process_elements(&template.elements, &mut frame_ctx, &Scope::Dynamic)?;
                    set_elements(&mut frame_ctx, &dynamic_elements);

                    render_frame(template, &mut frame_ctx)
                },
                target_info,
                align,
            )
        }

        // GIF with fixed frames, no GIF input
        (Some(frames), false) => {
            let mut encoder = GifEncoder::new();
            for frame_idx in 0..frames.count {
                let mut frame_ctx = ctx.clone();

                set_frame_vars(&mut frame_ctx, frame_idx, frames)?;

                let dynamic_elements =
                    process_elements(&template.elements, &mut frame_ctx, &Scope::Dynamic)?;
                set_elements(&mut frame_ctx, &dynamic_elements);

                let image = render_frame(template, &mut frame_ctx)?;
                encoder.add_frame(image, frames.duration)?;
            }
            encoder.finish()
        }

        // Static output, accepts GIF input
        (None, true) => {
            make_png_or_gif(images, |frame_images| {
                let mut frame_ctx = ctx.clone();
                update_input_images(&mut frame_ctx, &frame_images);

                // Re-process dynamic elements with per-frame input
                let dynamic_elements =
                    process_elements(&template.elements, &mut frame_ctx, &Scope::Dynamic)?;
                set_elements(&mut frame_ctx, &dynamic_elements);

                render_frame(template, &mut frame_ctx)
            })
        }

        // Pure static (PNG)
        (None, false) => {
            let dynamic_elements = process_elements(&template.elements, &mut ctx, &Scope::Dynamic)?;
            set_elements(&mut ctx, &dynamic_elements);
            let image = render_frame(template, &mut ctx)?;
            encode_png(image)
        }
    }
}

fn update_input_images(ctx: &mut Ctx, frame_images: &[Image]) {
    let image_values: Vec<Value> = frame_images
        .iter()
        .map(|img| Value::Image(img.clone()))
        .collect();
    if let Some(Value::Map(input)) = ctx.get_mut("input") {
        input.insert("images".to_string(), Value::Array(image_values));
    }
}

fn parse_frame_align(align: Option<&str>) -> Option<FrameAlign> {
    match align {
        Some("extend_loop") => Some(FrameAlign::ExtendLoop),
        Some("extend_last") => Some(FrameAlign::ExtendLast),
        Some("extend_first") => Some(FrameAlign::ExtendFirst),
        Some("no_extend") => Some(FrameAlign::NoExtend),
        _ => None,
    }
}
