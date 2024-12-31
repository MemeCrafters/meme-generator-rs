use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local};
use serde::Deserialize;
use serde_json::{Number, Value};
use skia_safe::{Codec, Data, ISize, ImageInfo};

use meme_generator_core::{
    error::Error,
    meme::{self, Meme, MemeInfo, MemeOption, MemeParams, MemeShortcut, OptionValue},
};

use crate::{encoder::encode_png, tools::grid_pattern_image};

pub use meme_options_derive::MemeOptions;

pub trait MemeOptions: Default + for<'de> Deserialize<'de> + Send + Sync {
    fn to_options(&self) -> Vec<MemeOption>;
}

#[macro_export]
macro_rules! shortcut {
    ($pattern:expr, $($field:ident = $value:expr),* $(,)?) => {
        meme_generator_core::meme::MemeShortcut {
            pattern: $pattern.to_string(),
            $(
                $field: $crate::builder::shortcut_setters::$field($value),
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

pub struct NamedImage<'a> {
    pub name: String,
    pub codec: Codec<'a>,
}

impl<'a> NamedImage<'a> {
    pub fn from(input: &meme::Image) -> Result<NamedImage<'static>, Error> {
        let data = Data::new_copy(&input.data);
        let codec = Codec::from_data(data)
            .ok_or(Error::ImageDecodeError("Skia decode error".to_string()))?;
        Ok(NamedImage {
            name: input.name.clone(),
            codec: codec,
        })
    }

    pub fn info(&self) -> ImageInfo {
        self.codec.info()
    }

    pub fn dimensions(&self) -> ISize {
        self.codec.dimensions()
    }

    pub fn width(&self) -> i32 {
        self.codec.info().width()
    }

    pub fn height(&self) -> i32 {
        self.codec.info().height()
    }
}

type MemeFunction<T> = fn(Vec<NamedImage>, Vec<String>, T) -> Result<Vec<u8>, Error>;

pub struct MemeBuilder<T>
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

pub mod meme_setters {
    use chrono::{DateTime, Local};
    use meme_generator_core::meme::MemeShortcut;
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
        images: Vec<meme::Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
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

        let options = options
            .iter()
            .map(|(key, value)| {
                let value = match value {
                    OptionValue::Boolean(value) => Value::Bool(*value),
                    OptionValue::String(value) => Value::String(value.clone()),
                    OptionValue::Integer(value) => Value::Number(Number::from(*value)),
                    OptionValue::Float(value) => {
                        Value::Number(Number::from_f64(f64::from(*value)).unwrap())
                    }
                };
                (key.clone(), value)
            })
            .collect();

        let options = serde_json::from_value(Value::Object(options))
            .map_err(|err| Error::DeserializeError(err.to_string()))?;
        let images = images
            .iter()
            .map(|image| NamedImage::from(image))
            .collect::<Result<Vec<NamedImage>, Error>>()?;
        (self.function)(images, texts, options)
    }

    fn generate_preview(&self) -> Result<Vec<u8>, Error> {
        let mut images = Vec::new();
        if self.min_images > 0 {
            let image = encode_png(grid_pattern_image())?;
            for i in 0..self.min_images {
                let name = if self.min_images == 1 {
                    "{name}".to_string()
                } else {
                    format!("{{name{}}}", i + 1)
                };
                images.push(meme::Image {
                    name: name,
                    data: image.clone(),
                });
            }
        }
        let texts = if self.default_texts.len() >= self.min_texts as usize
            && self.default_texts.len() <= self.max_texts as usize
        {
            self.default_texts.clone()
        } else {
            let mut texts = Vec::new();
            for i in 0..self.min_texts {
                let text = if self.min_texts == 1 {
                    "{text}".to_string()
                } else {
                    format!("{{text{}}}", i + 1)
                };
                texts.push(text);
            }
            texts
        };
        let options = HashMap::new();
        self.generate(images, texts, options)
    }
}

#[macro_export]
macro_rules! meme_builder {
    ($key:expr, $function:expr, $($field:ident = $value:expr),* $(,)?) => {
        $crate::builder::MemeBuilder {
            key: $key.to_string(),
            function: $function,
            $(
                $field: $crate::builder::meme_setters::$field($value),
            )*
            ..Default::default()
        }
    }
}
