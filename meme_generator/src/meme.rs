use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use skia_safe::{Codec, Data};

use crate::{
    error::Error,
    utils::{encoder::encode_png, tools::empty_image},
};

pub(crate) use meme_options_derive::MemeOptions;

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
pub enum OptionValue {
    Boolean(bool),
    String(String),
    Integer(i32),
    Float(f32),
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

pub(crate) use shortcut;

#[allow(dead_code)]
pub(crate) mod shortcut_setters {
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

pub(crate) trait MemeOptions: Default + for<'de> Deserialize<'de> + Send + Sync {
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
    T: MemeOptions,
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
    T: MemeOptions,
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
        options: &HashMap<String, OptionValue>,
    ) -> Result<Vec<u8>, Error>;
    fn generate_preview(&self) -> Result<Vec<u8>, Error>;
}

impl<T> Meme for MemeBuilder<T>
where
    T: MemeOptions,
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
        options: &HashMap<String, OptionValue>,
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

        let mut options_json = Map::new();
        for option in options {
            let key = option.0;
            let value = option.1;
            let value = match value {
                OptionValue::Boolean(value) => Value::Bool(value.clone()),
                OptionValue::String(value) => Value::String(value.clone()),
                OptionValue::Integer(value) => Value::Number(Number::from(value.clone())),
                OptionValue::Float(value) => {
                    Value::Number(Number::from_f64(f64::from(value.clone())).unwrap())
                }
            };
            options_json.insert(key.clone(), value);
        }

        let options = &serde_json::from_value(Value::Object(options_json))?;
        let mut images = images
            .iter()
            .map(|image| DecodedImage::from(image))
            .collect::<Result<Vec<DecodedImage>, Error>>()?;
        (self.function)(&mut images, texts, options)
    }

    fn generate_preview(&self) -> Result<Vec<u8>, Error> {
        let mut images = Vec::new();
        if self.min_images > 0 {
            let image = encode_png(&empty_image())?;
            for i in 0..self.min_images {
                let name = if self.min_images == 1 {
                    "{name}".to_string()
                } else {
                    format!("{{name{}}}", i + 1)
                };
                images.push(RawImage {
                    name: name,
                    data: image.clone(),
                });
            }
        }
        let mut texts = Vec::new();
        for i in 0..self.min_texts {
            let text = if self.min_texts == 1 {
                "{text}".to_string()
            } else {
                format!("{{text{}}}", i + 1)
            };
            texts.push(text);
        }
        let options = HashMap::new();
        self.generate(&images, &texts, &options)
    }
}
