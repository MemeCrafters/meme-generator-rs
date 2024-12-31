use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use chrono::{DateTime, Local};
use pyo3::prelude::*;

use meme_generator::{error, load_memes, meme, resources, VERSION};

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
    options: HashMap<String, OptionValue>,
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
struct Image(String, Vec<u8>);

#[derive(FromPyObject, IntoPyObject, Clone)]
enum OptionValue {
    #[pyo3(transparent)]
    Boolean(bool),
    #[pyo3(transparent)]
    String(String),
    #[pyo3(transparent)]
    Integer(i32),
    #[pyo3(transparent)]
    Float(f32),
}

impl From<meme::OptionValue> for OptionValue {
    fn from(value: meme::OptionValue) -> Self {
        match value {
            meme::OptionValue::Boolean(value) => OptionValue::Boolean(value),
            meme::OptionValue::String(value) => OptionValue::String(value),
            meme::OptionValue::Integer(value) => OptionValue::Integer(value),
            meme::OptionValue::Float(value) => OptionValue::Float(value),
        }
    }
}

impl Into<meme::OptionValue> for OptionValue {
    fn into(self) -> meme::OptionValue {
        match self {
            OptionValue::Boolean(value) => meme::OptionValue::Boolean(value),
            OptionValue::String(value) => meme::OptionValue::String(value),
            OptionValue::Integer(value) => meme::OptionValue::Integer(value),
            OptionValue::Float(value) => meme::OptionValue::Float(value),
        }
    }
}

#[pyclass]
#[derive(Clone)]
struct ImageDecodeError {
    #[pyo3(get)]
    error: String,
}

#[pyclass]
#[derive(Clone)]
struct ImageEncodeError {
    #[pyo3(get)]
    error: String,
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
    meme: &'static Box<dyn meme::Meme>,
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
                            name: name,
                            default: default,
                            description: description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases,
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
                            name: name,
                            default: default,
                            choices: choices,
                            description: description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases,
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
                            name: name,
                            default: default,
                            minimum: minimum,
                            maximum: maximum,
                            description: description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases,
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
                            name: name,
                            default: default,
                            minimum: minimum,
                            maximum: maximum,
                            description: description,
                            parser_flags: ParserFlags {
                                short: parser_flags.short,
                                long: parser_flags.long,
                                short_aliases: parser_flags.short_aliases,
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

    fn generate(
        &self,
        images: Vec<Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> MemeResult {
        let images = images
            .into_iter()
            .map(|Image(name, data)| meme::Image { name, data })
            .collect::<Vec<_>>();

        let options = options
            .into_iter()
            .map(|(name, value)| (name, value.into()))
            .collect::<HashMap<_, _>>();

        let result = self.meme.generate(images, texts, options);
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
            error::Error::ImageDecodeError(error) => {
                MemeResult::Err(Error::ImageDecodeError(ImageDecodeError { error }))
            }
            error::Error::ImageEncodeError(error) => {
                MemeResult::Err(Error::ImageEncodeError(ImageEncodeError { error }))
            }
            error::Error::IOError(error) => MemeResult::Err(Error::IOError(IOError { error })),
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

static LOADED_MEMES: LazyLock<HashMap<String, Box<dyn meme::Meme>>> =
    LazyLock::new(|| load_memes());

#[pyfunction]
fn get_version() -> &'static str {
    VERSION
}

#[pyfunction]
fn get_meme(key: &str) -> Option<Meme> {
    match LOADED_MEMES.get(key) {
        Some(meme) => Some(Meme { meme }),
        None => None,
    }
}

#[pyfunction]
fn get_memes() -> Vec<Meme> {
    LOADED_MEMES
        .values()
        .into_iter()
        .map(|meme| Meme { meme })
        .collect()
}

#[pyfunction]
fn get_meme_keys() -> Vec<String> {
    let mut keys = LOADED_MEMES.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    keys
}

#[pyfunction]
fn check_resources() {
    resources::check_resources_sync(None);
}

#[pyfunction]
fn check_resources_in_background() {
    resources::check_resources_in_background(None);
}
