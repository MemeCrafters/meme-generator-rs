use skia_safe::Image;

use crate::{
    error::Error,
    manager::register_meme,
    meme::DecodedImage,
    utils::{
        encoder::{make_gif_or_combined_gif, FrameAlign, GifInfo},
        image::ImageExt,
        load_image, local_date,
        options::NoOptions,
    },
};

fn kiss(images: &mut Vec<DecodedImage>, _: &Vec<String>, _: &NoOptions) -> Result<Vec<u8>, Error> {
    let user_locs = [
        (58, 90),
        (62, 95),
        (42, 100),
        (50, 100),
        (56, 100),
        (18, 120),
        (28, 110),
        (54, 100),
        (46, 100),
        (60, 100),
        (35, 115),
        (20, 120),
        (40, 96),
    ];
    let self_locs = [
        (92, 64),
        (135, 40),
        (84, 105),
        (80, 110),
        (155, 82),
        (60, 96),
        (50, 80),
        (98, 55),
        (35, 65),
        (38, 100),
        (70, 80),
        (84, 65),
        (75, 65),
    ];

    let func = |i: usize, images: &Vec<Image>| {
        let frame = load_image(format!("kiss/{i:02}.png"))?;
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let self_head = images[0].circle().resize_exact((40, 40));
        let user_head = images[1].circle().resize_exact((50, 50));
        canvas.draw_image(&user_head, user_locs[i], None);
        canvas.draw_image(&self_head, self_locs[i], None);
        Ok(surface.image_snapshot())
    };

    make_gif_or_combined_gif(
        images,
        func,
        GifInfo {
            frame_num: 13,
            duration: 0.05,
        },
        FrameAlign::ExtendLoop,
    )
}

register_meme!(
    "kiss",
    kiss,
    min_images = 2,
    max_images = 2,
    keywords = &["亲", "亲亲"],
    date_created = local_date(2021, 6, 11),
    date_modified = local_date(2023, 2, 14),
);
