use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Local};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

use meme_generator::{
    self, VERSION, error,
    meme::{self, OptionValue as MemeOptionValue},
};

mod resources;
mod tools;

#[napi(object)]
#[derive(Clone)]
pub struct ParserFlags {
    pub short: bool,
    pub long: bool,
    pub short_aliases: Vec<String>,
    pub long_aliases: Vec<String>,
}

#[napi(object)]
#[derive(Clone)]
pub struct BooleanOption {
    pub name: String,
    pub default: Option<bool>,
    pub description: Option<String>,
    pub parser_flags: ParserFlags,
}

#[napi(object)]
#[derive(Clone)]
pub struct StringOption {
    pub name: String,
    pub default: Option<String>,
    pub choices: Option<Vec<String>>,
    pub description: Option<String>,
    pub parser_flags: ParserFlags,
}

#[napi(object)]
#[derive(Clone)]
pub struct IntegerOption {
    pub name: String,
    pub default: Option<i32>,
    pub minimum: Option<i32>,
    pub maximum: Option<i32>,
    pub description: Option<String>,
    pub parser_flags: ParserFlags,
}

#[napi(object)]
#[derive(Clone)]
pub struct FloatOption {
    pub name: String,
    pub default: Option<f64>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub description: Option<String>,
    pub parser_flags: ParserFlags,
}

#[napi]
#[derive(Clone)]
pub enum MemeOption {
    Boolean(BooleanOption),
    String(StringOption),
    Integer(IntegerOption),
    Float(FloatOption),
}

#[napi(object)]
#[derive(Clone)]
pub struct MemeParams {
    pub min_images: u8,
    pub max_images: u8,
    pub min_texts: u8,
    pub max_texts: u8,
    pub default_texts: Vec<String>,
    pub options: Vec<MemeOption>,
}

#[napi(object)]
#[derive(Clone)]
pub struct MemeShortcut {
    pub pattern: String,
    pub humanized: Option<String>,
    pub names: Vec<String>,
    pub texts: Vec<String>,
    pub options: HashMap<String, OptionValue>,
}

#[napi]
#[derive(Clone)]
pub enum OptionValue {
    Boolean(bool),
    String(String),
    Integer(i32),
    Float(f64),
}

impl From<MemeOptionValue> for OptionValue {
    fn from(value: MemeOptionValue) -> Self {
        match value {
            MemeOptionValue::Boolean(b) => OptionValue::Boolean(b),
            MemeOptionValue::String(s) => OptionValue::String(s),
            MemeOptionValue::Integer(i) => OptionValue::Integer(i),
            MemeOptionValue::Float(f) => OptionValue::Float(f as f64),
        }
    }
}

impl From<OptionValue> for MemeOptionValue {
    fn from(value: OptionValue) -> Self {
        match value {
            OptionValue::Boolean(b) => MemeOptionValue::Boolean(b),
            OptionValue::String(s) => MemeOptionValue::String(s),
            OptionValue::Integer(i) => MemeOptionValue::Integer(i),
            OptionValue::Float(f) => MemeOptionValue::Float(f as f32),
        }
    }
}

#[napi(object)]
pub struct Image {
    pub name: String,
    pub data: Buffer,
}

#[napi(object)]
#[derive(Clone)]
pub struct MemeInfo {
    pub key: String,
    pub params: MemeParams,
    pub keywords: Vec<String>,
    pub shortcuts: Vec<MemeShortcut>,
    pub tags: HashSet<String>,
    pub date_created: DateTime<Local>,
    pub date_modified: DateTime<Local>,
}

