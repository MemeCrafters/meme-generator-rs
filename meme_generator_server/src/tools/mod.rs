use axum::{extract::Json, response::IntoResponse};
use tokio::task::spawn_blocking;

use meme_generator::tools::{
    RenderMemeListParams, RenderMemeStatisticsParams, render_meme_list, render_meme_statistics,
};

use crate::server::handle_image_result;

pub(crate) mod image_operations;

pub(crate) async fn render_list(Json(payload): Json<RenderMemeListParams>) -> impl IntoResponse {
    let payload = payload.clone();
    let result = spawn_blocking(move || render_meme_list(payload))
        .await
        .unwrap();
    handle_image_result(result).await
}

pub(crate) async fn render_statistics(
    Json(payload): Json<RenderMemeStatisticsParams>,
) -> impl IntoResponse {
    let payload = payload.clone();
    let result = spawn_blocking(move || render_meme_statistics(payload))
        .await
        .unwrap();
    handle_image_result(result).await
}
