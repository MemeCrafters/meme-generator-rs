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
    pub disabled: bool,
    #[napi(setter)]
    pub hot: bool,
    #[napi(setter)]
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

#[napi]
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

#[napi]
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