#[napi(object)]
#[derive(Clone)]
pub struct ImageDecodeError {
    pub error: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct ImageEncodeError {
    pub error: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct ImageAssetMissing {
    pub path: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct DeserializeError {
    pub error: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct ImageNumberMismatch {
    pub min: u8,
    pub max: u8,
    pub actual: u8,
}

#[napi(object)]
#[derive(Clone)]
pub struct TextNumberMismatch {
    pub min: u8,
    pub max: u8,
    pub actual: u8,
}

#[napi(object)]
#[derive(Clone)]
pub struct TextOverLength {
    pub text: String,
}

#[napi(object)]
#[derive(Clone)]
pub struct MemeFeedback {
    pub feedback: String,
}

#[napi]
#[derive(Clone)]
pub enum Error {
    ImageDecodeError(ImageDecodeError),
    ImageEncodeError(ImageEncodeError),
    ImageAssetMissing(ImageAssetMissing),
    DeserializeError(DeserializeError),
    ImageNumberMismatch(ImageNumberMismatch),
    TextNumberMismatch(TextNumberMismatch),
    TextOverLength(TextOverLength),
    MemeFeedback(MemeFeedback),
}

#[napi]
pub enum MemeResult {
    Ok(Buffer),
    Err(Error),
}

#[napi]
pub struct Meme {
    meme: &'static Box<dyn meme::Meme>,
}

#[napi]
impl Meme {
    #[napi(getter)]
    pub fn key(&self) -> String {
        self.meme.key()
    }

    #[napi(getter)]
    pub fn info(&self) -> MemeInfo {
        let info = self.meme.info();
        MemeInfo {
            key: info.key,
            params: MemeParams {
                min_images: info.params.min_images,
                max_images: info.params.max_images,
                min_texts: info.params.min_texts,
                max_texts: info.params.max_texts,
                default_texts: info.params.default_texts,
                options: info
                    .params
                    .options
                    .into_iter()
                    .map(|option| match option {
                        meme::MemeOption::Boolean {
                            name,
                            default,
                            description,
                            parser_flags,
                        } => MemeOption::Boolean(BooleanOption {
                            name,
                            default,
                            description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags
                                    .short_aliases
                                    .into_iter()
                                    .map(|c| c.to_string())
                                    .collect(),
                                long_aliases: parser_flags.long_aliases,
                            },
                        }),
                        meme::MemeOption::String {
                            name,
                            default,
                            choices,
                            description,
                            parser_flags,
                        } => MemeOption::String(StringOption {
                            name,
                            default,
                            choices,
                            description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags
                                    .short_aliases
                                    .into_iter()
                                    .map(|c| c.to_string())
                                    .collect(),
                                long_aliases: parser_flags.long_aliases,
                            },
                        }),
                        meme::MemeOption::Integer {
                            name,
                            default,
                            minimum,
                            maximum,
                            description,
                            parser_flags,
                        } => MemeOption::Integer(IntegerOption {
                            name,
                            default,
                            minimum,
                            maximum,
                            description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags
                                    .short_aliases
                                    .into_iter()
                                    .map(|c| c.to_string())
                                    .collect(),
                                long_aliases: parser_flags.long_aliases,
                            },
                        }),
                        meme::MemeOption::Float {
                            name,
                            default,
                            minimum,
                            maximum,
                            description,
                            parser_flags,
                        } => MemeOption::Float(FloatOption {
                            name,
                            default: default.map(|d| d as f64),
                            minimum: minimum.map(|m| m as f64),
                            maximum: maximum.map(|m| m as f64),
                            description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags
                                    .short_aliases
                                    .into_iter()
                                    .map(|c| c.to_string())
                                    .collect(),
                                long_aliases: parser_flags.long_aliases,
                            },
                        }),
                    })
                    .collect(),
            },
            keywords: info.keywords,
            shortcuts: info
                .shortcuts
                .into_iter()
                .map(|shortcut| MemeShortcut {
                    pattern: shortcut.pattern,
                    humanized: shortcut.humanized,
                    names: shortcut.names,
                    texts: shortcut.texts,
                    options: shortcut
                        .options
                        .into_iter()
                        .map(|(name, value)| (name, OptionValue::from(value)))
                        .collect(),
                })
                .collect(),
            tags: info.tags,
            date_created: info.date_created,
            date_modified: info.date_modified,
        }
    }

