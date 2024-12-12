use std::collections::HashSet;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use skia_safe::{Codec, Data};

use crate::error::Error;

pub use meme_options_derive::ToMemeOptions;

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
    pub parser_args: Vec<String>,
}

impl Default for MemeShortcut {
    fn default() -> Self {
        MemeShortcut {
            pattern: String::new(),
            humanized: None,
            names: Vec::new(),
            texts: Vec::new(),
            parser_args: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! shortcut {
    ($pattern:expr, $($field:ident = $value:expr),* $(,)?) => {
        crate::meme::MemeShortcut {
            pattern: $pattern.to_string(),
            $(
                $field: crate::meme::shortcut_setters::$field($value),
            )*
            ..Default::default()
        }
    };
}

pub mod shortcut_setters {
    pub fn humanized(humanized: &str) -> Option<String> {
        Some(humanized.to_string())
    }

    pub fn names(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    pub fn texts(texts: &[&str]) -> Vec<String> {
        texts.iter().map(|text| text.to_string()).collect()
    }

    pub fn parser_args(parser_args: &[&str]) -> Vec<String> {
        parser_args.iter().map(|arg| arg.to_string()).collect()
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

pub trait ToMemeOptions: Default + Send {
    fn to_options(&self) -> Vec<MemeOption>;
}

pub struct RawImage {
    pub name: String,
    pub data: Vec<u8>,
}

pub struct DecodedImage<'a> {
    pub name: String,
    pub codec: Codec<'a>,
}

impl<'a> DecodedImage<'a> {
    pub fn from(input: &RawImage) -> Result<DecodedImage<'static>, Error> {
        let data = Data::new_copy(&input.data);
        let codec = Codec::from_data(data).ok_or(Error::ImageDecodeError(None))?;
        Ok(DecodedImage {
            name: input.name.clone(),
            codec: codec,
        })
    }
}

type MemeFunction<T> = fn(&mut Vec<DecodedImage>, &Vec<String>, &T) -> Result<Vec<u8>, Error>;

pub(crate) struct MemeBuilder<T>
where
    T: ToMemeOptions + for<'de> Deserialize<'de> + Sync,
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
    T: ToMemeOptions + for<'de> Deserialize<'de> + Sync,
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

pub(crate) mod meme_setters {
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

    pub fn default_texts(default_texts: &[&str]) -> Vec<String> {
        default_texts.iter().map(|text| text.to_string()).collect()
    }

    pub fn keywords(keywords: &[&str]) -> Vec<String> {
        keywords.iter().map(|keyword| keyword.to_string()).collect()
    }

    pub fn shortcuts(shortcuts: &[MemeShortcut]) -> Vec<MemeShortcut> {
        shortcuts.to_vec()
    }

    pub fn tags(tags: HashSet<String>) -> HashSet<String> {
        tags
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
        images: &Vec<RawImage>,
        texts: &Vec<String>,
        options: &Map<String, Value>,
    ) -> Result<Vec<u8>, Error>;
}

impl<T> Meme for MemeBuilder<T>
where
    T: ToMemeOptions + for<'de> Deserialize<'de> + Sync,
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
                options: self.options.to_options(),
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
        images: &Vec<RawImage>,
        texts: &Vec<String>,
        options: &Map<String, Value>,
    ) -> Result<Vec<u8>, Error> {
        let info = self.info();
        if images.len() < info.params.min_images as usize
            || images.len() > info.params.max_images as usize
        {
            return Err(Error::ImageNumberMismatch(
                info.params.min_images,
                info.params.max_images,
                images.len() as u8,
            ));
        }
        if texts.len() < info.params.min_texts as usize
            || texts.len() > info.params.max_texts as usize
        {
            return Err(Error::TextNumberMismatch(
                info.params.min_texts,
                info.params.max_texts,
                texts.len() as u8,
            ));
        }
        let options = &serde_json::from_value(Value::Object(options.clone()))?;
        let mut images = images
            .iter()
            .map(|image| DecodedImage::from(image))
            .collect::<Result<Vec<DecodedImage>, Error>>()?;
        (self.function)(&mut images, texts, options)
    }
}
