use chrono::{DateTime, Local};
use meme_generator::{config, error, manager, meme, resources, version};
use pyo3::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

#[pymodule(name = "meme_generator")]
fn meme_generator_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ParserFlags>()?;
    m.add_class::<BooleanOption>()?;
    m.add_class::<StringOption>()?;
    m.add_class::<IntegerOption>()?;
    m.add_class::<FloatOption>()?;
    m.add_class::<MemeParams>()?;
    m.add_class::<MemeShortcut>()?;
    m.add_class::<MemeInfo>()?;
    m.add_class::<ImageDecodeError>()?;
    m.add_class::<ImageEncodeError>()?;
    m.add_class::<IOError>()?;
    m.add_class::<DeserializeError>()?;
    m.add_class::<ImageNumberMismatch>()?;
    m.add_class::<TextNumberMismatch>()?;
    m.add_class::<TextOverLength>()?;
    m.add_class::<MemeFeedback>()?;
    m.add_class::<Meme>()?;
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(get_meme, m)?)?;
    m.add_function(wrap_pyfunction!(get_memes, m)?)?;
    m.add_function(wrap_pyfunction!(get_meme_keys, m)?)?;
    m.add_function(wrap_pyfunction!(check_resources, m)?)?;
    m.add_function(wrap_pyfunction!(check_resources_in_background, m)?)?;
    Ok(())
}

#[pyclass]
#[derive(Clone)]
struct ParserFlags {
    #[pyo3(get)]
    short: bool,
    #[pyo3(get)]
    long: bool,
    #[pyo3(get)]
    short_aliases: Vec<char>,
    #[pyo3(get)]
    long_aliases: Vec<String>,
}

#[pyclass]
#[derive(Clone)]
struct BooleanOption {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    default: Option<bool>,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    parser_flags: ParserFlags,
}

#[pyclass]
#[derive(Clone)]
struct StringOption {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    default: Option<String>,
    #[pyo3(get)]
    choices: Option<Vec<String>>,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    parser_flags: ParserFlags,
}

#[pyclass]
#[derive(Clone)]
struct IntegerOption {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    default: Option<i32>,
    #[pyo3(get)]
    minimum: Option<i32>,
    #[pyo3(get)]
    maximum: Option<i32>,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    parser_flags: ParserFlags,
}

#[pyclass]
#[derive(Clone)]
struct FloatOption {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    default: Option<f32>,
    #[pyo3(get)]
    minimum: Option<f32>,
    #[pyo3(get)]
    maximum: Option<f32>,
    #[pyo3(get)]
    description: Option<String>,
    #[pyo3(get)]
    parser_flags: ParserFlags,
}

#[derive(IntoPyObject, Clone)]
enum MemeOption {
    Boolean(BooleanOption),
    String(StringOption),
    Integer(IntegerOption),
    Float(FloatOption),
}

#[pyclass]
#[derive(Clone)]
struct MemeParams {
    #[pyo3(get)]
    min_images: u8,
    #[pyo3(get)]
    max_images: u8,
    #[pyo3(get)]
    min_texts: u8,
    #[pyo3(get)]
    max_texts: u8,
    #[pyo3(get)]
    default_texts: Vec<String>,
    #[pyo3(get)]
    options: Vec<MemeOption>,
}

#[pyclass]
#[derive(Clone)]
struct MemeShortcut {
    #[pyo3(get)]
    pattern: String,
    #[pyo3(get)]
    humanized: Option<String>,
    #[pyo3(get)]
    names: Vec<String>,
    #[pyo3(get)]
    texts: Vec<String>,
    #[pyo3(get)]
    parser_args: Vec<String>,
}

#[pyclass]
#[derive(Clone)]
struct MemeInfo {
    #[pyo3(get)]
    key: String,
    #[pyo3(get)]
    params: MemeParams,
    #[pyo3(get)]
    keywords: Vec<String>,
    #[pyo3(get)]
    shortcuts: Vec<MemeShortcut>,
    #[pyo3(get)]
    tags: HashSet<String>,
    #[pyo3(get)]
    date_created: DateTime<Local>,
    #[pyo3(get)]
    date_modified: DateTime<Local>,
}

