use std::{fs, sync::RwLock};

use lazy_static::lazy_static;
use log::warn;
use serde::Deserialize;

use crate::utils::meme_home;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub meme: MemeConfig,
    pub resource: ResourceConfig,
    pub translate: TranslatorConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemeConfig {
    pub meme_disabled_list: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceConfig {
    pub resource_url: String,
}

#[derive(Debug, Clone, Deserialize)]
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
