use std::sync::LazyLock;

use serde::Deserialize;
use tracing::warn;

use meme_generator_core::config::read_config_file;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub meme: MemeConfig,
    pub resource: ResourceConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            meme: MemeConfig::default(),
            resource: ResourceConfig::default(),
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct MemeConfig {
    pub load_builtin_memes: bool,
    pub load_external_memes: bool,
    pub meme_disabled_list: Vec<String>,
}

impl Default for MemeConfig {
    fn default() -> Self {
        MemeConfig {
            load_builtin_memes: true,
            load_external_memes: false,
            meme_disabled_list: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ResourceConfig {
    pub resource_url: String,
    pub download_fonts: bool,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        ResourceConfig {
            resource_url: "https://cdn.jsdelivr.net/gh/MemeCrafters/meme-generator-rs@".to_string(),
            download_fonts: true,
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
