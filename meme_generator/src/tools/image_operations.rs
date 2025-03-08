use meme_generator_core::{error::Error, meme};
use meme_generator_utils::{
    builder::InputImage,
    decoder::CodecExt,
    encoder::{GifEncoder, encode_png, make_png_or_gif},
    image::{Fit, ImageExt},
    tools::new_surface,
};
use serde::{Deserialize, Serialize};
use skia_safe::{Codec, Data, IRect, Image};

fn decode_image(data: Vec<u8>) -> Result<Codec<'static>, Error> {
    let data = Data::new_copy(&data);
    Codec::from_data(data).ok_or(Error::ImageDecodeError("Skia decode error".to_string()))
}

fn input_image(data: Vec<u8>) -> Result<InputImage<'static>, Error> {
    let image = meme::Image {
        name: String::new(),
        data,
    };
    InputImage::from(&image)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInfo {
    pub width: i32,
    pub height: i32,
    pub is_multi_frame: bool,
    pub frame_count: Option<i32>,
    pub average_duration: Option<f32>,
}

pub fn inspect(image: Vec<u8>) -> Result<ImageInfo, Error> {
    let mut codec = decode_image(image)?;
    let is_multi_frame = codec.is_multi_frame();
    let frame_count = if is_multi_frame {
        Some(codec.get_frame_count() as i32)
    } else {
        None
    };
    let average_duration = if is_multi_frame {
        Some(codec.get_average_duration()?)
    } else {
        None
    };
    Ok(ImageInfo {
        width: codec.dimensions().width,
        height: codec.dimensions().height,
        is_multi_frame,
        frame_count,
        average_duration,
    })
}

pub fn flip_horizontal(image: Vec<u8>) -> Result<Vec<u8>, Error> {
    let images = vec![input_image(image)?];

    let func = |images: Vec<Image>| Ok(images[0].flip_horizontal());

    make_png_or_gif(images, func)
}

pub fn flip_vertical(image: Vec<u8>) -> Result<Vec<u8>, Error> {
    let images = vec![input_image(image)?];

    let func = |images: Vec<Image>| Ok(images[0].flip_vertical());

    make_png_or_gif(images, func)
}

pub fn rotate(image: Vec<u8>, degrees: Option<f32>) -> Result<Vec<u8>, Error> {
    let images = vec![input_image(image)?];

    let degrees = degrees.unwrap_or(90.0);

    let func = |images: Vec<Image>| Ok(images[0].rotate(degrees));

    make_png_or_gif(images, func)
}

pub fn resize(image: Vec<u8>, width: Option<i32>, height: Option<i32>) -> Result<Vec<u8>, Error> {
    let images = vec![input_image(image)?];

    let func = |images: Vec<Image>| {
        if width.is_none() && height.is_none() {
            Ok(images[0].clone())
        } else if width.is_none() {
            Ok(images[0].resize_height(height.unwrap()))
        } else if height.is_none() {
            Ok(images[0].resize_width(width.unwrap()))
        } else {
            Ok(images[0].resize_exact((width.unwrap(), height.unwrap())))
        }
    };

    make_png_or_gif(images, func)
}

pub fn crop(
    image: Vec<u8>,
    left: Option<i32>,
    top: Option<i32>,
    right: Option<i32>,
    bottom: Option<i32>,
) -> Result<Vec<u8>, Error> {
    let images = vec![input_image(image)?];

    let img = &images[0].image;
    let img_w = img.width();
    let img_h = img.height();
    let left = left.unwrap_or(0).max(0).min(img_w);
    let top = top.unwrap_or(0).max(0).min(img_h);
    let right = right.unwrap_or(img_w).max(0).min(img_w);
    let bottom = bottom.unwrap_or(img_h).max(0).min(img_h);

    let func = |images: Vec<Image>| Ok(images[0].crop(IRect::from_ltrb(left, top, right, bottom)));

    make_png_or_gif(images, func)
}

pub fn grayscale(image: Vec<u8>) -> Result<Vec<u8>, Error> {
    let images = vec![input_image(image)?];

    let func = |images: Vec<Image>| Ok(images[0].grayscale());

    make_png_or_gif(images, func)
}

