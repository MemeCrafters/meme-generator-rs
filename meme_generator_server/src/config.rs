use std::{
    net::{IpAddr, Ipv4Addr},
    sync::LazyLock,
};

use serde::Deserialize;
use tracing::warn;

use meme_generator::read_config_file;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: ServerConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
    /// Maximum request body size in bytes (default: 20MB)
    pub body_limit: usize,
    /// Maximum number of concurrent image generation tasks (default: 16)
    pub max_concurrent_tasks: usize,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: Ipv4Addr::new(0, 0, 0, 0).into(),
            port: 2233,
            body_limit: 20 * 1024 * 1024,
            max_concurrent_tasks: 16,
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