    #[napi]
    pub fn generate(
        &self,
        images: Vec<Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> MemeResult {
        let images = images
            .into_iter()
            .map(|Image { name, data }| meme::Image {
                name,
                data: data.to_vec(),
            })
            .collect::<Vec<_>>();

        let options = options
            .into_iter()
            .map(|(name, value)| (name, value.into()))
            .collect::<HashMap<_, _>>();

        let result = self.meme.generate(images, texts, options);
        handle_result(result)
    }

    #[napi]
    pub fn generate_preview(&self, options: Option<HashMap<String, OptionValue>>) -> MemeResult {
        let options = options.unwrap_or_default();

        let options = options
            .into_iter()
            .map(|(name, value)| (name, value.into()))
            .collect::<HashMap<_, _>>();

        let result = self.meme.generate_preview(options);
        handle_result(result)
    }
}

fn handle_result(result: Result<Vec<u8>, error::Error>) -> MemeResult {
    match result {
        Ok(data) => MemeResult::Ok(Buffer::from(data)),
        Err(error) => match error {
            error::Error::ImageDecodeError(error) => {
                MemeResult::Err(Error::ImageDecodeError(ImageDecodeError { error }))
            }
            error::Error::ImageEncodeError(error) => {
                MemeResult::Err(Error::ImageEncodeError(ImageEncodeError { error }))
            }
            error::Error::ImageAssetMissing(path) => {
                MemeResult::Err(Error::ImageAssetMissing(ImageAssetMissing { path }))
            }
            error::Error::DeserializeError(error) => {
                MemeResult::Err(Error::DeserializeError(DeserializeError { error }))
            }
            error::Error::ImageNumberMismatch(min, max, actual) => {
                MemeResult::Err(Error::ImageNumberMismatch(ImageNumberMismatch {
                    min,
                    max,
                    actual,
                }))
            }
            error::Error::TextNumberMismatch(min, max, actual) => {
                MemeResult::Err(Error::TextNumberMismatch(TextNumberMismatch {
                    min,
                    max,
                    actual,
                }))
            }
            error::Error::TextOverLength(text) => {
                MemeResult::Err(Error::TextOverLength(TextOverLength { text }))
            }
            error::Error::MemeFeedback(feedback) => {
                MemeResult::Err(Error::MemeFeedback(MemeFeedback { feedback }))
            }
        },
    }
}

#[napi]
pub fn get_version() -> String {
    VERSION.to_string()
}

#[napi]
pub fn get_meme(key: String) -> Option<Meme> {
    meme_generator::get_meme(key.as_str()).map(|meme| Meme { meme })
}

#[napi]
#[derive(Clone, PartialEq)]
pub enum MemeSortBy {
    Key = 0,
    Keywords = 1,
    KeywordsPinyin = 2,
    DateCreated = 3,
    DateModified = 4,
}

impl Into<meme_generator::MemeSortBy> for MemeSortBy {
    fn into(self) -> meme_generator::MemeSortBy {
        match self {
            MemeSortBy::Key => meme_generator::MemeSortBy::Key,
            MemeSortBy::Keywords => meme_generator::MemeSortBy::Keywords,
            MemeSortBy::KeywordsPinyin => meme_generator::MemeSortBy::KeywordsPinyin,
            MemeSortBy::DateCreated => meme_generator::MemeSortBy::DateCreated,
            MemeSortBy::DateModified => meme_generator::MemeSortBy::DateModified,
        }
    }
}

#[napi]
pub fn get_memes(sort_by: Option<MemeSortBy>, sort_reverse: Option<bool>) -> Vec<Meme> {
    let sort_by = sort_by.unwrap_or(MemeSortBy::Key);
    meme_generator::get_memes_sorted(sort_by.into(), sort_reverse.unwrap_or(false))
        .into_iter()
        .map(|meme| Meme { meme })
        .collect()
}

#[napi]
pub fn get_meme_keys(sort_by: Option<MemeSortBy>, sort_reverse: Option<bool>) -> Vec<String> {
    let sort_by = sort_by.unwrap_or(MemeSortBy::Key);
    meme_generator::get_meme_keys_sorted(sort_by.into(), sort_reverse.unwrap_or(false))
}

#[napi]
pub fn search_memes(query: String, include_tags: Option<bool>) -> Vec<String> {
    meme_generator::search_memes(query.as_str(), include_tags.unwrap_or(false))
}