pub fn invert(image: Vec<u8>) -> Result<Vec<u8>, Error> {
    let images = vec![input_image(image)?];

    let func = |images: Vec<Image>| Ok(images[0].invert());

    make_png_or_gif(images, func)
}

pub fn merge_horizontal(images: Vec<Vec<u8>>) -> Result<Vec<u8>, Error> {
    let images = images
        .into_iter()
        .map(|image| input_image(image))
        .collect::<Result<Vec<_>, _>>()?;

    let func = |images: Vec<Image>| {
        let img_h = images.iter().map(|img| img.height()).min().unwrap();
        let images = images
            .into_iter()
            .map(|img| img.resize_height(img_h))
            .collect::<Vec<_>>();
        let img_w = images.iter().map(|img| img.width()).sum();
        let mut surface = new_surface((img_w, img_h));
        let canvas = surface.canvas();
        let mut x = 0;
        for img in images {
            canvas.draw_image(&img, (x, 0), None);
            x += img.width();
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

pub fn merge_vertical(images: Vec<Vec<u8>>) -> Result<Vec<u8>, Error> {
    let images = images
        .into_iter()
        .map(|image| input_image(image))
        .collect::<Result<Vec<_>, _>>()?;

    let func = |images: Vec<Image>| {
        let img_w = images.iter().map(|img| img.width()).min().unwrap();
        let images = images
            .into_iter()
            .map(|img| img.resize_width(img_w))
            .collect::<Vec<_>>();
        let img_h = images.iter().map(|img| img.height()).sum();
        let mut surface = new_surface((img_w, img_h));
        let canvas = surface.canvas();
        let mut y = 0;
        for img in images {
            canvas.draw_image(&img, (0, y), None);
            y += img.height();
        }
        Ok(surface.image_snapshot())
    };

    make_png_or_gif(images, func)
}

pub fn gif_split(image: Vec<u8>) -> Result<Vec<Vec<u8>>, Error> {
    let mut codec = decode_image(image)?;
    let count = codec.get_frame_count();
    let mut frames = vec![];
    for i in 0..count {
        let frame = codec.get_frame(i)?;
        frames.push(frame);
    }
    frames
        .into_iter()
        .map(|frame| encode_png(frame))
        .collect::<Result<Vec<_>, _>>()
}

pub fn gif_merge(images: Vec<Vec<u8>>, duration: Option<f32>) -> Result<Vec<u8>, Error> {
    let codecs = images
        .into_iter()
        .map(|image| decode_image(image))
        .collect::<Result<Vec<_>, _>>()?;
    let mut frames = vec![];
    for mut codec in codecs {
        let count = codec.get_frame_count();
        for i in 0..count {
            let frame = codec.get_frame(i)?;
            frames.push(frame);
        }
    }
    let min_w = frames.iter().map(|img| img.width()).min().unwrap();
    let min_h = frames.iter().map(|img| img.height()).min().unwrap();

    let duration = duration.unwrap_or(0.1);
    let mut encoder = GifEncoder::new();
    for frame in frames {
        encoder.add_frame(frame.resize_fit((min_w, min_h), Fit::Contain), duration)?;
    }
    Ok(encoder.finish())
}

pub fn gif_reverse(image: Vec<u8>) -> Result<Vec<u8>, Error> {
    let mut codec = decode_image(image)?;
    let count = codec.get_frame_count();
    let mut frames = vec![];
    for i in 0..count {
        let frame = codec.get_frame(i)?;
        frames.push(frame);
    }
    let duration = codec.get_average_duration()?;

    let mut encoder = GifEncoder::new();
    for i in 0..count {
        encoder.add_frame(frames[count - i - 1].clone(), duration)?;
    }
    Ok(encoder.finish())
}

pub fn gif_change_duration(image: Vec<u8>, duration: f32) -> Result<Vec<u8>, Error> {
    let mut codec = decode_image(image)?;
    let count = codec.get_frame_count();
    let mut encoder = GifEncoder::new();
    for i in 0..count {
        let frame = codec.get_frame(i)?;
        encoder.add_frame(frame, duration)?;
    }
    Ok(encoder.finish())
}
