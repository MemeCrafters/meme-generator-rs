use std::{fs, path::PathBuf, sync::LazyLock};

use directories::UserDirs;
use log::warn;
use serde::Deserialize;

pub fn meme_home() -> PathBuf {
    let user_dirs = UserDirs::new().unwrap();
    user_dirs.home_dir().join(".meme_generator")
}

pub static MEME_HOME: LazyLock<PathBuf> = LazyLock::new(meme_home);

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub meme: MemeConfig,
    pub resource: ResourceConfig,
    pub gif: GifConfig,
    pub font: FontConfig,
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
pub struct GifConfig {
    pub gif_max_frames: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FontConfig {
    pub default_font_families: Vec<String>,
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
            gif: GifConfig {
                gif_max_frames: 200,
            },
            font: FontConfig {
                default_font_families: vec![
                    "Arial",
                    "Tahoma",
                    "Helvetica Neue",
                    "Segoe UI",
                    "PingFang SC",
                    "Hiragino Sans GB",
                    "Microsoft YaHei",
                    "Source Han Sans SC",
                    "Noto Sans SC",
                    "Noto Sans CJK SC",
                    "WenQuanYi Micro Hei",
                    "Apple Color Emoji",
                    "Noto Color Emoji",
                    "Segoe UI Emoji",
                    "Segoe UI Symbol",
                ]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            },
            translate: TranslatorConfig {
                baidu_trans_appid: "".to_string(),
                baidu_trans_apikey: "".to_string(),
            },
        }
    }
}

fn load_config() -> Config {
    let config_path = MEME_HOME.join("config.toml");
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

pub static MEME_CONFIG: LazyLock<Config> = LazyLock::new(load_config);
