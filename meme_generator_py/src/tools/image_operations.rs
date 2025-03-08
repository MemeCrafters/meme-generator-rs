use pyo3::prelude::*;

use meme_generator::{error, tools::image_operations};

use crate::{
    Error, ImageDecodeError,
    tools::{ImageResult, ImagesResult, handle_image_result, handle_images_result},
};

pub(crate) fn register_image_operations_module(
    parent_module: &Bound<'_, PyModule>,
) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "image_operations")?;
    m.add_class::<ImageInfo>()?;
    m.add_function(wrap_pyfunction!(inspect, &m)?)?;
    m.add_function(wrap_pyfunction!(flip_horizontal, &m)?)?;
    m.add_function(wrap_pyfunction!(flip_vertical, &m)?)?;
    m.add_function(wrap_pyfunction!(rotate, &m)?)?;
    m.add_function(wrap_pyfunction!(resize, &m)?)?;
    m.add_function(wrap_pyfunction!(crop, &m)?)?;
    m.add_function(wrap_pyfunction!(grayscale, &m)?)?;
    m.add_function(wrap_pyfunction!(invert, &m)?)?;
    m.add_function(wrap_pyfunction!(merge_horizontal, &m)?)?;
    m.add_function(wrap_pyfunction!(merge_vertical, &m)?)?;
    m.add_function(wrap_pyfunction!(gif_split, &m)?)?;
    m.add_function(wrap_pyfunction!(gif_merge, &m)?)?;
    m.add_function(wrap_pyfunction!(gif_reverse, &m)?)?;
    m.add_function(wrap_pyfunction!(gif_change_duration, &m)?)?;
    parent_module.add_submodule(&m)?;
    Python::with_gil(|py| {
        py.import("sys")?
            .getattr("modules")?
            .set_item("meme_generator.tools.image_operations", m)
    })?;
    Ok(())
}

#[pyclass]
#[derive(Clone)]
struct ImageInfo {
    #[pyo3(get)]
    width: i32,
    #[pyo3(get)]
    height: i32,
    #[pyo3(get)]
    is_multi_frame: bool,
    #[pyo3(get)]
    frame_count: Option<i32>,
    #[pyo3(get)]
    average_duration: Option<f32>,
}

#[derive(IntoPyObject, Clone)]
enum ImageInfoResult {
    Ok(ImageInfo),
    Err(Error),
}

#[pyfunction]
fn inspect(image: Vec<u8>) -> ImageInfoResult {
    let result = image_operations::inspect(image);
    match result {
        Ok(info) => ImageInfoResult::Ok(ImageInfo {
            width: info.width,
            height: info.height,
            is_multi_frame: info.is_multi_frame,
            frame_count: info.frame_count,
            average_duration: info.average_duration,
        }),
        Err(error) => match error {
            error::Error::ImageDecodeError(error) => {
                ImageInfoResult::Err(Error::ImageDecodeError(ImageDecodeError { error }))
            }
            _ => unreachable!(),
        },
    }
}

#[pyfunction]
fn flip_horizontal(image: Vec<u8>) -> ImageResult {
    let result = image_operations::flip_horizontal(image);
    handle_image_result(result)
}

#[pyfunction]
fn flip_vertical(image: Vec<u8>) -> ImageResult {
    let result = image_operations::flip_vertical(image);
    handle_image_result(result)
}

#[pyfunction]
#[pyo3(signature = (image, degrees=90.0))]
fn rotate(image: Vec<u8>, degrees: Option<f32>) -> ImageResult {
    let result = image_operations::rotate(image, degrees);
    handle_image_result(result)
}

#[pyfunction]
#[pyo3(signature = (image, width=None, height=None))]
fn resize(image: Vec<u8>, width: Option<i32>, height: Option<i32>) -> ImageResult {
    let result = image_operations::resize(image, width, height);
    handle_image_result(result)
}

#[pyfunction]
#[pyo3(signature = (image, left=None, top=None, right=None, bottom=None))]
fn crop(
    image: Vec<u8>,
    left: Option<i32>,
    top: Option<i32>,
    right: Option<i32>,
    bottom: Option<i32>,
) -> ImageResult {
    let result = image_operations::crop(image, left, top, right, bottom);
    handle_image_result(result)
}

#[pyfunction]
fn grayscale(image: Vec<u8>) -> ImageResult {
    let result = image_operations::grayscale(image);
    handle_image_result(result)
}

#[pyfunction]
fn invert(image: Vec<u8>) -> ImageResult {
    let result = image_operations::invert(image);
    handle_image_result(result)
}

#[pyfunction]
fn merge_horizontal(images: Vec<Vec<u8>>) -> ImageResult {
    let result = image_operations::merge_horizontal(images);
    handle_image_result(result)
}

#[pyfunction]
fn merge_vertical(images: Vec<Vec<u8>>) -> ImageResult {
    let result = image_operations::merge_vertical(images);
    handle_image_result(result)
}

#[pyfunction]
fn gif_split(image: Vec<u8>) -> ImagesResult {
    let result = image_operations::gif_split(image);
    handle_images_result(result)
}

#[pyfunction]
#[pyo3(signature = (images, duration=0.1))]
fn gif_merge(images: Vec<Vec<u8>>, duration: Option<f32>) -> ImageResult {
    let result = image_operations::gif_merge(images, duration);
    handle_image_result(result)
}

#[pyfunction]
fn gif_reverse(image: Vec<u8>) -> ImageResult {
    let result = image_operations::gif_reverse(image);
    handle_image_result(result)
}

#[pyfunction]
fn gif_change_duration(image: Vec<u8>, duration: f32) -> ImageResult {
    let result = image_operations::gif_change_duration(image, duration);
    handle_image_result(result)
}
