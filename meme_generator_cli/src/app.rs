use std::{collections::HashMap, net::SocketAddr};

use axum::{
    body::Body,
    extract::{Json, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::{net::TcpListener, task::spawn_blocking};

use meme_generator::{
    manager::{get_meme, get_meme_keys},
    meme::RawImage,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ImageData {
    id: String,
    base64_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Image {
    name: String,
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemeRequest {
    images: Vec<Image>,
    image_data: Vec<ImageData>,
    texts: Vec<String>,
    options: Map<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemeResponse {
    success: bool,
    message: String,
}

async fn meme_keys() -> Json<Vec<String>> {
    Json(get_meme_keys())
}

async fn meme_info(Path(key): Path<String>) -> impl IntoResponse {
    if let Some(meme) = get_meme(&key) {
        Json(meme.info()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Meme not found").into_response()
    }
}

async fn meme_generate(
    Path(key): Path<String>,
    Json(payload): Json<MemeRequest>,
) -> impl IntoResponse {
    let meme = match get_meme(&key) {
        Some(meme) => meme,
        None => return (StatusCode::NOT_FOUND, "Meme not found").into_response(),
    };

    let mut id_to_data: HashMap<String, Vec<u8>> = HashMap::new();
    for ImageData { id, base64_data } in payload.image_data {
        match general_purpose::STANDARD.decode(base64_data) {
            Ok(decoded_data) => {
                id_to_data.insert(id, decoded_data);
            }
            Err(err) => {
                return (
                    StatusCode::BAD_REQUEST,
                    format!("Invalid Base64 data for image id {id}: {err}"),
                )
                    .into_response();
            }
        }
    }

    let mut images: Vec<RawImage> = Vec::new();
    for Image { name, id } in payload.images {
        if let Some(data) = id_to_data.get(&id) {
            images.push(RawImage {
                name,
                data: data.clone(),
            });
        } else {
            return (
                StatusCode::BAD_REQUEST,
                format!("Image id {id} is referenced but no data provided"),
            )
                .into_response();
        }
    }
    let texts = payload.texts;
    let options = match serde_json::to_string(&payload.options) {
        Ok(options) => options,
        Err(err) => {
            return (StatusCode::BAD_REQUEST, format!("Invalid options: {err}")).into_response();
        }
    };

    let result = spawn_blocking(move || meme.generate(&images, &texts, options))
        .await
        .unwrap()
        .unwrap(); // TODO

    let kind = infer::get(&result).unwrap();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", kind.mime_type())
        .body(Body::from(result))
        .unwrap()
}

pub(crate) async fn run() {
    let app = Router::new()
        .route("/memes/keys", get(meme_keys))
        .route("/memes/:key/info", get(meme_info))
        .route("/memes/:key", post(meme_generate));

    let addr = SocketAddr::from(([0, 0, 0, 0], 2233));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