#[derive(FromPyObject, Clone)]
struct RawImage(String, Vec<u8>);

#[derive(FromPyObject, Clone)]
enum OptionValue {
    #[pyo3(transparent, annotation = "bool")]
    Boolean(bool),
    #[pyo3(transparent, annotation = "str")]
    String(String),
    #[pyo3(transparent, annotation = "int")]
    Integer(i32),
    #[pyo3(transparent, annotation = "float")]
    Float(f32),
}

#[pyclass]
#[derive(Clone)]
struct ImageDecodeError {
    #[pyo3(get)]
    error: Option<String>,
}

#[pyclass]
#[derive(Clone)]
struct ImageEncodeError {
    #[pyo3(get)]
    error: Option<String>,
}

#[pyclass]
#[derive(Clone)]
struct IOError {
    #[pyo3(get)]
    error: String,
}

#[pyclass]
#[derive(Clone)]
struct DeserializeError {
    #[pyo3(get)]
    error: String,
}

#[pyclass]
#[derive(Clone)]
struct ImageNumberMismatch {
    #[pyo3(get)]
    min: u8,
    #[pyo3(get)]
    max: u8,
    #[pyo3(get)]
    actual: u8,
}

#[pyclass]
#[derive(Clone)]
struct TextNumberMismatch {
    #[pyo3(get)]
    min: u8,
    #[pyo3(get)]
    max: u8,
    #[pyo3(get)]
    actual: u8,
}

#[pyclass]
#[derive(Clone)]
struct TextOverLength {
    #[pyo3(get)]
    text: String,
}

#[pyclass]
#[derive(Clone)]
struct MemeFeedback {
    #[pyo3(get)]
    feedback: String,
}

#[derive(IntoPyObject, Clone)]
enum Error {
    ImageDecodeError(ImageDecodeError),
    ImageEncodeError(ImageEncodeError),
    IOError(IOError),
    DeserializeError(DeserializeError),
    ImageNumberMismatch(ImageNumberMismatch),
    TextNumberMismatch(TextNumberMismatch),
    TextOverLength(TextOverLength),
    MemeFeedback(MemeFeedback),
}

#[derive(IntoPyObject, Clone)]
enum MemeResult {
    Ok(Vec<u8>),
    Err(Error),
}

#[pyclass]
struct Meme {
    meme: Arc<dyn meme::Meme>,
}

#[pymethods]
impl Meme {
    #[getter]
    fn key(&self) -> String {
        self.meme.key()
    }

