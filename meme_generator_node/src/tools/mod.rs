mod image_operations;

use crate::{Error, ImageDecodeError, ImageEncodeError};
use meme_generator::{error, tools};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use std::collections::HashMap;
#[napi]
/// 图片结果
pub enum ImageResult {
    /// 成功
    Ok(Buffer),
    /// 错误信息
    Err(Error),
}

fn handle_image_result(result: Result<Vec<u8>, error::Error>) -> ImageResult {
    match result {
        Ok(data) => ImageResult::Ok(Buffer::from(data)),
        Err(error) => match error {
            error::Error::ImageDecodeError(error) => {
                ImageResult::Err(Error::ImageDecodeError(ImageDecodeError { error }))
            }
            error::Error::ImageEncodeError(error) => {
                ImageResult::Err(Error::ImageEncodeError(ImageEncodeError { error }))
            }
            _ => unreachable!(),
        },
    }
}

#[napi]
/// 图片结果
pub enum ImagesResult {
    /// 成功
    Ok(Vec<Buffer>),
    /// 错误信息
    Err(Error),
}

fn handle_images_result(result: Result<Vec<Vec<u8>>, error::Error>) -> ImagesResult {
    match result {
        Ok(data) => ImagesResult::Ok(data.into_iter().map(Buffer::from).collect()),
        Err(error) => match error {
            error::Error::ImageDecodeError(error) => {
                ImagesResult::Err(Error::ImageDecodeError(ImageDecodeError { error }))
            }
            error::Error::ImageEncodeError(error) => {
                ImagesResult::Err(Error::ImageEncodeError(ImageEncodeError { error }))
            }
            _ => unreachable!(),
        },
    }
}

#[napi(object)]
#[derive(Clone)]
/// 表情属性
pub struct MemeProperties {
    #[napi(setter)]
    /// 是否禁用
    pub disabled: bool,
    #[napi(setter)]
    /// 是否最热
    pub hot: bool,
    #[napi(setter)]
    /// 是否最新
    pub new: bool,
}

impl Into<tools::MemeProperties> for MemeProperties {
    fn into(self) -> tools::MemeProperties {
        tools::MemeProperties {
            disabled: self.disabled,
            hot: self.hot,
            new: self.new,
        }
    }
}

#[napi]
#[derive(Clone, PartialEq)]
pub enum MemeSortBy {
    /// 按键
    Key = 0,
    /// 关键词
    Keywords = 1,
    /// 关键词拼音
    KeywordsPinyin = 2,
    /// 创建时间
    DateCreated = 3,
    /// 修改时间
    DateModified = 4,
}
impl Into<tools::MemeSortBy> for MemeSortBy {
    fn into(self) -> tools::MemeSortBy {
        match self {
            MemeSortBy::Key => tools::MemeSortBy::Key,
            MemeSortBy::Keywords => tools::MemeSortBy::Keywords,
            MemeSortBy::KeywordsPinyin => tools::MemeSortBy::KeywordsPinyin,
            MemeSortBy::DateCreated => tools::MemeSortBy::DateCreated,
            MemeSortBy::DateModified => tools::MemeSortBy::DateModified,
        }
    }
}

#[napi(js_name = "render_meme_list")]
/// 渲染表情列表
pub fn render_meme_list(
    meme_properties: Option<HashMap<String, MemeProperties>>,
    exclude_memes: Option<Vec<String>>,
    sort_by: Option<MemeSortBy>,
    sort_reverse: Option<bool>,
    text_template: Option<String>,
    add_category_icon: Option<bool>,
) -> ImageResult {
    let meme_properties = meme_properties.unwrap_or_default();
    let exclude_memes = exclude_memes.unwrap_or_default();
    let sort_by = sort_by.unwrap_or(MemeSortBy::KeywordsPinyin);
    let sort_reverse = sort_reverse.unwrap_or(false);
    let text_template = text_template.unwrap_or_else(|| "{index}. {keywords}".to_string());
    let add_category_icon = add_category_icon.unwrap_or(true);

    let result = tools::render_meme_list(tools::RenderMemeListParams {
        meme_properties: meme_properties
            .into_iter()
            .map(|(key, value)| (key, value.into()))
            .collect(),
        exclude_memes,
        sort_by: sort_by.into(),
        sort_reverse,
        text_template,
        add_category_icon,
    });
    handle_image_result(result)
}

#[napi]
#[derive(Clone, PartialEq)]
/// 统计类型
pub enum MemeStatisticsType {
    /// 按表情数量统计
    MemeCount = 0,
    /// 按时间数量统计
    TimeCount = 1,
}

impl Into<tools::MemeStatisticsType> for MemeStatisticsType {
    fn into(self) -> tools::MemeStatisticsType {
        match self {
            MemeStatisticsType::MemeCount => tools::MemeStatisticsType::MemeCount,
            MemeStatisticsType::TimeCount => tools::MemeStatisticsType::TimeCount,
        }
    }
}

#[napi(js_name = "render_meme_statistics")]
/// 渲染表情统计
pub fn render_meme_statistics(
    title: String,
    statistics_type: MemeStatisticsType,
    data: Vec<(String, i32)>,
) -> ImageResult {
    let result = tools::render_meme_statistics(tools::RenderMemeStatisticsParams {
        title,
        statistics_type: statistics_type.into(),
        data,
    });
    handle_image_result(result)
}
