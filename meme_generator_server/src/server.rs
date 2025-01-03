use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::LazyLock,
};

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
use serde_json::{json, Value};
use tokio::{net::TcpListener, runtime::Runtime, task::spawn_blocking};

use meme_generator::{
    error::Error,
    load_memes,
    meme::{self, Meme, OptionValue},
    VERSION,
};

use crate::config::CONFIG;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    options: HashMap<String, OptionValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    data: Value,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

static LOADED_MEMES: LazyLock<HashMap<String, Box<dyn Meme>>> = LazyLock::new(|| load_memes());

async fn meme_keys() -> Json<Vec<String>> {
    Json(LOADED_MEMES.keys().cloned().collect::<Vec<_>>())
}

async fn meme_info(Path(key): Path<String>) -> impl IntoResponse {
    if let Some(meme) = LOADED_MEMES.get(&key) {
        Json(meme.info()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Meme not found").into_response()
    }
}

async fn meme_preview(Path(key): Path<String>) -> impl IntoResponse {
    let meme = match LOADED_MEMES.get(&key) {
        Some(meme) => meme,
        None => return (StatusCode::NOT_FOUND, "Meme not found").into_response(),
    };

    match spawn_blocking(move || meme.generate_preview())
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

async fn meme_generate(
    Path(key): Path<String>,
    Json(payload): Json<MemeRequest>,
) -> impl IntoResponse {
    let meme = match LOADED_MEMES.get(&key) {
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

    let mut images: Vec<meme::Image> = Vec::new();
    for Image { name, id } in payload.images {
        if let Some(data) = id_to_data.get(&id) {
            images.push(meme::Image {
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
    let options = payload.options;

    match spawn_blocking(move || meme.generate(images, texts, options))
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

fn handle_error(error: Error) -> ErrorResponse {
    let message = format!("{error}");
    match error {
        Error::ImageDecodeError(err) => ErrorResponse {
            code: 510,
            message,
            data: json!({ "error": err }),
        },
        Error::ImageEncodeError(err) => ErrorResponse {
            code: 520,
            message,
            data: json!({ "error": err }),
        },
        Error::IOError(err) => ErrorResponse {
            code: 530,
            message,
            data: json!({ "error": err }),
        },
        Error::DeserializeError(err) => ErrorResponse {
            code: 540,
            message,
            data: json!({ "error": err }),
        },
        Error::ImageNumberMismatch(min, max, actual) => ErrorResponse {
            code: 550,
            message,
            data: json!({ "min": min, "max": max, "actual": actual }),
        },
        Error::TextNumberMismatch(min, max, actual) => ErrorResponse {
            code: 551,
            message,
            data: json!({ "min": min, "max": max, "actual": actual }),
        },
        Error::TextOverLength(text) => ErrorResponse {
            code: 560,
            message,
            data: json!({ "text": text }),
        },
        Error::MemeFeedback(feedback) => ErrorResponse {
            code: 570,
            message,
            data: json!({ "feedback": feedback }),
        },
    }
}

pub async fn run_server(host: Option<IpAddr>, port: Option<u16>) {
    let app = Router::new()
        .route("/meme/version", get(|| async { VERSION }))
        .route("/meme/keys", get(meme_keys))
        .route("/memes/:key/info", get(meme_info))
        .route("/memes/:key/preview", get(meme_preview))
        .route("/memes/:key", post(meme_generate));

    let host = host.unwrap_or(CONFIG.server.host);
    let port = port.unwrap_or(CONFIG.server.port);
    let addr = SocketAddr::new(host, port);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

pub fn run_server_sync(host: Option<IpAddr>, port: Option<u16>) {
    Runtime::new().unwrap().block_on(run_server(host, port));
}
