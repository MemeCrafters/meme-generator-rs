use std::{path::PathBuf, sync::LazyLock};

use serde::Deserialize;
use tracing::warn;

use meme_generator_core::config::{MEME_HOME, read_config_file};

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub api: ApiConfig,
    pub encoder: EncoderConfig,
    pub font: FontConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api: ApiConfig::default(),
            encoder: EncoderConfig::default(),
            font: FontConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    pub baidu_trans_appid: Option<String>,
    pub baidu_trans_apikey: Option<String>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            baidu_trans_appid: None,
            baidu_trans_apikey: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct EncoderConfig {
    pub gif_max_frames: u16,
}

impl Default for EncoderConfig {
    fn default() -> Self {
        EncoderConfig {
            gif_max_frames: 200,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct FontConfig {
    pub use_local_fonts: bool,
    pub default_font_families: Vec<String>,
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            use_local_fonts: true,
            default_font_families: vec!["Noto Sans SC", "Noto Color Emoji"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

fn parse_config() -> Config {
    let config_content = read_config_file();
    if config_content.is_empty() {
        Config::default()
    } else {
        toml::from_str(&config_content).unwrap_or_else(|_| {
            warn!("Failed to parse config file, using default config");
            Config::default()
        })
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(parse_config);

pub static FONTS_DIR: LazyLock<PathBuf> = LazyLock::new(|| match option_env!("MEME_FONTS_DIR") {
    Some(dir) => PathBuf::from(dir),
    None => MEME_HOME.join("resources/fonts"),
});

pub static IMAGES_DIR: LazyLock<PathBuf> = LazyLock::new(|| match option_env!("MEME_IMAGES_DIR") {
    Some(dir) => PathBuf::from(dir),
    None => MEME_HOME.join("resources/images"),
});
