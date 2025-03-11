use axum::{
    extract::Json,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;

use meme_generator::{error::Error, tools::image_operations};

use crate::server::{
    create_temp_file, get_temp_file, handle_error, handle_image_result, handle_server_error,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImageRequest {
    image_id: String,
}

pub(crate) async fn inspect(Json(payload): Json<ImageRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    match spawn_blocking(move || image_operations::inspect(data))
        .await
        .unwrap()
    {
        Ok(result) => Json(result).into_response(),
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn flip_horizontal(Json(payload): Json<ImageRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || image_operations::flip_horizontal(data))
        .await
        .unwrap();
    handle_image_result(result).await
}

pub(crate) async fn flip_vertical(Json(payload): Json<ImageRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || image_operations::flip_vertical(data))
        .await
        .unwrap();
    handle_image_result(result).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct RotateRequest {
    image_id: String,
    degrees: Option<f32>,
}

pub(crate) async fn rotate(Json(payload): Json<RotateRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || image_operations::rotate(data, payload.degrees))
        .await
        .unwrap();
    handle_image_result(result).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ResizeRequest {
    image_id: String,
    width: Option<i32>,
    height: Option<i32>,
}

pub(crate) async fn resize(Json(payload): Json<ResizeRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result =
        spawn_blocking(move || image_operations::resize(data, payload.width, payload.height))
            .await
            .unwrap();
    handle_image_result(result).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CropRequest {
    image_id: String,
    left: Option<i32>,
    top: Option<i32>,
    right: Option<i32>,
    bottom: Option<i32>,
}

pub(crate) async fn crop(Json(payload): Json<CropRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || {
        image_operations::crop(
            data,
            payload.left,
            payload.top,
            payload.right,
            payload.bottom,
        )
    })
    .await
    .unwrap();
    handle_image_result(result).await
}

pub(crate) async fn grayscale(Json(payload): Json<ImageRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || image_operations::grayscale(data))
        .await
        .unwrap();
    handle_image_result(result).await
}

pub(crate) async fn invert(Json(payload): Json<ImageRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || image_operations::invert(data))
        .await
        .unwrap();
    handle_image_result(result).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImagesRequest {
    image_ids: Vec<String>,
}

pub(crate) async fn merge_horizontal(Json(payload): Json<ImagesRequest>) -> Response {
    let mut images = vec![];
    for image_id in payload.image_ids {
        match get_temp_file(&image_id).await {
            Ok(data) => images.push(data),
            Err(err) => return handle_server_error(err).into_response(),
        };
    }

    let result = spawn_blocking(move || image_operations::merge_horizontal(images))
        .await
        .unwrap();
    handle_image_result(result).await
}

pub(crate) async fn merge_vertical(Json(payload): Json<ImagesRequest>) -> Response {
    let mut images = vec![];
    for image_id in payload.image_ids {
        match get_temp_file(&image_id).await {
            Ok(data) => images.push(data),
            Err(err) => return handle_server_error(err).into_response(),
        };
    }

    let result = spawn_blocking(move || image_operations::merge_vertical(images))
        .await
        .unwrap();
    handle_image_result(result).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImagesResponse {
    image_ids: Vec<String>,
}

pub(crate) async fn handle_images_result(result: Result<Vec<Vec<u8>>, Error>) -> Response {
    let mut image_ids = vec![];
    match result {
        Ok(data) => {
            for d in data {
                match create_temp_file(d).await {
                    Ok(id) => image_ids.push(id),
                    Err(err) => return handle_server_error(err).into_response(),
                };
            }
        }
        Err(error) => return handle_error(error).into_response(),
    }
    let response = ImagesResponse { image_ids };
    Json(response).into_response()
}

pub(crate) async fn gif_split(Json(payload): Json<ImageRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || image_operations::gif_split(data))
        .await
        .unwrap();
    handle_images_result(result).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GifMergeRequest {
    image_ids: Vec<String>,
    duration: Option<f32>,
}

pub(crate) async fn gif_merge(Json(payload): Json<GifMergeRequest>) -> Response {
    let mut images = vec![];
    for image in payload.image_ids {
        match get_temp_file(&image).await {
            Ok(data) => images.push(data),
            Err(err) => return handle_server_error(err).into_response(),
        };
    }

    let result = spawn_blocking(move || image_operations::gif_merge(images, payload.duration))
        .await
        .unwrap();
    handle_image_result(result).await
}

pub(crate) async fn gif_reverse(Json(payload): Json<ImageRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result = spawn_blocking(move || image_operations::gif_reverse(data))
        .await
        .unwrap();
    handle_image_result(result).await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GifDurationRequest {
    image_id: String,
    duration: f32,
}

pub(crate) async fn gif_change_duration(Json(payload): Json<GifDurationRequest>) -> Response {
    let data = match get_temp_file(&payload.image_id).await {
        Ok(data) => data,
        Err(err) => return handle_server_error(err).into_response(),
    };

    let result =
        spawn_blocking(move || image_operations::gif_change_duration(data, payload.duration))
            .await
            .unwrap();
    handle_image_result(result).await
}
