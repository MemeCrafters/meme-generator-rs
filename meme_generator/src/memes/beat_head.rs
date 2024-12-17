use skia_safe::{IRect, Image};

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        canvas::CanvasExt,
        encoder::{make_gif_or_combined_gif, GifInfo},
        image::ImageExt,
        load_image, local_date, new_surface,
        options::NoOptions,
    },
};

const DEFAULT_TEXT: &str = "怎么说话的你";

fn beat_head(
    images: &mut Vec<DecodedImage>,
    texts: &Vec<String>,
    _: &NoOptions,
) -> Result<Vec<u8>, Error> {
    let text = if !texts.is_empty() {
        &texts[0]
    } else {
        DEFAULT_TEXT
    };

    let locs = [(160, 121, 76, 76), (172, 124, 69, 69), (208, 166, 52, 52)];

    let func = |i: usize, images: &Vec<Image>| {
        let (x, y, w, h) = locs[i];
        let head = images[0].circle().resize_exact((w, h));
        let frame = load_image(format!("beat_head/{i}.png"))?;
        let mut surface = new_surface(frame.dimensions());
        let canvas = surface.canvas();
        canvas.draw_image(&head, (x, y), None);
        canvas.draw_image(&frame, (0, 0), None);
        canvas.draw_text_area_auto_font_size(
            IRect::from_ltrb(175, 28, 316, 82),
            text,
            10.0,
            50.0,
            None,
        )?;
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 3,
            duration: 0.05,
        },
        None,
    )
}

register_meme!(
    "beat_head",
    beat_head,
    min_images = 1,
    max_images = 1,
    min_texts = 0,
    max_texts = 1,
    default_texts = &[DEFAULT_TEXT],
    keywords = &["拍头"],
    date_created = local_date(2023, 3, 8),
    date_modified = local_date(2023, 3, 8),
);
