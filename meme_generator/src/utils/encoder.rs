use gif::{DisposalMethod, Encoder, Frame, Repeat};
use skia_safe::{image::CachingHint, AlphaType, ColorType, EncodedImageFormat, Image, ImageInfo};

use crate::{
    config::MEME_CONFIG,
    error::{EncodeError, Error},
    meme::DecodedImage,
    utils::decoder::CodecExt,
};

pub(crate) fn encode_gif(images: &Vec<Image>, duration: f32) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    let delay = (duration * 100.0) as u16;
    {
        let mut encoder = Encoder::new(
            &mut bytes,
            images[0].width() as u16,
            images[0].height() as u16,
            &[],
        )?;
        encoder.set_repeat(Repeat::Infinite)?;
        for image in images {
            let image_info = ImageInfo::new(
                image.dimensions(),
                ColorType::RGBA8888,
                AlphaType::Unpremul,
                None,
            );
            let row_bytes = image_info.min_row_bytes();
            let data_size = image_info.compute_min_byte_size();
            let mut data = vec![0u8; data_size];
            image.read_pixels(
                &image_info,
                &mut data,
                row_bytes,
                (0, 0),
                CachingHint::Allow,
            );
            let mut frame =
                Frame::from_rgba_speed(image.width() as u16, image.height() as u16, &mut data, 10);
            frame.delay = delay;
            frame.dispose = DisposalMethod::Background;
            encoder.write_frame(&frame)?;
        }
    }
    Ok(bytes)
}

fn encode_image(
    image: &Image,
    format: EncodedImageFormat,
    quality: impl Into<Option<u32>>,
) -> Result<Vec<u8>, Error> {
    let data = image
        .encode(None, format, quality)
        .ok_or(EncodeError::SkiaEncodeError)?;
    Ok(data.as_bytes().to_vec())
}

pub(crate) fn encode_png(image: &Image) -> Result<Vec<u8>, Error> {
    encode_image(image, EncodedImageFormat::PNG, None)
}

/// gif 对齐方式
#[allow(dead_code)]
#[derive(PartialEq)]
pub(crate) enum FrameAlign {
    /// 以循环方式延长
    ExtendLoop,

    /// 延长第一帧
    ExtendFirst,

    /// 延长最后一帧
    ExtendLast,

    /// 不延长
    NoExtend,
}

#[derive(Debug, Clone)]
pub(crate) struct GifInfo {
    /// 帧数
    pub frame_num: u32,

    /// 帧间隔，单位为秒
    pub duration: f32,
}

impl GifInfo {
    pub fn total_duration(&self) -> f32 {
        self.frame_num as f32 * self.duration
    }
}

