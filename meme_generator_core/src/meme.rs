use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserFlags {
    pub short: bool,
    pub long: bool,
    pub short_aliases: Vec<char>,
    pub long_aliases: Vec<String>,
}

impl Default for ParserFlags {
    fn default() -> Self {
        ParserFlags {
            short: false,
            long: false,
            short_aliases: Vec::new(),
            long_aliases: Vec::new(),
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
        default: Option<f32>,
        minimum: Option<f32>,
        maximum: Option<f32>,
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
    pub humanized: Option<String>,
    pub names: Vec<String>,
    pub texts: Vec<String>,
    pub options: HashMap<String, OptionValue>,
}

impl Default for MemeShortcut {
    fn default() -> Self {
        MemeShortcut {
            pattern: String::new(),
            humanized: None,
            names: Vec::new(),
            texts: Vec::new(),
            options: HashMap::new(),
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

pub struct Image {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionValue {
    Boolean(bool),
    String(String),
    Integer(i32),
    Float(f32),
}

impl Into<OptionValue> for bool {
    fn into(self) -> OptionValue {
        OptionValue::Boolean(self)
    }
}

impl Into<OptionValue> for String {
    fn into(self) -> OptionValue {
        OptionValue::String(self)
    }
}

impl Into<OptionValue> for &str {
    fn into(self) -> OptionValue {
        OptionValue::String(self.to_string())
    }
}

impl Into<OptionValue> for i32 {
    fn into(self) -> OptionValue {
        OptionValue::Integer(self)
    }
}

impl Into<OptionValue> for f32 {
    fn into(self) -> OptionValue {
        OptionValue::Float(self)
    }
}

pub trait Meme: Send + Sync {
    fn key(&self) -> String;
    fn info(&self) -> MemeInfo;
    fn generate(
        &self,
        images: Vec<Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> Result<Vec<u8>, Error>;
    fn generate_preview(&self) -> Result<Vec<u8>, Error>;
}
