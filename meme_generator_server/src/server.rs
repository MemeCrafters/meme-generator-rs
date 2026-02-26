use std::{
    collections::HashMap,
    error, fmt,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::LazyLock,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    Router,
    body::Body,
    extract::{DefaultBodyLimit, Json, Multipart, Path, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use base64_serde::base64_serde_type;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    runtime::Runtime,
    task::spawn_blocking,
    time::interval,
};
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info, warn};

use meme_generator::{
    MEME_HOME, MemeSortBy, VERSION,
    error::Error,
    get_meme, get_meme_keys_sorted, get_memes_sorted,
    meme::{self, OptionValue},
    search_memes,
};

use crate::{
    config::CONFIG,
    tools::{
        image_operations::{
            crop, flip_horizontal, flip_vertical, gif_change_duration, gif_merge, gif_reverse,
            gif_split, grayscale, inspect, invert, merge_horizontal, merge_vertical, resize,
            rotate,
        },
        render_list, render_statistics,
    },
};

base64_serde_type!(Base64Standard, base64::engine::general_purpose::STANDARD);

static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());

pub static TEMP_DIR: LazyLock<PathBuf> = LazyLock::new(|| MEME_HOME.join("tmp"));

fn clear_temp_dir() {
    let _ = std::fs::remove_dir_all(&*TEMP_DIR);
    let _ = std::fs::create_dir(&*TEMP_DIR);
}