/// 将多个 gif 按照目标帧数和帧间隔对齐
///
/// - `gif_infos` 每个 gif 的帧数和帧间隔
/// - `target_gif_info` 目标 gif 的帧数和帧间隔
/// - `frame_align` gif 对齐方式
///
/// 返回值：每个 gif 的帧索引列表和目标 gif 的帧索引列表
pub(crate) fn get_aligned_gif_indexes(
    gif_infos: &Vec<GifInfo>,
    target_gif_info: &GifInfo,
    frame_align: impl Into<Option<FrameAlign>>,
) -> (Vec<Vec<usize>>, Vec<usize>) {
    let mut target_frame_indexes: Vec<usize> = (0..target_gif_info.frame_num as usize).collect();

    let max_total_duration = gif_infos
        .iter()
        .map(|gif_info| gif_info.total_duration())
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let target_total_duration = target_gif_info.total_duration();

    let diff_duration = max_total_duration - target_total_duration;
    if diff_duration >= target_gif_info.duration {
        let diff_num = (diff_duration / target_gif_info.duration).ceil() as i32;
        let frame_align = frame_align.into().unwrap_or(FrameAlign::ExtendLoop);
        match frame_align {
            FrameAlign::ExtendFirst => {
                let mut origin_frame_indexes = target_frame_indexes.clone();
                target_frame_indexes = vec![0; diff_num as usize];
                target_frame_indexes.append(&mut origin_frame_indexes);
            }
            FrameAlign::ExtendLast => {
                let mut append_frame_indexes =
                    vec![target_gif_info.frame_num as usize - 1; diff_num as usize];
                target_frame_indexes.append(&mut append_frame_indexes);
            }
            FrameAlign::ExtendLoop => {
                let mut total_frame_num = target_gif_info.frame_num;
                let max_frame_num = MEME_CONFIG.encoder.gif_max_frames;
                while total_frame_num + target_gif_info.frame_num <= max_frame_num as u32 {
                    total_frame_num += target_gif_info.frame_num;
                    let mut append_frame_indexes =
                        (0..target_gif_info.frame_num as usize).collect();
                    target_frame_indexes.append(&mut append_frame_indexes);
                    let total_duration = total_frame_num as f32 * target_gif_info.duration;
                    if gif_infos.iter().all(|gif_info| {
                        ((total_duration / gif_info.total_duration() as f32).round()
                            * gif_info.total_duration()
                            - total_duration)
                            .abs()
                            <= target_gif_info.duration
                    }) {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    let mut frame_indexes: Vec<Vec<usize>> = Vec::new();
    for gif_info in gif_infos {
        let mut frame_index = 0;
        let mut time_start = 0.0;
        let mut indexes: Vec<usize> = Vec::new();
        for i in 0..target_frame_indexes.len() {
            while frame_index < gif_info.frame_num {
                let duration = i as f32 * target_gif_info.duration - time_start;
                if duration >= frame_index as f32 * gif_info.duration
                    && duration < (frame_index + 1) as f32 * gif_info.duration
                {
                    indexes.push(frame_index as usize);
                    break;
                } else {
                    frame_index += 1;
                    if frame_index >= gif_info.frame_num {
                        frame_index = 0;
                        time_start += gif_info.total_duration();
                    }
                }
            }
        }
        frame_indexes.push(indexes);
    }

    (frame_indexes, target_frame_indexes)
}

/// 制作 png 或 gif
///
/// - `images` 图片列表
/// - `func`: 图片处理函数，传入图片列表，返回处理后的图片
///
pub(crate) fn make_png_or_gif<F>(images: &mut Vec<DecodedImage>, func: F) -> Result<Vec<u8>, Error>
where
    F: Fn(&Vec<Image>) -> Result<Image, Error>,
{
    let mut images = images
        .iter_mut()
        .map(|image| &mut image.codec)
        .collect::<Vec<_>>();

    let mut gif_flags: Vec<bool> = Vec::new();
    let mut gif_infos: Vec<GifInfo> = Vec::new();
    for image in images.iter_mut() {
        if image.is_multi_frame() {
            gif_flags.push(true);
            gif_infos.push(GifInfo {
                frame_num: image.get_frame_count() as u32,
                duration: image.get_average_duration()?,
            });
        } else {
            gif_flags.push(false);
        }
    }

    if gif_infos.len() == 0 {
        let images = images
            .iter_mut()
            .map(|image| image.first_frame())
            .collect::<Result<Vec<_>, Error>>()?;
        return Ok(encode_png(&func(&images)?)?);
    } else if gif_infos.len() == 1 {
        let mut frames: Vec<Image> = Vec::new();
        let gif_info = &gif_infos[0];
        for i in 0..gif_info.frame_num {
            let mut frame_images: Vec<Image> = Vec::new();
            for (j, image) in images.iter_mut().enumerate() {
                if gif_flags[j] {
                    frame_images.push(image.get_frame(i as usize)?);
                } else {
                    frame_images.push(image.first_frame()?);
                }
            }
            let frame = func(&frame_images)?;
            frames.push(frame);
        }
        return Ok(encode_gif(&frames, gif_info.duration)?);
    }

    let mut target_gif_index = 0;
    let mut target_duration = gif_infos[0].duration;
    for (i, gif_info) in gif_infos.iter().enumerate() {
        if gif_info.duration < target_duration {
            target_duration = gif_info.duration;
            target_gif_index = i;
        }
    }
    let target_gif_info = gif_infos[target_gif_index].clone();
    gif_infos.remove(target_gif_index);

    let (mut frame_indexes, target_frame_indexes) =
        get_aligned_gif_indexes(&gif_infos, &target_gif_info, FrameAlign::ExtendLoop);
    let target_frame_num = target_frame_indexes.len();
    frame_indexes.insert(target_gif_index, target_frame_indexes);

    let mut frames: Vec<Image> = Vec::new();
    for i in 0..target_frame_num {
        let mut frame_images: Vec<Image> = Vec::new();
        let mut gif_index = 0;
        for (j, image) in images.iter_mut().enumerate() {
            if gif_flags[j] {
                frame_images.push(image.get_frame(frame_indexes[gif_index][i] as usize)?);
                gif_index += 1;
            } else {
                frame_images.push(image.first_frame()?);
            }
        }
        let frame = func(&frame_images)?;
        frames.push(frame);
    }

    Ok(encode_gif(&frames, target_duration)?)
}

/// 使用静图或动图制作 gif
///
/// - `images` 图片列表
/// - `func` 图片处理函数生成，传入第几帧，返回对应的图片处理函数
/// - `target_gif_info` 目标 gif 的帧数和时间间隔
/// - `frame_align` gif 对齐方式
///
pub(crate) fn make_gif_or_combined_gif<F>(
    images: &mut Vec<DecodedImage>,
    func: F,
    target_gif_info: GifInfo,
    frame_align: impl Into<Option<FrameAlign>>,
) -> Result<Vec<u8>, Error>
where
    F: Fn(usize, &Vec<Image>) -> Result<Image, Error>,
{
    let mut images = images
        .iter_mut()
        .map(|image| &mut image.codec)
        .collect::<Vec<_>>();

    let mut gif_flags: Vec<bool> = Vec::new();
    let mut gif_infos: Vec<GifInfo> = Vec::new();
    for image in images.iter_mut() {
        if image.is_multi_frame() {
            gif_flags.push(true);
            gif_infos.push(GifInfo {
                frame_num: image.get_frame_count() as u32,
                duration: image.get_average_duration()?,
            });
        } else {
            gif_flags.push(false);
        }
    }

    if gif_infos.len() == 0 {
        let mut frames: Vec<Image> = Vec::new();
        let frame_images = images
            .iter_mut()
            .map(|image| image.first_frame())
            .collect::<Result<Vec<_>, Error>>()?;
        for i in 0..target_gif_info.frame_num {
            let frame = func(i as usize, &frame_images)?;
            frames.push(frame);
        }
        return Ok(encode_gif(&frames, target_gif_info.duration)?);
    }

    let (frame_indexes, target_frame_indexes) =
        get_aligned_gif_indexes(&gif_infos, &target_gif_info, frame_align);

    let mut frames: Vec<Image> = Vec::new();
    for (i, target_index) in target_frame_indexes.iter().enumerate() {
        let mut frame_images: Vec<Image> = Vec::new();
        let mut gif_index = 0;
        for (j, image) in images.iter_mut().enumerate() {
            if gif_flags[j] {
                frame_images.push(image.get_frame(frame_indexes[gif_index][i] as usize)?);
                gif_index += 1;
            } else {
                frame_images.push(image.first_frame()?);
            }
        }
        let frame = func(*target_index, &frame_images)?;
        frames.push(frame);
    }

    encode_gif(&frames, target_gif_info.duration)
}
