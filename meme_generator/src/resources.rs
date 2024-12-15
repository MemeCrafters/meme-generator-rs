use std::{fs, path::Path};

use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    runtime::Runtime,
    task,
};

use crate::{
    config::{MEME_CONFIG, MEME_HOME},
    version::VERSION,
};

#[derive(Deserialize)]
struct FileWithHash {
    file: String,
    hash: String,
}

#[derive(Deserialize)]
struct Resources {
    fonts: Vec<FileWithHash>,
    images: Vec<FileWithHash>,
}

fn resource_url(base_url: &str, name: &str) -> String {
    format!("{base_url}v{VERSION}/resources/{name}")
}

pub async fn check_resources(base_url: &str) {
    let client = Client::new();
    let resources = match fetch_resource_list(&client, base_url).await {
        Some(resources) => resources,
        None => return,
    };

    if MEME_CONFIG.resource.download_fonts {
        download_resources(&client, base_url, "fonts", &resources.fonts).await;
    }
    download_resources(&client, base_url, "images", &resources.images).await;
}

pub fn check_resources_sync(base_url: &str) {
    Runtime::new().unwrap().block_on(check_resources(base_url));
}

pub fn check_resources_in_background(base_url: &str) {
    let base_url = base_url.to_string();
    std::thread::spawn(move || {
        Runtime::new().unwrap().block_on(check_resources(&base_url));
    });
}

async fn fetch_resource_list(client: &Client, base_url: &str) -> Option<Resources> {
    let url = resource_url(base_url, "resources.json");
    let resp = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Failed to download {url}: {e}");
            return None;
        }
    };
    match resp.json::<Resources>().await {
        Ok(resources) => Some(resources),
        Err(e) => {
            eprintln!("Failed to parse resources.json: {e}");
            None
        }
    }
}

async fn download_resources(
    client: &Client,
    base_url: &str,
    resource_type: &str,
    resources: &[FileWithHash],
) {
    let resources_dir = MEME_HOME.join("resources").join(resource_type);

    let mut to_download = vec![];
    for res in resources {
        let file_path = resources_dir.join(&res.file);
        if !file_path.exists() || !is_file_hash_equal(&file_path, &res.hash).await {
            to_download.push(res);
        }
    }
    let total_files = to_download.len();
    if total_files == 0 {
        return;
    }

    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .progress_chars("#>-"),
    );
    println!("Downloading {resource_type}");

    let mut tasks = vec![];
    for resource in to_download {
        let file_path = resources_dir.join(&resource.file);
        let client = client.clone();
        let pb = pb.clone();
        let file_url = resource_url(
            base_url,
            format!("{resource_type}/{}", resource.file).as_str(),
        );

        tasks.push(task::spawn(async move {
            download_file(&client, &file_url, &file_path).await;
            pb.inc(1);
        }));
    }

    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("Task failed: {e}");
        }
    }

    pb.finish();
}

async fn is_file_hash_equal(file_path: &Path, expected_hash: &str) -> bool {
    if !file_path.exists() {
        return false;
    }
    let mut file = match File::open(file_path).await {
        Ok(file) => file,
        Err(_) => return false,
    };
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 4096];
    loop {
        let n = match file.read(&mut buffer).await {
            Ok(n) => n,
            Err(_) => return false,
        };
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let file_hash = format!("{:x}", hasher.finalize());
    file_hash == expected_hash
}

async fn download_file(client: &Client, url: &str, file_path: &Path) {
    if let Some(parent) = file_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create directory {}: {e}", parent.display());
            return;
        }
    }

    let mut resp = match client.get(url).send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("Failed to download {}: HTTP error {}", url, resp.status());
                return;
            }
            resp
        }
        Err(e) => {
            eprintln!("Failed to download {}: {e}", url);
            return;
        }
    };

    let mut file = match File::create(file_path).await {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file {}: {e}", file_path.display());
            return;
        }
    };

    while let Some(chunk) = match resp.chunk().await {
        Ok(chunk) => chunk,
        Err(e) => {
            eprintln!("Failed to download chunk from {}: {e}", url);
            return;
        }
    } {
        if let Err(e) = file.write_all(&chunk).await {
            eprintln!("Failed to write file {}: {e}", file_path.display());
            return;
        }
    }
}
