mod resources;
mod tools;

use meme_generator::{
    self, VERSION, error,
    meme::{self, OptionValue as MemeOptionValue},
};
use napi::bindgen_prelude::Buffer;
use napi_derive::napi;
use std::collections::{HashMap, HashSet};

#[napi(object)]
#[derive(Clone)]
/// 解析参数标志
pub struct ParserFlags {
    /// 是否使用短参数
    pub short: bool,
    /// 是否使用长参数
    pub long: bool,
    /// 短参数别名
    pub short_aliases: Vec<String>,
    /// 长参数别名
    pub long_aliases: Vec<String>,
}

#[napi(object)]
#[derive(Clone)]
/// 布尔型选项
pub struct BooleanOption {
    /// 选项名称
    pub name: String,
    /// 默认值
    pub default: Option<bool>,
    /// 选项描述
    pub description: Option<String>,
    /// 解析参数标志
    pub parser_flags: ParserFlags,
}

#[napi(object)]
#[derive(Clone)]
/// 字符串选项
pub struct StringOption {
    /// 选项名称
    pub name: String,
    /// 默认值
    pub default: Option<String>,
    /// 可选值
    pub choices: Option<Vec<String>>,
    /// 选项描述
    pub description: Option<String>,
    /// 解析参数标志
    pub parser_flags: ParserFlags,
}

#[napi(object)]
#[derive(Clone)]
/// 整型选项
pub struct IntegerOption {
    /// 选项名称
    pub name: String,
    /// 默认值
    pub default: Option<i32>,
    /// 最小值
    pub minimum: Option<i32>,
    /// 最大值
    pub maximum: Option<i32>,
    /// 选项描述
    pub description: Option<String>,
    /// 解析参数标志
    pub parser_flags: ParserFlags,
}

#[napi(object)]
#[derive(Clone)]
/// 浮点型选项
pub struct FloatOption {
    /// 选项名称
    pub name: String,
    /// 默认值
    pub default: Option<f64>,
    /// 最小值
    pub minimum: Option<f64>,
    /// 最大值
    pub maximum: Option<f64>,
    /// 选项描述
    pub description: Option<String>,
    /// 解析参数标志
    pub parser_flags: ParserFlags,
}

#[napi]
#[derive(Clone)]
/// 表情选项
pub enum MemeOption {
    /// 布尔型选项
    Boolean(BooleanOption),
    /// 字符串型选项
    String(StringOption),
    /// 整型选项
    Integer(IntegerOption),
    /// 浮点型选项
    Float(FloatOption),
}

#[napi(object)]
#[derive(Clone)]
/// 表情参数
pub struct MemeParams {
    /// 最小图片数量
    pub min_images: u8,
    /// 最大图片数量
    pub max_images: u8,
    /// 最小文字数量
    pub min_texts: u8,
    /// 最大文字数量
    pub max_texts: u8,
    /// 默认文字
    pub default_texts: Vec<String>,
    /// 选项
    pub options: Vec<MemeOption>,
}

#[napi(object)]
#[derive(Clone)]
/// 表情快捷方式
pub struct MemeShortcut {
    /// 快捷方式模式
    pub pattern: String,
    /// 快捷方式描述
    pub humanized: Option<String>,
    /// 快捷方式名称
    pub names: Vec<String>,
    /// 快捷方式文字
    pub texts: Vec<String>,
    /// 快捷方式选项
    pub options: HashMap<String, OptionValue>,
}

