use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::InputImage,
    canvas::CanvasExt,
    encoder::encode_gif,
    image::ImageExt,
    text_params,
    tools::{load_image, local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn psyduck(_: Vec<InputImage>, texts: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let text_image = |text: &str| -> Result<Image, Error> {
        let mut surface = new_surface((155, 100));
        let canvas = surface.canvas();
        let padding = 5;
        canvas.draw_text_area_auto_font_size(
            skia_safe::IRect::from_ltrb(padding, padding, 155 - padding, 100 - padding),
            text,
            30.0,
            80.0,
            text_params!(font_families = &["FZSJ-QINGCRJ"]),
        )?;
        Ok(surface.image_snapshot())
    };

    let left_img = text_image(&texts[0])?;
    let right_img = text_image(&texts[1])?;

    let params = [
        ("left", [(0, 11), (154, 0), (161, 89), (20, 104)], (18, 42)),
        ("left", [(0, 9), (153, 0), (159, 89), (20, 101)], (15, 38)),
        ("left", [(0, 7), (148, 0), (156, 89), (21, 97)], (14, 23)),
        ("", [(0, 0), (0, 0), (0, 0), (0, 0)], (0, 0)),
        (
            "right",
            [(10, 0), (143, 17), (124, 104), (0, 84)],
            (298, 18),
        ),
        (
            "right",
            [(13, 0), (143, 27), (125, 113), (0, 83)],
            (298, 30),
        ),
        (
            "right",
            [(13, 0), (143, 27), (125, 113), (0, 83)],
            (298, 26),
        ),
        (
            "right",
            [(13, 0), (143, 27), (125, 113), (0, 83)],
            (298, 30),
        ),
        (
            "right",
            [(13, 0), (143, 27), (125, 113), (0, 83)],
            (302, 20),
        ),
        (
            "right",
            [(13, 0), (141, 23), (120, 102), (0, 82)],
            (300, 24),
        ),
        (
            "right",
            [(13, 0), (140, 22), (118, 100), (0, 82)],
            (299, 22),
        ),
        ("right", [(9, 0), (128, 16), (109, 89), (0, 80)], (303, 23)),
        ("", [(0, 0), (0, 0), (0, 0), (0, 0)], (0, 0)),
        ("left", [(0, 13), (152, 0), (158, 89), (17, 109)], (35, 36)),
        ("left", [(0, 13), (152, 0), (158, 89), (17, 109)], (31, 29)),
        ("left", [(0, 17), (149, 0), (155, 90), (17, 120)], (45, 33)),
        ("left", [(0, 14), (152, 0), (156, 91), (17, 115)], (40, 27)),
        ("left", [(0, 12), (154, 0), (158, 90), (17, 109)], (35, 28)),
    ];

    let mut frames = Vec::new();
    for i in 0..18 {
        let frame = load_image(format!("psyduck/{i:02}.jpg"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let (side, points, pos) = params[i];
        if side == "left" {
            canvas.draw_image(&left_img.perspective(&points), pos, None);
        } else if side == "right" {
            canvas.draw_image(&right_img.perspective(&points), pos, None);
        }
        frames.push(surface.image_snapshot());
    }

    encode_gif(frames, 0.2)
}

register_meme!(
    "psyduck",
    psyduck,
    min_texts = 2,
    max_texts = 2,
    default_texts = &["来份", "涩图"],
    keywords = &["可达鸭"],
    date_created = local_date(2022, 6, 14),
    date_modified = local_date(2023, 2, 14),
);
