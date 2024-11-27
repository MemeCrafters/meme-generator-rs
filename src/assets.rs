use std::{collections::HashMap, path::Path};

use indicatif::{ProgressBar, ProgressStyle};
use log::warn;
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    task,
};

use crate::{error::Error, utils::meme_home};

#[derive(Deserialize)]
struct Assets {
    assets: HashMap<String, String>,
}

pub async fn check_and_download_assets(base_url: &str) {
    let assets_dir = meme_home().join("assets");

    let client = Client::new();

    let url = format!("{}/assets.json", base_url);
    let resp = match client.get(&url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            warn!("Failed to download {}: {}", url, e);
            return;
        }
    };
    let assets: Assets = match resp.json().await {
        Ok(assets) => assets,
        Err(e) => {
            warn!("Failed to parse assets.json: {}", e);
            return;
        }
    };

    let total_files = assets.assets.len();
    let pb = ProgressBar::new(total_files as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .progress_chars("#>-"),
    );

    let mut tasks = vec![];
    for (file, hash) in assets.assets.into_iter() {
        let file_path = assets_dir.join(file.clone());
        let client = client.clone();
        let pb = pb.clone();
        let base_url = base_url.to_string();
        tasks.push(task::spawn(async move {
            if !file_path.exists() || !is_file_hash_equal(&file_path, &hash).await {
                download_file(
                    &client,
                    &format!("{}/{}", base_url, file.clone()),
                    &file_path,
                )
                .await;
            }
            pb.inc(1);
            Ok::<(), Error>(())
        }));
    }

    for task in tasks {
        match task.await {
            Ok(_) => {}
            Err(e) => {
                warn!("Failed to download file: {}", e);
            }
        }
    }
    pb.finish_with_message("Assets updated successfully.");
}

async fn is_file_hash_equal(file_path: &Path, expected_hash: &str) -> bool {
    if !file_path.exists() {
        return false;
    }
    let mut file = match File::open(file_path).await {
        Ok(file) => file,
        Err(e) => {
            warn!("Failed to open file {}: {}", file_path.display(), e);
            return false;
        }
    };
    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 1024];
    loop {
        let n = match file.read(&mut buffer).await {
            Ok(n) => n,
            Err(e) => {
                warn!("Failed to read file {}: {}", file_path.display(), e);
                return false;
            }
        };
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    let result = hasher.finalize();
    let file_hash = format!("{:x}", result);
    file_hash == expected_hash
}

async fn download_file(client: &Client, url: &str, file_path: &Path) {
    let mut resp = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            warn!("Failed to download {}: {}", url, e);
            return;
        }
    };
    let mut file = match File::create(file_path).await {
        Ok(file) => file,
        Err(e) => {
            warn!("Failed to create file {}: {}", file_path.display(), e);
            return;
        }
    };
    while let Some(chunk) = match resp.chunk().await {
        Ok(chunk) => chunk,
        Err(e) => {
            warn!("Failed to download {}: {}", url, e);
            return;
        }
    } {
        match file.write_all(&chunk).await {
            Ok(_) => {}
            Err(e) => {
                warn!("Failed to write file {}: {}", file_path.display(), e);
                return;
            }
        }
    }
}
