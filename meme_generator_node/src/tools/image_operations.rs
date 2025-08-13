use crate::{
    Error, ImageDecodeError,
    tools::{ImageResult, ImagesResult, handle_image_result, handle_images_result},
};
use meme_generator::{error, tools::image_operations};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

#[napi(object)]
#[derive(Clone)]
/// 图片信息
pub struct ImageInfo {
    /// 宽度
    pub width: i32,
    /// 高度
    pub height: i32,
    /// 是否为多帧图片
    pub is_multi_frame: bool,
    /// 帧数
    pub frame_count: Option<i32>,
    /// 平均时长
    pub average_duration: Option<f64>,
}

#[napi]
#[derive(Clone)]
/// 图片信息结果
pub enum ImageInfoResult {
    /// 成功
    Ok(Option<ImageInfo>),
    /// 错误信息
    Err(Error),
}

#[napi(js_name = "inspect")]
/// 图片信息
pub fn inspect(image: Buffer) -> ImageInfoResult {
    let result = image_operations::inspect(image.to_vec());
    match result {
        Ok(info) => ImageInfoResult::Ok(Some(ImageInfo {
            width: info.width,
            height: info.height,
            is_multi_frame: info.is_multi_frame,
            frame_count: info.frame_count,
            average_duration: info.average_duration.map(|a| a as f64),
        })),
        Err(error) => match error {
            error::Error::ImageDecodeError(error) => {
                ImageInfoResult::Err(Error::ImageDecodeError(ImageDecodeError { error }))
            }
            _ => unreachable!(),
        },
    }
}

#[napi(js_name = "flip_horizontal")]
/// 水平翻转图片
pub fn flip_horizontal(image: Buffer) -> ImageResult {
    let result = image_operations::flip_horizontal(image.to_vec());
    handle_image_result(result)
}

#[napi(js_name = "flip_vertical")]
/// 垂直翻转图片
pub fn flip_vertical(image: Buffer) -> ImageResult {
    let result = image_operations::flip_vertical(image.to_vec());
    handle_image_result(result)
}

#[napi(js_name = "rotate")]
/// 旋转图片
pub fn rotate(image: Buffer, degrees: Option<f64>) -> ImageResult {
    let degrees = Some(degrees.unwrap_or(90.0));
    let result = image_operations::rotate(image.to_vec(), degrees.map(|d| d as f32));
    handle_image_result(result)
}

#[napi(js_name = "resize")]
/// 调整图片大小
pub fn resize(image: Buffer, width: Option<i32>, height: Option<i32>) -> ImageResult {
    let result = image_operations::resize(image.to_vec(), width, height);
    handle_image_result(result)
}

#[napi(js_name = "crop")]
/// 裁剪图片
pub fn crop(
    image: Buffer,
    left: Option<i32>,
    top: Option<i32>,
    right: Option<i32>,
    bottom: Option<i32>,
) -> ImageResult {
    let result = image_operations::crop(image.to_vec(), left, top, right, bottom);
    handle_image_result(result)
}

#[napi(js_name = "grayscale")]
/// 灰度化图片
pub fn grayscale(image: Buffer) -> ImageResult {
    let result = image_operations::grayscale(image.to_vec());
    handle_image_result(result)
}

#[napi(js_name = "invert")]
/// 反色图片
pub fn invert(image: Buffer) -> ImageResult {
    let result = image_operations::invert(image.to_vec());
    handle_image_result(result)
}

#[napi(js_name = "merge_horizontal")]
/// 水平合并图片
pub fn merge_horizontal(images: Vec<Buffer>) -> ImageResult {
    let result =
        image_operations::merge_horizontal(images.into_iter().map(|i| i.to_vec()).collect());
    handle_image_result(result)
}

#[napi(js_name = "merge_vertical")]
/// 垂直合并图片
pub fn merge_vertical(images: Vec<Buffer>) -> ImageResult {
    let result = image_operations::merge_vertical(images.into_iter().map(|i| i.to_vec()).collect());
    handle_image_result(result)
}

#[napi(js_name = "gif_split")]
/// 拆分gif图片
pub fn gif_split(image: Buffer) -> ImagesResult {
    let result = image_operations::gif_split(image.to_vec());
    handle_images_result(result)
}

#[napi(js_name = "gif_merge")]
/// 合并gif图片
pub fn gif_merge(images: Vec<Buffer>, duration: Option<f64>) -> ImageResult {
    let result = image_operations::gif_merge(
        images.into_iter().map(|i| i.to_vec()).collect(),
        duration.map(|d| d as f32),
    );
    handle_image_result(result)
}

#[napi(js_name = "gif_reverse")]
/// 反转gif图片
pub fn gif_reverse(image: Buffer) -> ImageResult {
    let result = image_operations::gif_reverse(image.to_vec());
    handle_image_result(result)
}

#[napi(js_name = "gif_change_duration")]
/// 改变gif图片时长, gif变速
pub fn gif_change_duration(image: Buffer, duration: f64) -> ImageResult {
    let result = image_operations::gif_change_duration(image.to_vec(), duration as f32);
    handle_image_result(result)
}
