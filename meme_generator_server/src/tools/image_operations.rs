use std::io::{Cursor, Write};

use axum::{
    body::Body,
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use zip::write::SimpleFileOptions;

use meme_generator::tools::image_operations;

use crate::server::{ImageData, handle_error, handle_image_data, handle_image_result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImageRequest {
    image: ImageData,
}

pub(crate) async fn flip_horizontal(Json(payload): Json<ImageRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::flip_horizontal(data))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn flip_vertical(Json(payload): Json<ImageRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::flip_vertical(data))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RotateRequest {
    image: ImageData,
    degrees: Option<f32>,
}

pub(crate) async fn rotate(Json(payload): Json<RotateRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::rotate(data, payload.degrees))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ResizeRequest {
    image: ImageData,
    width: Option<i32>,
    height: Option<i32>,
}

pub(crate) async fn resize(Json(payload): Json<ResizeRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::resize(data, payload.width, payload.height))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CropRequest {
    image: ImageData,
    left: Option<i32>,
    top: Option<i32>,
    right: Option<i32>,
    bottom: Option<i32>,
}

pub(crate) async fn crop(Json(payload): Json<CropRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || {
        image_operations::crop(
            data,
            payload.left,
            payload.top,
            payload.right,
            payload.bottom,
        )
    })
    .await
    .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn grayscale(Json(payload): Json<ImageRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::grayscale(data))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn invert(Json(payload): Json<ImageRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::invert(data))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImagesRequest {
    images: Vec<ImageData>,
}

pub(crate) async fn merge_horizontal(Json(payload): Json<ImagesRequest>) -> Response {
    let mut images = vec![];
    for image in payload.images {
        match handle_image_data(image).await {
            Ok(data) => images.push(data),
            Err(err) => return err.into_response(),
        };
    }

    match spawn_blocking(move || image_operations::merge_horizontal(images))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn merge_vertical(Json(payload): Json<ImagesRequest>) -> Response {
    let mut images = vec![];
    for image in payload.images {
        match handle_image_data(image).await {
            Ok(data) => images.push(data),
            Err(err) => return err.into_response(),
        };
    }

    match spawn_blocking(move || image_operations::merge_vertical(images))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) fn handle_images_result(images: Vec<Vec<u8>>) -> Response {
    let mut data = Vec::new();
    let cursor = Cursor::new(&mut data);
    let mut zip = zip::ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);
    for (i, image) in images.iter().enumerate() {
        zip.start_file(format!("{i}.png"), options).unwrap();
        zip.write_all(image).unwrap();
    }
    zip.finish().unwrap();

    let kind = infer::get(&data).unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", kind.mime_type())
        .body(Body::from(data))
        .unwrap()
}

pub(crate) async fn gif_split(Json(payload): Json<ImageRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::gif_split(data))
        .await
        .unwrap()
    {
        Ok(result) => handle_images_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GifMergeRequest {
    images: Vec<ImageData>,
    duration: Option<f32>,
}

pub(crate) async fn gif_merge(Json(payload): Json<GifMergeRequest>) -> Response {
    let mut images = vec![];
    for image in payload.images {
        match handle_image_data(image).await {
            Ok(data) => images.push(data),
            Err(err) => return err.into_response(),
        };
    }

    match spawn_blocking(move || image_operations::gif_merge(images, payload.duration))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn gif_reverse(Json(payload): Json<ImageRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::gif_reverse(data))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GifDurationRequest {
    image: ImageData,
    duration: f32,
}

pub(crate) async fn gif_change_duration(Json(payload): Json<GifDurationRequest>) -> Response {
    let data = match handle_image_data(payload.image).await {
        Ok(data) => data,
        Err(err) => return err.into_response(),
    };

    match spawn_blocking(move || image_operations::gif_change_duration(data, payload.duration))
        .await
        .unwrap()
    {
        Ok(result) => handle_image_result(result),
        Err(error) => handle_error(error).into_response(),
    }
}
