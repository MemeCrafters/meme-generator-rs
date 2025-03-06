use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use axum::{
    Router,
    body::Body,
    extract::{Json, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use base64_serde::base64_serde_type;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{net::TcpListener, runtime::Runtime, task::spawn_blocking};
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info};

use meme_generator::{
    VERSION,
    error::Error,
    get_meme, get_meme_keys, get_memes,
    meme::{self, OptionValue},
    search_memes,
    tools::{
        RenderMemeListParams, RenderMemeStatisticsParams, render_meme_list, render_meme_statistics,
    },
};

use crate::config::CONFIG;

base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Image {
    name: String,
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ImageData {
    Url {
        url: String,
        headers: Option<HashMap<String, String>>,
    },
    Path {
        path: PathBuf,
    },
    Data {
        #[serde(with = "Base64Standard")]
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemeRequest {
    images: Vec<Image>,
    image_data: HashMap<String, ImageData>,
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

async fn meme_keys() -> Json<Vec<&'static str>> {
    Json(get_meme_keys())
}

async fn meme_info(Path(key): Path<String>) -> impl IntoResponse {
    if let Some(meme) = get_meme(&key) {
        Json(meme.info()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Meme not found").into_response()
    }
}

async fn meme_infos() -> impl IntoResponse {
    let infos = get_memes()
        .iter()
        .map(|meme| meme.info())
        .collect::<Vec<_>>();
    Json(infos).into_response()
}

#[derive(Deserialize)]
struct SearchQuery {
    query: String,
    include_tags: Option<bool>,
}

async fn meme_search(Query(query): Query<SearchQuery>) -> Json<Vec<String>> {
    let keys = search_memes(&query.query, query.include_tags.unwrap_or(false));
    Json(keys)
}

async fn meme_preview(Path(key): Path<String>) -> impl IntoResponse {
    let meme = match get_meme(&key) {
        Some(meme) => meme,
        None => return (StatusCode::NOT_FOUND, "Meme not found").into_response(),
    };

    match spawn_blocking(move || meme.generate_preview(HashMap::new()))
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
    let meme = match get_meme(&key) {
        Some(meme) => meme,
        None => return (StatusCode::NOT_FOUND, "Meme not found").into_response(),
    };

    let mut id_to_data: HashMap<String, Vec<u8>> = HashMap::new();
    let client = reqwest::Client::new();
    for (id, image_data) in payload.image_data {
        let data = match image_data {
            ImageData::Url { url, headers } => {
                let headers = headers.unwrap_or_default();
                let request = client.get(&url);
                let request = headers
                    .iter()
                    .fold(request, |request, (key, value)| request.header(key, value));
                match request.send().await {
                    Ok(resp) => match resp.bytes().await {
                        Ok(bytes) => bytes.to_vec(),
                        Err(err) => {
                            return (
                                StatusCode::BAD_REQUEST,
                                format!("Failed to read image data from URL {url}: {err}"),
                            )
                                .into_response();
                        }
                    },
                    Err(err) => {
                        return (
                            StatusCode::BAD_REQUEST,
                            format!("Failed to fetch image data from URL {url}: {err}"),
                        )
                            .into_response();
                    }
                }
            }
            ImageData::Path { path } => match std::fs::read(&path) {
                Ok(data) => data,
                Err(err) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        format!("Failed to read image data from path {path:?}: {err}"),
                    )
                        .into_response();
                }
            },
            ImageData::Data { data } => data,
        };
        id_to_data.insert(id, data);
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

async fn render_list(Json(payload): Json<RenderMemeListParams>) -> impl IntoResponse {
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

async fn render_statistics(Json(payload): Json<RenderMemeStatisticsParams>) -> impl IntoResponse {
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
        Error::ImageAssetMissing(path) => ErrorResponse {
            code: 530,
            message,
            data: json!({ "path": path }),
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
        .route("/meme/infos", get(meme_infos))
        .route("/meme/search", get(meme_search))
        .route("/meme/tools/render_list", post(render_list))
        .route("/meme/tools/render_statistics", post(render_statistics))
        .route("/memes/:key/info", get(meme_info))
        .route("/memes/:key/preview", get(meme_preview))
        .route("/memes/:key", post(meme_generate))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let host = host.unwrap_or(CONFIG.server.host);
    let port = port.unwrap_or(CONFIG.server.port);
    let addr = SocketAddr::new(host, port);
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("Server running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

pub fn run_server_sync(host: Option<IpAddr>, port: Option<u16>) {
    Runtime::new().unwrap().block_on(run_server(host, port));
}
