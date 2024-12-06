use std::collections::HashSet;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use skia_safe::Image;

use crate::error::Error;

pub use meme_options_derive::MemeOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserFlags {
    pub short: bool,
    pub long: bool,
    pub short_aliases: Option<Vec<char>>,
    pub long_aliases: Option<Vec<String>>,
}

impl Default for ParserFlags {
    fn default() -> Self {
        ParserFlags {
            short: false,
            long: false,
            short_aliases: None,
            long_aliases: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MemeOption {
    Boolean {
        name: String,
        default: Option<bool>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    String {
        name: String,
        default: Option<String>,
        choices: Option<Vec<String>>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    Integer {
        name: String,
        default: Option<i32>,
        minimum: Option<i32>,
        maximum: Option<i32>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    Float {
        name: String,
        default: Option<f64>,
        minimum: Option<f64>,
        maximum: Option<f64>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeParams {
    pub min_images: u8,
    pub max_images: u8,
    pub min_texts: u8,
    pub max_texts: u8,
    pub default_texts: Vec<String>,
    pub options: Vec<MemeOption>,
}

impl Default for MemeParams {
    fn default() -> Self {
        MemeParams {
            min_images: 0,
            max_images: 0,
            min_texts: 0,
            max_texts: 0,
            default_texts: Vec::new(),
            options: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeShortcut {
    pub pattern: String,
    pub args: Vec<String>,
    pub humanized: Option<String>,
}

impl Default for MemeShortcut {
    fn default() -> Self {
        MemeShortcut {
            pattern: String::new(),
            args: Vec::new(),
            humanized: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeInfo {
    pub key: String,
    pub params: MemeParams,
    pub keywords: Vec<String>,
    pub shortcuts: Vec<MemeShortcut>,
    pub tags: HashSet<String>,
    pub date_created: DateTime<Local>,
    pub date_modified: DateTime<Local>,
}

impl Default for MemeInfo {
    fn default() -> Self {
        MemeInfo {
            key: String::new(),
            params: MemeParams::default(),
            keywords: Vec::new(),
            shortcuts: Vec::new(),
            tags: HashSet::new(),
            date_created: Local::now(),
            date_modified: Local::now(),
        }
    }
}

pub trait IntoMemeOptions {
    fn into_options(&self) -> Vec<MemeOption>;
}

pub trait MemeGenerate {
    fn generate(
        &self,
        images: &Vec<Image>,
        texts: &Vec<String>,
        options: &impl IntoMemeOptions,
    ) -> Result<Vec<u8>, Error>;
}
