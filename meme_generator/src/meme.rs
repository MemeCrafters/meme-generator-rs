use std::collections::HashSet;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::{decoder::decode_image, error::Error};

pub use meme_options_derive::MemeOptions;

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

pub trait MemeOptions: Send {
    fn into_options(&self) -> Vec<MemeOption>;
}

pub struct InputImage {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct DecodedImage {
    pub name: String,
    pub image: skia_safe::Image,
}

impl InputImage {
    pub fn decode(&self) -> Result<DecodedImage, Error> {
        let image = decode_image(&self.data)?;
        Ok(DecodedImage {
            name: self.name.clone(),
            image,
        })
    }
}

#[derive(MemeOptions, Deserialize)]
pub struct NoOptions {}

type MemeFunction<T> = fn(&Vec<DecodedImage>, &Vec<String>, &T) -> Result<Vec<u8>, Error>;

pub struct MemeBuilder<T>
where
    T: MemeOptions + for<'de> Deserialize<'de> + Default + Sync,
{
    pub key: String,
    pub min_images: u8,
    pub max_images: u8,
    pub min_texts: u8,
    pub max_texts: u8,
    pub default_texts: Vec<String>,
    pub options: T,
    pub keywords: Vec<String>,
    pub shortcuts: Vec<MemeShortcut>,
    pub tags: HashSet<String>,
    pub date_created: DateTime<Local>,
    pub date_modified: DateTime<Local>,
    pub function: MemeFunction<T>,
}

impl<T> Default for MemeBuilder<T>
where
    T: MemeOptions + for<'de> Deserialize<'de> + Default + Sync,
{
    fn default() -> Self {
        MemeBuilder {
            key: String::new(),
            min_images: 0,
            max_images: 0,
            min_texts: 0,
            max_texts: 0,
            default_texts: Vec::new(),
            options: T::default(),
            keywords: Vec::new(),
            shortcuts: Vec::new(),
            tags: HashSet::new(),
            date_created: Local::now(),
            date_modified: Local::now(),
            function: |_, _, _| Ok(Vec::new()),
        }
    }
}

pub mod meme_setters {
    use crate::meme::MemeShortcut;
    use chrono::{DateTime, Local};
    use std::collections::HashSet;

    pub fn min_images(min_images: u8) -> u8 {
        min_images
    }

    pub fn max_images(max_images: u8) -> u8 {
        max_images
    }

    pub fn min_texts(min_texts: u8) -> u8 {
        min_texts
    }

    pub fn max_texts(max_texts: u8) -> u8 {
        max_texts
    }

    pub fn default_texts(default_texts: &Vec<&str>) -> Vec<String> {
        default_texts.iter().map(|text| text.to_string()).collect()
    }

    pub fn keywords(keywords: Vec<&str>) -> Vec<String> {
        keywords.iter().map(|keyword| keyword.to_string()).collect()
    }

    pub fn shortcuts(shortcuts: Vec<MemeShortcut>) -> Vec<MemeShortcut> {
        shortcuts
    }

    pub fn tags(tags: Vec<&str>) -> HashSet<String> {
        tags.iter().map(|tag| tag.to_string()).collect()
    }

    pub fn date_created(date_created: DateTime<Local>) -> DateTime<Local> {
        date_created
    }

    pub fn date_modified(date_modified: DateTime<Local>) -> DateTime<Local> {
        date_modified
    }
}

pub trait Meme: Send + Sync {
    fn key(&self) -> String;
    fn info(&self) -> MemeInfo;
    fn generate(
        &self,
        images: &Vec<InputImage>,
        texts: &Vec<String>,
        options: String,
    ) -> Result<Vec<u8>, Error>;
}

impl<T> Meme for MemeBuilder<T>
where
    T: MemeOptions + for<'de> Deserialize<'de> + Default + Sync,
{
    fn key(&self) -> String {
        self.key.clone()
    }

    fn info(&self) -> MemeInfo {
        MemeInfo {
            key: self.key.clone(),
            params: MemeParams {
                min_images: self.min_images,
                max_images: self.max_images,
                min_texts: self.min_texts,
                max_texts: self.max_texts,
                default_texts: self.default_texts.clone(),
                options: self.options.into_options(),
            },
            keywords: self.keywords.clone(),
            shortcuts: self.shortcuts.clone(),
            tags: self.tags.clone(),
            date_created: self.date_created.clone(),
            date_modified: self.date_modified.clone(),
        }
    }

    fn generate(
        &self,
        images: &Vec<InputImage>,
        texts: &Vec<String>,
        options: String,
    ) -> Result<Vec<u8>, Error> {
        let options = &serde_json::from_str(&options)?;
        let images = images
            .iter()
            .map(|image| image.decode())
            .collect::<Result<Vec<DecodedImage>, Error>>()?;
        (self.function)(&images, texts, options)
    }
}
