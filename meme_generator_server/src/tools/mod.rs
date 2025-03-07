use axum::{
    body::Body,
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tokio::task::spawn_blocking;

use meme_generator::tools::{
    RenderMemeListParams, RenderMemeStatisticsParams, render_meme_list, render_meme_statistics,
};

use crate::server::handle_error;

pub(crate) mod image_operations;

pub(crate) async fn render_list(Json(payload): Json<RenderMemeListParams>) -> impl IntoResponse {
    let payload = payload.clone();
    match spawn_blocking(move || render_meme_list(payload))
        .await
        .unwrap()
    {
        Ok(result) => {
            let kind = infer::get(&result).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", kind.mime_type())
                .body(Body::from(result))
                .unwrap()
        }
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) async fn render_statistics(
    Json(payload): Json<RenderMemeStatisticsParams>,
) -> impl IntoResponse {
    let payload = payload.clone();
    match spawn_blocking(move || render_meme_statistics(payload))
        .await
        .unwrap()
    {
        Ok(result) => {
            let kind = infer::get(&result).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", kind.mime_type())
                .body(Body::from(result))
                .unwrap()
        }
        Err(error) => handle_error(error).into_response(),
    }
}
