use std::collections::HashMap;

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

use meme_generator::{error, tools};

use crate::{Error, ImageDecodeError, ImageEncodeError};

mod image_operations;

#[napi]
pub enum ImageResult {
    Ok(Buffer),
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
pub enum ImagesResult {
    Ok(Vec<Buffer>),
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
pub struct MemeProperties {
    #[napi(setter)]
    pub disabled: Option<bool>,
    #[napi(setter)]
    pub hot: Option<bool>,
    #[napi(setter)]
    pub new: Option<bool>,
}

impl Into<tools::MemeProperties> for MemeProperties {
    fn into(self) -> tools::MemeProperties {
        tools::MemeProperties {
            disabled: self.disabled.unwrap_or(false),
            hot: self.hot.unwrap_or(false),
            new: self.new.unwrap_or(false),
        }
    }
}

#[napi]
#[derive(Clone, PartialEq)]
pub enum MemeSortBy {
    Key = 0,
    Keywords = 1,
    KeywordsPinyin = 2,
    DateCreated = 3,
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

#[napi(object)]
#[derive(Clone)]
pub struct RenderMemeListParams {
    #[napi(setter)]
    pub meme_properties: Option<HashMap<String, MemeProperties>>,
    #[napi(setter)]
    pub exclude_memes: Option<Vec<String>>,
    #[napi(setter)]
    pub sort_by: Option<MemeSortBy>,
    #[napi(setter)]
    pub sort_reverse: Option<bool>,
    #[napi(setter)]
    pub text_template: Option<String>,
    #[napi(setter)]
    pub add_category_icon: Option<bool>,
}

#[napi]
pub fn render_meme_list(render_meme_list_params: RenderMemeListParams) -> ImageResult {
    let meme_properties = render_meme_list_params.meme_properties.unwrap_or_default();
    let exclude_memes = render_meme_list_params.exclude_memes.unwrap_or_default();
    let sort_by = render_meme_list_params
        .sort_by
        .unwrap_or(MemeSortBy::KeywordsPinyin);
    let sort_reverse = render_meme_list_params.sort_reverse.unwrap_or(false);
    let text_template = render_meme_list_params
        .text_template
        .unwrap_or_else(|| "{index}. {keywords}".to_string());
    let add_category_icon = render_meme_list_params.add_category_icon.unwrap_or(true);

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
pub enum MemeStatisticsType {
    MemeCount = 0,
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

#[napi(object)]
#[derive(Clone)]
pub struct RenderMemeStatisticsParams {
    #[napi(setter)]
    pub title: String,
    #[napi(setter)]
    pub statistics_type: MemeStatisticsType,
    #[napi(setter)]
    pub data: Vec<(String, i32)>,
}

#[napi]
pub fn render_meme_statistics(
    render_meme_statistics_params: RenderMemeStatisticsParams,
) -> ImageResult {
    let result = tools::render_meme_statistics(tools::RenderMemeStatisticsParams {
        title: render_meme_statistics_params.title,
        statistics_type: render_meme_statistics_params.statistics_type.into(),
        data: render_meme_statistics_params.data,
    });
    handle_image_result(result)
}