async fn cleanup_temp_files() {
    let temp_dir = &*TEMP_DIR;
    if !temp_dir.exists() {
        return;
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let max_age_seconds = 10 * 60;
    let mut deleted_count = 0;

    if let Ok(entries) = std::fs::read_dir(temp_dir) {
        for entry in entries {
            if let Ok(entry) = entry
                && let Ok(metadata) = entry.metadata()
                && metadata.is_file()
                && let Ok(modified) = metadata.modified()
                && let Ok(modified_secs) = modified.duration_since(UNIX_EPOCH)
                && now - modified_secs.as_secs() > max_age_seconds
            {
                if let Err(e) = std::fs::remove_file(entry.path()) {
                    warn!("Failed to delete temp file {:?}: {}", entry.path(), e);
                } else {
                    deleted_count += 1;
                }
            }
        }
    }

    if deleted_count > 0 {
        info!("Cleaned up {} temporary files", deleted_count);
    }
}

#[derive(Debug)]
pub(crate) enum ServerError {
    RequestError(reqwest::Error),
    IOError(std::io::Error),
    MemeGeneratorError(Error),
}

impl From<reqwest::Error> for ServerError {
    fn from(err: reqwest::Error) -> Self {
        ServerError::RequestError(err)
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> Self {
        ServerError::IOError(err)
    }
}

impl From<Error> for ServerError {
    fn from(err: Error) -> Self {
        ServerError::MemeGeneratorError(err)
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::RequestError(err) => write!(f, "Request error: {err}"),
            ServerError::IOError(err) => write!(f, "IO error: {err}"),
            ServerError::MemeGeneratorError(err) => write!(f, "{err}"),
        }
    }
}

impl error::Error for ServerError {}

pub(crate) async fn create_temp_file(data: Vec<u8>) -> Result<String, ServerError> {
    let id = format!("{:x}", md5::compute(&data));
    let path = TEMP_DIR.join(&id);
    if path.exists() {
        return Ok(id);
    }
    File::create(&path).await?.write_all(&data).await?;
    Ok(id)
}

pub(crate) async fn get_temp_file(id: &str) -> Result<Vec<u8>, ServerError> {
    let path = TEMP_DIR.join(id);
    let mut file = File::open(&path).await?;
    let mut data = Vec::new();
    file.read_to_end(&mut data).await?;
    Ok(data)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum ImageData {
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
pub(crate) struct UploadImageResponse {
    image_id: String,
}

async fn download_url(
    url: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<Vec<u8>, ServerError> {
    let headers = headers.unwrap_or_default();
    let request = REQWEST_CLIENT.get(url);
    let request = headers
        .iter()
        .fold(request, |request, (key, value)| request.header(key, value));
    let response = request.send().await?;
    let data = response.bytes().await?;
    Ok(data.to_vec())
}

pub(crate) async fn upload_image(Json(image_data): Json<ImageData>) -> Response {
    let data = match image_data {
        ImageData::Url { url, headers } => match download_url(&url, headers).await {
            Ok(data) => data,
            Err(err) => {
                return handle_server_error(err).into_response();
            }
        },
        ImageData::Path { path } => match std::fs::read(&path) {
            Ok(data) => data,
            Err(err) => {
                return handle_server_error(err.into()).into_response();
            }
        },
        ImageData::Data { data } => data,
    };
    match create_temp_file(data).await {
        Ok(id) => {
            let response = UploadImageResponse { image_id: id };
            Json(response).into_response()
        }
        Err(err) => handle_server_error(err).into_response(),
    }
}

pub(crate) async fn upload_image_multipart(mut multipart: Multipart) -> Response {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap();
        if name == "file" {
            let data = field.bytes().await.unwrap().to_vec();
            match create_temp_file(data).await {
                Ok(id) => {
                    let response = UploadImageResponse { image_id: id };
                    return Json(response).into_response();
                }
                Err(err) => {
                    return handle_server_error(err).into_response();
                }
            }
        }
    }
    (StatusCode::BAD_REQUEST, "The field 'file' is required").into_response()
}

pub(crate) async fn get_image(Path(id): Path<String>) -> Response {
    match get_temp_file(&id).await {
        Ok(data) => {
            let kind = infer::get(&data).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", kind.mime_type())
                .body(Body::from(data))
                .unwrap()
        }
        Err(err) => handle_server_error(err).into_response(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Image {
    name: String,
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MemeRequest {
    images: Vec<Image>,
    texts: Vec<String>,
    options: HashMap<String, OptionValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImageResponse {
    image_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ErrorResponse {
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

#[derive(Deserialize)]
struct SortQuery {
    sort_by: Option<MemeSortBy>,
    sort_reverse: Option<bool>,
}

async fn meme_keys(Query(query): Query<SortQuery>) -> Response {
    let sort_by = query.sort_by.unwrap_or(MemeSortBy::Key);
    let sort_reverse = query.sort_reverse.unwrap_or(false);
    Json(get_meme_keys_sorted(sort_by, sort_reverse)).into_response()
}

async fn meme_info(Path(key): Path<String>) -> Response {
    if let Some(meme) = get_meme(&key) {
        Json(meme.info()).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Meme not found").into_response()
    }
}

async fn meme_infos(Query(query): Query<SortQuery>) -> Response {
    let sort_by = query.sort_by.unwrap_or(MemeSortBy::Key);
    let sort_reverse = query.sort_reverse.unwrap_or(false);
    let infos = get_memes_sorted(sort_by, sort_reverse)
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

async fn meme_search(Query(query): Query<SearchQuery>) -> Response {
    let keys = search_memes(&query.query, query.include_tags.unwrap_or(false));
    Json(keys).into_response()
}

async fn meme_preview(Path(key): Path<String>) -> Response {
    let meme = match get_meme(&key) {
        Some(meme) => meme,
        None => return (StatusCode::NOT_FOUND, "Meme not found").into_response(),
    };

    let result = spawn_blocking(move || meme.generate_preview(HashMap::new()))
        .await
        .unwrap();
    handle_image_result(result).await
}

async fn meme_generate(Path(key): Path<String>, Json(payload): Json<MemeRequest>) -> Response {
    let meme = match get_meme(&key) {
        Some(meme) => meme,
        None => return (StatusCode::NOT_FOUND, "Meme not found").into_response(),
    };

    let mut images: Vec<meme::Image> = Vec::new();
    for Image { name, id } in payload.images {
        match get_temp_file(&id).await {
            Ok(data) => images.push(meme::Image { name, data }),
            Err(err) => return handle_server_error(err).into_response(),
        }
    }
    let texts = payload.texts;
    let options = payload.options;

    let result = spawn_blocking(move || meme.generate(images, texts, options))
        .await
        .unwrap();
    handle_image_result(result).await
}

pub(crate) async fn handle_image_result(result: Result<Vec<u8>, Error>) -> Response {
    match result {
        Ok(data) => {
            let id = match create_temp_file(data).await {
                Ok(id) => id,
                Err(err) => return handle_server_error(err).into_response(),
            };
            let response = ImageResponse { image_id: id };
            Json(response).into_response()
        }
        Err(error) => handle_error(error).into_response(),
    }
}

pub(crate) fn handle_server_error(error: ServerError) -> ErrorResponse {
    let message = format!("{error}");
    match error {
        ServerError::RequestError(err) => ErrorResponse {
            code: 410,
            message,
            data: json!({ "error": format!("{err}") }),
        },
        ServerError::IOError(err) => ErrorResponse {
            code: 420,
            message,
            data: json!({ "error": format!("{err}") }),
        },
        ServerError::MemeGeneratorError(err) => handle_error(err),
    }
}

pub(crate) fn handle_error(error: Error) -> ErrorResponse {
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
    clear_temp_dir();

    let cleanup_task = {
        let mut interval = interval(Duration::from_secs(10 * 60));
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                cleanup_temp_files().await;
            }
        })
    };

    let app = Router::new()
        .route("/image/upload", post(upload_image))
        .layer(DefaultBodyLimit::disable())
        .route("/image/upload/multipart", post(upload_image_multipart))
        .layer(DefaultBodyLimit::disable())
        .route("/image/{id}", get(get_image))
        .route("/meme/version", get(|| async { VERSION }))
        .route("/meme/keys", get(meme_keys))
        .route("/meme/infos", get(meme_infos))
        .route("/meme/search", get(meme_search))
        .route("/memes/{key}/info", get(meme_info))
        .route("/memes/{key}/preview", get(meme_preview))
        .route("/memes/{key}", post(meme_generate))
        .route("/tools/render_list", post(render_list))
        .route("/tools/render_statistics", post(render_statistics))
        .route("/tools/image_operations/inspect", post(inspect))
        .route(
            "/tools/image_operations/flip_horizontal",
            post(flip_horizontal),
        )
        .route("/tools/image_operations/flip_vertical", post(flip_vertical))
        .route("/tools/image_operations/rotate", post(rotate))
        .route("/tools/image_operations/resize", post(resize))
        .route("/tools/image_operations/crop", post(crop))
        .route("/tools/image_operations/grayscale", post(grayscale))
        .route("/tools/image_operations/invert", post(invert))
        .route(
            "/tools/image_operations/merge_horizontal",
            post(merge_horizontal),
        )
        .route(
            "/tools/image_operations/merge_vertical",
            post(merge_vertical),
        )
        .route("/tools/image_operations/gif_split", post(gif_split))
        .route("/tools/image_operations/gif_merge", post(gif_merge))
        .route("/tools/image_operations/gif_reverse", post(gif_reverse))
        .route(
            "/tools/image_operations/gif_change_duration",
            post(gif_change_duration),
        )
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

    tokio::select! {
        _ = axum::serve(listener, app) => {
            info!("Server stopped");
        }
        _ = cleanup_task => {}
    }
}

pub fn run_server_sync(host: Option<IpAddr>, port: Option<u16>) {
    Runtime::new().unwrap().block_on(run_server(host, port));
}
