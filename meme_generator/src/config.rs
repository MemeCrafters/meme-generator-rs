use std::{
    env, fs,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::LazyLock,
};

use directories::UserDirs;
use serde::Deserialize;

pub fn meme_home() -> PathBuf {
    match env::var("MEME_HOME") {
        Ok(value) => PathBuf::from(value),
        Err(_) => {
            let user_dirs = UserDirs::new().unwrap();
            user_dirs.home_dir().join(".meme_generator")
        }
    }
}

pub static MEME_HOME: LazyLock<PathBuf> = LazyLock::new(meme_home);

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub meme: MemeConfig,
    pub resource: ResourceConfig,
    pub encoder: EncoderConfig,
    pub font: FontConfig,
    pub service: ServiceConfig,
    pub server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            meme: MemeConfig::default(),
            resource: ResourceConfig::default(),
            encoder: EncoderConfig::default(),
            font: FontConfig::default(),
            service: ServiceConfig::default(),
            server: ServerConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct MemeConfig {
    pub meme_disabled_list: Vec<String>,
}

impl Default for MemeConfig {
    fn default() -> Self {
        MemeConfig {
            meme_disabled_list: vec![],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResourceConfig {
    pub resource_url: String,
    pub download_fonts: bool,
}

impl Default for ResourceConfig {
    fn default() -> Self {
        ResourceConfig {
            resource_url:
                "https://ghp.ci/https://raw.githubusercontent.com/MemeCrafters/meme-generator-rs/"
                    .to_string(),
            download_fonts: true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
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
pub struct FontConfig {
    pub default_font_families: Vec<String>,
}

impl Default for FontConfig {
    fn default() -> Self {
        FontConfig {
            default_font_families: vec!["Noto Sans SC", "Noto Color Emoji"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceConfig {
    pub baidu_trans_appid: Option<String>,
    pub baidu_trans_apikey: Option<String>,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        ServiceConfig {
            baidu_trans_apikey: None,
            baidu_trans_appid: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: IpAddr,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host: Ipv4Addr::new(127, 0, 0, 1).into(),
            port: 2233,
        }
    }
}

fn load_config() -> Config {
    let config_path = MEME_HOME.join("config.toml");
    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).unwrap_or_else(|_| {
                eprintln!("Failed to create config directory");
            });
            fs::write(&config_path, "").unwrap_or_else(|_| {
                eprintln!("Failed to create config file");
            });
        }
    }
    if config_path.exists() {
        let config_content = fs::read_to_string(config_path).unwrap_or_else(|_| {
            eprintln!("Failed to read config file, using default config");
            String::new()
        });
        if config_content.is_empty() {
            Config::default()
        } else {
            toml::from_str(&config_content).unwrap_or_else(|_| {
                eprintln!("Failed to parse config file, using default config");
                Config::default()
            })
        }
    } else {
        Config::default()
    }
}

pub static MEME_CONFIG: LazyLock<Config> = LazyLock::new(load_config);
