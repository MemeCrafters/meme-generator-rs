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
use serde_json::{json, Map, Value};
use tokio::{net::TcpListener, task::spawn_blocking};

use meme_generator::{
    error::{EncodeError, Error},
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
struct ErrorResponse {
    err_code: u16,
    message: String,
    data: Option<Value>,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
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
    let options = payload.options;

    match spawn_blocking(move || meme.generate(&images, &texts, &options))
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
        Err(error) => {
            let message = format!("{error}");
            match error {
                Error::ImageDecodeError(Some(err)) => ErrorResponse {
                    err_code: 510,
                    message,
                    data: Some(json!({ "error": format!("{err:?}") })),
                }
                .into_response(),
                Error::ImageDecodeError(None) => ErrorResponse {
                    err_code: 510,
                    message,
                    data: None,
                }
                .into_response(),
                Error::ImageEncodeError(encode_err) => match encode_err {
                    EncodeError::GifEncodeError(err) => ErrorResponse {
                        err_code: 520,
                        message,
                        data: Some(json!({ "error": format!("{err:?}") })),
                    }
                    .into_response(),
                    EncodeError::SkiaEncodeError => ErrorResponse {
                        err_code: 521,
                        message,
                        data: None,
                    }
                    .into_response(),
                },
                Error::IOError(err) => ErrorResponse {
                    err_code: 530,
                    message,
                    data: Some(json!({ "error": format!("{err:?}") })),
                }
                .into_response(),
                Error::DeserializeError(err) => ErrorResponse {
                    err_code: 540,
                    message,
                    data: Some(json!({ "error": format!("{err:?}") })),
                }
                .into_response(),
                Error::ImageNumberMismatch(min, max, actual) => ErrorResponse {
                    err_code: 550,
                    message,
                    data: Some(json!({ "min": min, "max": max, "actual": actual })),
                }
                .into_response(),
                Error::TextNumberMismatch(min, max, actual) => ErrorResponse {
                    err_code: 551,
                    message,
                    data: Some(json!({ "min": min, "max": max, "actual": actual })),
                }
                .into_response(),
                Error::TextOverLength(text) => ErrorResponse {
                    err_code: 560,
                    message,
                    data: Some(json!({ "text": text })),
                }
                .into_response(),
                Error::MemeFeedback(feedback) => ErrorResponse {
                    err_code: 570,
                    message,
                    data: Some(json!({ "feedback": feedback })),
                }
                .into_response(),
            }
        }
    }
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
