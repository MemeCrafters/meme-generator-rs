use std::{fs, sync::RwLock};

use lazy_static::lazy_static;
use log::warn;
use serde::Deserialize;

use crate::utils::meme_home;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub meme: MemeConfig,
    pub resource: ResourceConfig,
    pub gif: GifConfig,
    pub translate: TranslatorConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemeConfig {
    pub meme_disabled_list: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResourceConfig {
    pub resource_url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GifConfig {
    pub gif_max_size: f64,
    pub gif_max_frames: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TranslatorConfig {
    pub baidu_trans_appid: String,
    pub baidu_trans_apikey: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            meme: MemeConfig {
                meme_disabled_list: vec![],
            },
            resource: ResourceConfig {
                resource_url:
                    "https://ghp.ci/https://raw.githubusercontent.com/MeetWq/meme-generator/"
                        .to_string(),
            },
            gif: GifConfig {
                gif_max_size: 20.0,
                gif_max_frames: 200,
            },
            translate: TranslatorConfig {
                baidu_trans_appid: "".to_string(),
                baidu_trans_apikey: "".to_string(),
            },
        }
    }
}

fn load_config() -> Config {
    let config_path = meme_home().join("config.toml");
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|_| {
                warn!("Failed to create config directory");
            });
            fs::write(&config_path, "").unwrap_or_else(|_| {
                warn!("Failed to create config file");
            });
        }
    }
    if config_path.exists() {
        let config_content = fs::read_to_string(config_path).unwrap_or_else(|_| {
            warn!("Failed to read config file, using default config");
            String::new()
        });
        toml::from_str(&config_content).unwrap_or_else(|_| {
            warn!("Failed to parse config file, using default config");
            Config::default()
        })
    } else {
        Config::default()
    }
}

lazy_static! {
    static ref CONFIG: RwLock<Config> = RwLock::new(load_config());
}

pub fn get_config() -> Config {
    CONFIG.read().unwrap().clone()
}
