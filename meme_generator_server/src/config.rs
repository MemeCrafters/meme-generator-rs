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
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: Ipv4Addr::new(0, 0, 0, 0).into(),
            port: 2233,
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
