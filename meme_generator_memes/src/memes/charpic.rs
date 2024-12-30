use skia_safe::{textlayout::TextAlign, Color, Image};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::DecodedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{local_date, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn charpic(images: Vec<DecodedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let str_map = "@@$$&B88QMMGW##EE93SPPDOOU**==()+^,\"--''.  ";
    let text2image = Text2Image::from_text("@", 15.0, text_params!(font_families = &["Consolas"]));
    let ratio = text2image.longest_line() / text2image.height();

    let func = |images: &Vec<Image>| {
        let image = images[0].resize_width(150);
        let image = image.resize_exact((
            image.width(),
            (image.height() as f32 * ratio).round() as i32,
        ));
        let image = image.peek_pixels().unwrap();
        let mut lines = vec![];
        for y in 0..image.height() {
            let mut line = String::new();
            for x in 0..image.width() {
                let color = image.get_color((x, y));
                let gray = (0.2126 * color.r() as f32
                    + 0.7152 * color.g() as f32
                    + 0.0722 * color.b() as f32) as u8;
                let index = (gray as usize * str_map.len() / 256).min(str_map.len() - 1);
                line.push_str(&str_map[index..=index]);
            }
            lines.push(line);
        }
        let text = lines.join("\n");
        let text2image = Text2Image::from_text(
            &text,
            15.0,
            text_params!(font_families = &["Consolas"], text_align = TextAlign::Left),
        );
        let frame_w = text2image.longest_line().ceil() as i32;
        let frame_h = text2image.height().ceil() as i32;
        let mut surface = new_surface((frame_w, frame_h));
        let canvas = surface.canvas();
        canvas.clear(Color::WHITE);
        text2image.draw_on_canvas(canvas, (0, 0));
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "charpic",
    charpic,
    min_images = 1,
    max_images = 1,
    keywords = &["字符画"],
    date_created = local_date(2022, 7, 21),
    date_modified = local_date(2024, 11, 1),
);
