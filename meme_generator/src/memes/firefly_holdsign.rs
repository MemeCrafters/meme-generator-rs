use rand::Rng;
use skia_safe::IRect;

use crate::{
    error::Error,
    manager::register_meme,
    meme::{DecodedImage, MemeOptions},
    tags::MemeTags,
    utils::{
        canvas::CanvasExt, color_from_hex_code, encoder::encode_png, image::ImageExt, load_image,
        local_date, new_paint, new_surface, text::TextParams,
    },
};

#[derive(MemeOptions)]
struct Number {
    /// 图片编号
    #[option(short, long, minimum = 0, maximum = 21)]
    number: i32,
}

fn firefly_holdsign(
    _: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    options: &Number,
) -> Result<Vec<u8>, Error> {
    let text = &texts[0];
    let mut num = options.number;
    if num == 0 {
        let mut rng = rand::thread_rng();
        num = rng.gen_range(1..=21);
    }

    let params = [
        (
            (300, 200),
            (144, 322),
            ((0, 66), (276, 0), (319, 178), (43, 244)),
        ),
        (
            (300, 250),
            (-46, -50),
            ((0, 83), (312, 0), (348, 243), (46, 314)),
        ),
        (
            (300, 150),
            (106, 351),
            ((0, 0), (286, 0), (276, 149), (12, 149)),
        ),
        (
            (250, 200),
            (245, -6),
            ((31, 0), (288, 49), (256, 239), (0, 190)),
        ),
        (
            (500, 200),
            (0, 0),
            ((0, 0), (492, 0), (462, 198), (25, 198)),
        ),
        (
            (350, 150),
            (74, 359),
            ((0, 52), (345, 0), (364, 143), (31, 193)),
        ),
        (
            (270, 200),
            (231, -9),
            ((31, 0), (305, 49), (270, 245), (0, 192)),
        ),
        (
            (350, 150),
            (64, 340),
            ((0, 44), (345, 0), (358, 153), (34, 197)),
        ),
        (
            (230, 100),
            (57, 261),
            ((10, 0), (243, 38), (222, 132), (0, 99)),
        ),
        (
            (240, 150),
            (-24, -20),
            ((0, 32), (235, 0), (254, 146), (24, 182)),
        ),
        (
            (230, 140),
            (133, -35),
            ((40, 0), (267, 68), (227, 203), (0, 133)),
        ),
        (
            (169, 124),
            (107, 236),
            ((0, 0), (169, 0), (169, 124), (0, 124)),
        ),
        (
            (210, 140),
            (156, -7),
            ((24, 0), (227, 32), (204, 172), (0, 136)),
        ),
        (
            (250, 123),
            (53, 237),
            ((0, 3), (250, 0), (250, 123), (0, 123)),
        ),
        (
            (200, 140),
            (168, -9),
            ((29, 0), (222, 40), (192, 177), (0, 135)),
        ),
        ((256, 96), (50, 264), ((0, 0), (256, 0), (256, 96), (0, 96))),
        (
            (120, 200),
            (174, 130),
            ((116, 0), (240, 67), (117, 269), (0, 195)),
        ),
        (
            (250, 140),
            (-42, -27),
            ((0, 77), (244, 0), (288, 132), (42, 210)),
        ),
        (
            (230, 130),
            (-64, -42),
            ((0, 110), (229, 0), (294, 126), (64, 245)),
        ),
        (
            (183, 133),
            (0, 227),
            ((0, 0), (183, 9), (183, 133), (0, 133)),
        ),
        (
            (255, 106),
            (50, 254),
            ((2, 4), (256, 0), (257, 106), (0, 106)),
        ),
    ];
    let (size, loc, points) = params[num as usize - 1];

    let mut text_surface = new_surface(size);
    let canvas = text_surface.canvas();
    let padding = 10;
    canvas.draw_text_area_auto_font_size(
        IRect::from_ltrb(padding, padding, size.0 - padding, size.1 - padding),
        text,
        30.0,
        80.0,
        TextParams {
            font_families: vec!["FZShaoEr-M11S".to_string()],
            paint: new_paint(color_from_hex_code("#3b0b07")),
            ..Default::default()
        },
    )?;
    let text_image = text_surface.image_snapshot();

    let frame = load_image(format!("firefly_holdsign/{num:02}.png"))?;
    let mut surface = frame.to_surface();
    let canvas = surface.canvas();
    canvas.draw_image(
        &text_image.perspective(points.0, points.1, points.2, points.3),
        loc,
        None,
    );
    Ok(encode_png(&surface.image_snapshot())?)
}

register_meme!(
    "firefly_holdsign",
    firefly_holdsign,
    min_texts = 1,
    max_texts = 1,
    default_texts = &["我超爱你"],
    keywords = &["流萤举牌"],
    tags = MemeTags::firefly(),
    date_created = local_date(2024, 5, 5),
    date_modified = local_date(2024, 5, 6),
);
