use skia_safe::{textlayout::TextAlign, Color, IRect};

use meme_generator_core::error::Error;
use meme_generator_utils::{
    builder::NamedImage,
    encoder::encode_png,
    image::ImageExt,
    text::Text2Image,
    text_params,
    tools::{color_from_hex_code, load_image, local_date, new_paint, new_surface},
};

use crate::{options::NoOptions, register_meme};

fn my_friend_say(
    images: Vec<NamedImage>,
    texts: Vec<String>,
    _: NoOptions,
) -> Result<Vec<u8>, Error> {
    let name = &images[0].name;

    let name_img = Text2Image::from_text(
        name,
        25.0,
        text_params!(paint = new_paint(color_from_hex_code("#868894"))),
    );
    let name_w = name_img.longest_line() as i32;
    let name_h = name_img.height() as i32;
    if name_w >= 600 {
        return Err(Error::TextOverLength(name.to_string()));
    }

    let corner1 = load_image("my_friend/corner1.png")?;
    let corner2 = load_image("my_friend/corner2.png")?;
    let corner3 = load_image("my_friend/corner3.png")?;
    let corner4 = load_image("my_friend/corner4.png")?;
    let label = load_image("my_friend/label.png")?;

    let avatar = images[0].image.circle().resize_exact((100, 100));

    let make_dialog = |text: &str| {
        let mut text_img =
            Text2Image::from_text(text, 40.0, text_params!(text_align = TextAlign::Left));
        text_img.layout(600.0);
        let text_w = text_img.longest_line() as i32;
        let text_h = text_img.height() as i32;
        let box_w = text_w.max(name_w + 15) + 140;
        let box_h = (text_h + 103).max(150);

        let mut box_surface = new_surface((box_w, box_h));
        let canvas = box_surface.canvas();
        canvas.draw_image(&corner1, (0, 0), None);
        canvas.draw_image(&corner2, (0, box_h - 75), None);
        canvas.draw_image(&corner3, (text_w + 70, 0), None);
        canvas.draw_image(&corner4, (text_w + 70, box_h - 75), None);
        canvas.draw_irect(
            IRect::from_xywh(70, 20, text_w, box_h - 40),
            &new_paint(Color::WHITE),
        );
        canvas.draw_irect(
            IRect::from_xywh(27, 75, text_w + 88, box_h - 150),
            &new_paint(Color::WHITE),
        );
        text_img.draw_on_canvas(&canvas, (70, 20 + (box_h - 40 - text_h) / 2));
        let box_img = box_surface.image_snapshot();

        let mut dialog_surface = new_surface((box_img.width() + 130, box_img.height() + 60));
        let canvas = dialog_surface.canvas();
        canvas.clear(color_from_hex_code("#eaedf4"));
        canvas.draw_image(&avatar, (20, 20), None);
        canvas.draw_image(&box_img, (130, 60), None);
        canvas.draw_image(&label, (160, 25), None);
        name_img.draw_on_canvas(&canvas, (260, 22 + (35 - name_h) / 2));
        dialog_surface.image_snapshot()
    };

    let dialogs = texts
        .iter()
        .map(|text| make_dialog(text))
        .collect::<Vec<_>>();

    let frame_w = dialogs.iter().map(|d| d.width()).max().unwrap();
    let frame_h = dialogs.iter().map(|d| d.height()).sum();
    let mut surface = new_surface((frame_w, frame_h));
    let canvas = surface.canvas();
    canvas.clear(color_from_hex_code("#eaedf4"));

    let mut current_h = 0;
    for dialog in dialogs {
        canvas.draw_image(&dialog, (0, current_h), None);
        current_h += dialog.height();
    }
    encode_png(surface.image_snapshot())
}

register_meme!(
    "my_friend_say",
    my_friend_say,
    min_images = 1,
    max_images = 1,
    min_texts = 1,
    max_texts = 10,
    default_texts = &["让我康康"],
    keywords = &["我朋友说"],
    date_created = local_date(2022, 3, 11),
    date_modified = local_date(2023, 2, 14),
);