#[napi]
#[derive(Clone)]
pub enum OptionValue {
    /// 布尔型选项
    Boolean(bool),
    /// 字符串型选项
    String(String),
    /// 整型选项
    Integer(i32),
    /// 浮点型选项
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
/// 图片属性对象
pub struct Image {
    /// 图片名称
    pub name: String,
    /// 图片数据
    pub data: Buffer,
}

#[napi(object)]
#[derive(Clone)]
/// 表情信息
pub struct MemeInfo {
    /// 表情键
    pub key: String,
    /// 表情参数
    pub params: MemeParams,
    /// 表情关键词
    pub keywords: Vec<String>,
    /// 表情快捷方式
    pub shortcuts: Vec<MemeShortcut>,
    /// 表情标签
    pub tags: HashSet<String>,
    /// 表情创建时间
    pub date_created: String,
    /// 表情修改时间
    pub date_modified: String,
}

#[napi(object)]
#[derive(Clone)]
/// 图片解码错误
pub struct ImageDecodeError {
    pub error: String,
}

#[napi(object)]
#[derive(Clone)]
/// 图片编码错误
pub struct ImageEncodeError {
    pub error: String,
}

#[napi(object)]
#[derive(Clone)]
/// 图片资源缺失错误
pub struct ImageAssetMissing {
    /// 图片路径
    pub path: String,
}

#[napi(object)]
#[derive(Clone)]
/// 反序列化错误
pub struct DeserializeError {
    /// 错误信息
    pub error: String,
}

#[napi(object)]
#[derive(Clone)]
/// 图片数量不匹配错误
pub struct ImageNumberMismatch {
    /// 最小数量
    pub min: u8,
    /// 最大数量
    pub max: u8,
    /// 实际数量
    pub actual: u8,
}

#[napi(object)]
#[derive(Clone)]
/// 文字数量不匹配错误
pub struct TextNumberMismatch {
    /// 最小数量
    pub min: u8,
    /// 最大数量
    pub max: u8,
    /// 实际数量
    pub actual: u8,
}

#[napi(object)]
#[derive(Clone)]
/// 文字长度超过最大限制错误
pub struct TextOverLength {
    /// 文字内容
    pub text: String,
}

#[napi(object)]
#[derive(Clone)]
/// 表情反馈错误
pub struct MemeFeedback {
    /// 反馈内容
    pub feedback: String,
}

#[napi]
#[derive(Clone)]
pub enum Error {
    /// 图片解码错误
    ImageDecodeError(ImageDecodeError),
    /// 图片编码错误
    ImageEncodeError(ImageEncodeError),
    /// 图片资源缺失错误
    ImageAssetMissing(ImageAssetMissing),
    /// 反序列化错误
    DeserializeError(DeserializeError),
    /// 图片数量不匹配错误
    ImageNumberMismatch(ImageNumberMismatch),
    /// 文字数量不匹配错误
    TextNumberMismatch(TextNumberMismatch),
    /// 文字长度超过最大限制错误
    TextOverLength(TextOverLength),
    /// 表情反馈错误
    MemeFeedback(MemeFeedback),
}

#[napi]
pub enum MemeResult {
    /// 成功生成
    Ok(Buffer),
    /// 生成失败
    Err(Error),
}

#[napi]
pub struct Meme {
    meme: &'static dyn meme::Meme,
}

#[napi]
impl Meme {
    #[napi(getter)]
    /// 获取表情键
    pub fn key(&self) -> String {
        self.meme.key()
    }
    #[napi(getter)]
    /// 获取表情参数
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
            date_created: info.date_created.to_rfc3339(),
            date_modified: info.date_modified.to_rfc3339(),
        }
    }

    #[napi]
    /// 生成表情
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
    /// 生成预览图片
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
/// 获取版本号
pub fn version() -> String {
    VERSION.to_string()
}

#[napi(js_name = "get_meme")]
/// 获取meme对象
pub fn get_meme(key: String) -> Option<Meme> {
    meme_generator::get_meme(key.as_str()).map(|meme| Meme {
        meme: meme.as_ref(),
    })
}

#[napi(js_name = "get_memes")]
/// 获取所有meme对象
pub fn get_memes() -> Vec<Meme> {
    meme_generator::get_memes()
        .into_iter()
        .map(|meme| Meme {
            meme: meme.as_ref(),
        })
        .collect()
}

#[napi(js_name = "get_meme_keys")]
/// 获取所有meme键
pub fn get_meme_keys() -> Vec<&'static str> {
    meme_generator::get_meme_keys()
}

#[napi(js_name = "search_memes")]
/// 搜索meme
pub fn search_memes(query: String, include_tags: Option<bool>) -> Vec<String> {
    meme_generator::search_memes(query.as_str(), include_tags.unwrap_or(false))
}
