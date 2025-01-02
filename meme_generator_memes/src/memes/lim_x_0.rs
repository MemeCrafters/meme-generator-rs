use skia_safe::Image;

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::make_png_or_gif,
    image::ImageExt,
    tools::{load_image, local_date},
};

use crate::{options::NoOptions, register_meme};

fn lim_x_0(images: Vec<NamedImage>, _: Vec<String>, _: NoOptions) -> Result<Vec<u8>, Error> {
    let frame = load_image("lim_x_0/0.png")?;

    let locs = [
        (143, 32),
        (155, 148),
        (334, 149),
        (275, 266),
        (486, 266),
        (258, 383),
        (439, 382),
        (343, 539),
        (577, 487),
        (296, 717),
        (535, 717),
        (64, 896),
        (340, 896),
        (578, 897),
        (210, 1038),
        (644, 1039),
        (64, 1192),
        (460, 1192),
        (698, 1192),
        (1036, 141),
        (1217, 141),
        (1243, 263),
        (1140, 378),
        (1321, 378),
        (929, 531),
        (1325, 531),
        (1592, 531),
        (1007, 687),
        (1390, 687),
        (1631, 686),
        (1036, 840),
        (1209, 839),
        (1447, 839),
        (1141, 1018),
        (1309, 1019),
        (1546, 1019),
        (1037, 1197),
        (1317, 1198),
        (1555, 1197),
    ];

    let func = |images: Vec<Image>| {
        let mut surface = frame.to_surface();
        let canvas = surface.canvas();
        let img_c = images[0].circle().resize_exact((72, 72));
        let img_tp = images[0].circle().resize_exact((51, 51));
        for loc in locs {
            canvas.draw_image(&img_c, loc, None);
        }
        canvas.draw_image(&img_tp, (948, 247), None);
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

register_meme!(
    "lim_x_0",
    lim_x_0,
    min_images = 1,
    max_images = 1,
    keywords = &["等价无穷小"],
    date_created = local_date(2023, 1, 9),
    date_modified = local_date(2023, 2, 14),
);
