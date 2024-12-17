use skia_safe::{textlayout::TextAlign, Color, Color4f, IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt,
        color_from_hex_code,
        decoder::CodecExt,
        encoder::make_png_or_gif,
        image::{Fit, ImageExt},
        load_image, local_date, new_paint, new_stroke_paint, new_surface,
        options::Gender,
        text::{text_params, Text2Image},
    },
};

fn ask(
    images: &mut Vec<DecodedImage>,
    _: &Vec<String>,
    options: &Gender,
) -> Result<Vec<u8>, Error> {
    let name = images[0].name.clone();
    let ta = match options.gender.as_str() {
        "male" => "他",
        _ => "她",
    };
    let text = format!("{name}不知道哦。");

    let name_image = Text2Image::from_text(
        name.clone(),
        40.0,
        text_params!(
            font_families = &["HYWenHei"],
            paint = new_paint(color_from_hex_code("#fcbd0d")),
            stroke_paint = new_stroke_paint(Color4f::new(0.0, 0.0, 0.0, 0.7), 3.0),
        ),
    );
    let text_image = Text2Image::from_text(
        text,
        38.0,
        text_params!(
            font_families = &["HYWenHei"],
            paint = new_paint(Color::WHITE),
            stroke_paint = new_stroke_paint(Color4f::new(0.0, 0.0, 0.0, 0.7), 3.0),
        ),
    );
    if name_image.longest_line() > 500.0 {
        return Err(Error::TextOverLength(name));
    }
    let line_width = text_image.longest_line() + 200.0;

    let image = images[0].codec.first_frame()?;
    let image_height = 900;
    let image_width = image.width() * image_height / image.height();
    let image_width = image_width
        .max((line_width + 100.0) as i32)
        .min(image_height);

    let mut surface = new_surface((image_width, image_height));
    let canvas = surface.canvas();
    let mask = load_image("ask/mask.png")?;
    let mask = mask.resize_fit((image_width, image_height), Fit::Cover);
    canvas.draw_image(&mask, (0, 0), None);
    let line = load_image("ask/line.png")?;
    let line = line.resize_exact((line_width as i32, line.height()));
    canvas.draw_image(&line, ((image_width - line_width as i32) / 2, 725), None);
    name_image.draw_on_canvas(
        canvas,
        ((image_width - name_image.longest_line() as i32) / 2, 678),
    );
    text_image.draw_on_canvas(
        canvas,
        ((image_width - text_image.longest_line() as i32) / 2, 740),
    );
    let dialog = surface.image_snapshot();
    let image_height = 640;
    let dialog = dialog.resize_height(image_height);
    let image_width = dialog.width();

    let padding_w = 30;
    let padding_h = 80;
    let frame_width = image_width + padding_w * 2;
    let frame_height = image_height + padding_h * 2;
    let mut surface = new_surface((frame_width, frame_height));
    let canvas = surface.canvas();
    canvas.clear(Color::WHITE);
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding_w, 0, frame_width - padding_w, padding_h),
        format!("让{name}告诉你吧"),
        20.0,
        35.0,
        text_params!(text_align = TextAlign::Left),
    )?;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(
            padding_w,
            frame_height - padding_h,
            frame_width - padding_w,
            frame_height,
        ),
        format!("啊这，{ta}说不知道"),
        20.0,
        35.0,
        text_params!(text_align = TextAlign::Left),
    )?;
    let frame = surface.image_snapshot();

    let func = |images: &Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let image = images[0].resize_fit((image_width, image_height), Fit::Cover);
        canvas.draw_image(&image, (padding_w, padding_h), None);
        canvas.draw_image(&dialog, (padding_w, padding_h), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "ask",
    ask,
    min_images = 1,
    max_images = 1,
    keywords = &["问问"],
    date_created = local_date(2022, 2, 23),
    date_modified = local_date(2023, 2, 14),
);