    #[getter]
    fn info(&self) -> MemeInfo {
        let info = self.meme.info();
        MemeInfo {
            key: info.key,
            params: MemeParams {
                min_images: info.params.min_images,
                max_images: info.params.max_images,
                min_texts: info.params.min_texts,
                max_texts: info.params.max_texts,
                default_texts: info.params.default_texts.clone(),
                options: info
                    .params
                    .options
                    .iter()
                    .map(|option| match option {
                        meme::MemeOption::Boolean {
                            name,
                            default,
                            description,
                            parser_flags,
                        } => MemeOption::Boolean(BooleanOption {
                            name: name.clone(),
                            default: default.clone(),
                            description: description.clone(),
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases.clone(),
                                long_aliases: parser_flags.long_aliases.clone(),
                            },
                        }),
                        meme::MemeOption::String {
                            name,
                            default,
                            choices,
                            description,
                            parser_flags,
                        } => MemeOption::String(StringOption {
                            name: name.clone(),
                            default: default.clone(),
                            choices: choices.clone(),
                            description: description.clone(),
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases.clone(),
                                long_aliases: parser_flags.long_aliases.clone(),
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
                            name: name.clone(),
                            default: default.clone(),
                            minimum: minimum.clone(),
                            maximum: maximum.clone(),
                            description: description.clone(),
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases.clone(),
                                long_aliases: parser_flags.long_aliases.clone(),
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
                            name: name.clone(),
                            default: default.clone(),
                            minimum: minimum.clone(),
                            maximum: maximum.clone(),
                            description: description.clone(),
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases.clone(),
                                long_aliases: parser_flags.long_aliases.clone(),
                            },
                        }),
                    })
                    .collect(),
            },
            keywords: info.keywords.clone(),
            shortcuts: info
                .shortcuts
                .iter()
                .map(|shortcut| MemeShortcut {
                    pattern: shortcut.pattern.clone(),
                    humanized: shortcut.humanized.clone(),
                    names: shortcut.names.clone(),
                    texts: shortcut.texts.clone(),
                    parser_args: shortcut.parser_args.clone(),
                })
                .collect(),
            tags: info.tags.clone(),
            date_created: info.date_created.clone(),
            date_modified: info.date_modified.clone(),
        }
    }

    fn generate(
        &self,
        images: Vec<RawImage>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> MemeResult {
        let images = images
            .into_iter()
            .map(|RawImage(name, data)| meme::RawImage { name, data })
            .collect::<Vec<_>>();

        let options = options
            .into_iter()
            .map(|(name, value)| {
                (
                    name,
                    match value {
                        OptionValue::Boolean(value) => meme::OptionValue::Boolean(value),
                        OptionValue::String(value) => meme::OptionValue::String(value),
                        OptionValue::Integer(value) => meme::OptionValue::Integer(value),
                        OptionValue::Float(value) => meme::OptionValue::Float(value),
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        let result = self.meme.generate(&images, &texts, &options);
        handle_result(result)
    }

    fn generate_preview(&self) -> MemeResult {
        let result = self.meme.generate_preview();
        handle_result(result)
    }
}

fn handle_result(result: Result<Vec<u8>, error::Error>) -> MemeResult {
    match result {
        Ok(data) => MemeResult::Ok(data),
        Err(error) => match error {
            error::Error::ImageDecodeError(Some(err)) => {
                MemeResult::Err(Error::ImageDecodeError(ImageDecodeError {
                    error: Some(format!("{err:?}")),
                }))
            }
            error::Error::ImageDecodeError(None) => {
                MemeResult::Err(Error::ImageDecodeError(ImageDecodeError { error: None }))
            }
            error::Error::ImageEncodeError(encode_err) => {
                MemeResult::Err(Error::ImageEncodeError(match encode_err {
                    error::EncodeError::GifEncodeError(err) => ImageEncodeError {
                        error: Some(format!("{err}")),
                    },
                    error::EncodeError::SkiaEncodeError => ImageEncodeError { error: None },
                }))
            }
            error::Error::IOError(err) => MemeResult::Err(Error::IOError(IOError {
                error: format!("{err}"),
            })),
            error::Error::DeserializeError(err) => {
                MemeResult::Err(Error::DeserializeError(DeserializeError {
                    error: format!("{err}"),
                }))
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

#[pyfunction]
fn get_version() -> String {
    version::VERSION.to_string()
}

#[pyfunction]
fn get_meme(key: &str) -> Option<Meme> {
    match manager::get_meme(key) {
        Some(meme) => Some(Meme { meme }),
        None => None,
    }
}

#[pyfunction]
fn get_memes() -> Vec<Meme> {
    manager::get_memes()
        .into_iter()
        .map(|meme| Meme { meme })
        .collect()
}

#[pyfunction]
fn get_meme_keys() -> Vec<String> {
    manager::get_meme_keys()
}

#[pyfunction]
fn check_resources() {
    resources::check_resources_sync(config::MEME_CONFIG.resource.resource_url.as_str());
}

#[pyfunction]
fn check_resources_in_background() {
    resources::check_resources_in_background(config::MEME_CONFIG.resource.resource_url.as_str());
}
