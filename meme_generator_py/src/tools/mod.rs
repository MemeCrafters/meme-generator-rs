use std::collections::HashMap;

use pyo3::prelude::*;

use meme_generator::{error, tools};

use crate::{Error, ImageDecodeError, ImageEncodeError};

mod image_operations;

use image_operations::register_image_operations_module;

pub(crate) fn register_tools_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "tools")?;
    m.add_class::<MemeProperties>()?;
    m.add_class::<MemeSortBy>()?;
    m.add_class::<MemeStatisticsType>()?;
    m.add_function(wrap_pyfunction!(render_meme_list, &m)?)?;
    m.add_function(wrap_pyfunction!(render_meme_statistics, &m)?)?;
    register_image_operations_module(&m)?;
    parent_module.add_submodule(&m)?;
    parent_module
        .py()
        .import("sys")?
        .getattr("modules")?
        .set_item("meme_generator.tools", m)?;
    Ok(())
}

#[derive(IntoPyObject, Clone)]
enum ImageResult {
    Ok(Vec<u8>),
    Err(Error),
}

fn handle_image_result(result: Result<Vec<u8>, error::Error>) -> ImageResult {
    match result {
        Ok(data) => ImageResult::Ok(data),
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

#[derive(IntoPyObject, Clone)]
enum ImagesResult {
    Ok(Vec<Vec<u8>>),
    Err(Error),
}

fn handle_images_result(result: Result<Vec<Vec<u8>>, error::Error>) -> ImagesResult {
    match result {
        Ok(data) => ImagesResult::Ok(data),
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

#[pyclass(from_py_object)]
#[derive(Clone)]
struct MemeProperties {
    #[pyo3(set)]
    disabled: bool,
    #[pyo3(set)]
    hot: bool,
    #[pyo3(set)]
    new: bool,
}

#[pymethods]
impl MemeProperties {
    #[new]
    #[pyo3(signature = (disabled=false, hot=false, new=false))]
    fn new(disabled: bool, hot: bool, new: bool) -> Self {
        Self { disabled, hot, new }
    }
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

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Clone, PartialEq)]
enum MemeSortBy {
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

#[pyfunction]
#[pyo3(signature = (meme_properties=HashMap::new(), exclude_memes=Vec::new(), sort_by=MemeSortBy::KeywordsPinyin, sort_reverse=false, text_template="{index}. {keywords}".to_string(), add_category_icon=true))]
fn render_meme_list(
    meme_properties: HashMap<String, MemeProperties>,
    exclude_memes: Vec<String>,
    sort_by: MemeSortBy,
    sort_reverse: bool,
    text_template: String,
    add_category_icon: bool,
) -> ImageResult {
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

#[pyclass(eq, eq_int, from_py_object)]
#[derive(Clone, PartialEq)]
enum MemeStatisticsType {
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

#[pyfunction]
fn render_meme_statistics(
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
