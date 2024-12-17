use skia_safe::{textlayout::TextAlign, Color, Color4f, IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    tags::MemeTags,
    utils::{
        canvas::CanvasExt, encoder::encode_gif, image::ImageExt, load_image, local_date, new_paint,
        new_stroke_paint, new_surface, options::NoOptions, text::text_params,
    },
};

const DEFAULT_TEXT: &str = "傻逼";

fn blamed_mahiro(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let params = [
        (((70, 0), (50, 84), (50, 100), (0, 1)), (98, 94)),
        (((70, 0), (50, 84), (50, 100), (0, 1)), (98, 94)),
        (((55, 0), (55, 128), (45, 208), (0, 32)), (183, 16)),
        (((58, 0), (98, 148), (76, 162), (0, 34)), (254, -4)),
        (((68, 0), (114, 164), (95, 172), (0, 46)), (244, 4)),
        (((64, 0), (126, 156), (98, 172), (0, 54)), (240, 20)),
        (((71, 0), (159, 185), (129, 203), (0, 40)), (202, 70)),
        (((71, 0), (159, 185), (129, 203), (0, 40)), (202, 70)),
        (((67, 0), (143, 187), (111, 201), (0, 33)), (198, 91)),
        (((66, 0), (131, 195), (98, 211), (0, 33)), (186, 83)),
        (((66, 0), (131, 195), (98, 211), (0, 33)), (186, 83)),
        (((65, 0), (108, 195), (81, 204), (0, 28)), (166, 37)),
        (((66, 0), (96, 190), (68, 197), (0, 28)), (164, 8)),
        (((66, 0), (96, 190), (68, 197), (0, 28)), (164, 8)),
        (((70, 0), (91, 186), (65, 197), (0, 26)), (162, -6)),
        (((70, 0), (91, 187), (65, 197), (0, 27)), (158, -20)),
        (((70, 0), (91, 187), (65, 197), (0, 27)), (158, -20)),
        (((74, 0), (82, 190), (58, 197), (0, 30)), (174, -13)),
        (((74, 0), (84, 192), (59, 200), (0, 35)), (182, -12)),
        (((74, 0), (84, 192), (59, 200), (0, 35)), (182, -12)),
        (((78, 0), (86, 196), (62, 205), (0, 35)), (182, -10)),
        (((76, 0), (84, 194), (58, 205), (0, 36)), (188, -9)),
    ];

    let mut frames: Vec<Image> = Vec::new();
    let mut surface = new_surface((400, 80));
    let canvas = surface.canvas();
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(0, -5, 350, 85),
        text,
        60.0,
        80.0,
        text_params!(
            font_families = &["FZKaTong-M19S"],
            text_align = TextAlign::Left,
            paint = new_paint(Color4f::new(0.4, 0.2, 0.3, 1.0)),
            stroke_paint = new_stroke_paint(Color::WHITE, 1.5),
        ),
    )?;
    let text_image = surface.image_snapshot();

    for i in 0..24 {
        let frame = load_image(format!("blamed_mahiro/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        if i >= 2 {
            let (points, pos) = params[i - 2];
            let text_image = text_image.perspective(points.0, points.1, points.2, points.3);
            canvas.draw_image(&text_image, pos, None);
        }
        frames.push(surface.image_snapshot());
    }

    encode_gif(&frames, 0.08)
}

register_meme!(
    "blamed_mahiro",
    blamed_mahiro,
    min_texts = 1,
    max_texts = 1,
    tags = MemeTags::mahiro(),
    keywords = &["真寻挨骂"],
    default_texts = &[DEFAULT_TEXT],
    date_created = local_date(2024, 8, 26),
    date_modified = local_date(2024, 8, 26),
);
