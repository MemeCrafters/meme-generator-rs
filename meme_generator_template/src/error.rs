use std::fmt;

use meme_generator_core::error::Error as CoreError;

/// Template-specific error type with detailed categorization.
#[derive(Debug)]
pub enum TemplateError {
    /// Expression parsing error (tokenizer/parser)
    ExprParse(String),
    /// Expression evaluation error (undefined variable, division by zero, etc.)
    ExprEval(String),
    /// Type mismatch when converting values
    TypeError {
        expected: &'static str,
        actual: String,
    },
    /// Resource reference/loading error
    Resource(String),
    /// Frame variable resolution error
    FrameVar(String),
    /// Image operation error
    Operation(String),
    /// Element processing error
    Element(String),
    /// Layer composition error
    Layer(String),
    /// Template configuration error (invalid font_style, text_align, etc.)
    Config(String),
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::ExprParse(msg) => write!(f, "expression parse error: {msg}"),
            TemplateError::ExprEval(msg) => write!(f, "expression eval error: {msg}"),
            TemplateError::TypeError { expected, actual } => {
                write!(f, "type error: expected {expected}, got {actual}")
            }
            TemplateError::Resource(msg) => write!(f, "resource error: {msg}"),
            TemplateError::FrameVar(msg) => write!(f, "frame variable error: {msg}"),
            TemplateError::Operation(msg) => write!(f, "operation error: {msg}"),
            TemplateError::Element(msg) => write!(f, "element error: {msg}"),
            TemplateError::Layer(msg) => write!(f, "layer error: {msg}"),
            TemplateError::Config(msg) => write!(f, "config error: {msg}"),
        }
    }
}

impl std::error::Error for TemplateError {}

impl From<TemplateError> for CoreError {
    fn from(err: TemplateError) -> Self {
        CoreError::TemplateError(err.to_string())
    }
}

/// Extension trait for adding location context to error results.
pub trait ContextExt<T> {
    fn context(self, ctx: impl std::fmt::Display) -> Result<T, CoreError>;
}

impl<T> ContextExt<T> for Result<T, CoreError> {
    fn context(self, ctx: impl std::fmt::Display) -> Result<T, CoreError> {
        self.map_err(|e| match e {
            CoreError::TemplateError(msg) => CoreError::TemplateError(format!("[{ctx}] {msg}")),
            other => other,
        })
    }
}

impl<T> ContextExt<T> for Result<T, TemplateError> {
    fn context(self, ctx: impl std::fmt::Display) -> Result<T, CoreError> {
        self.map_err(|e| CoreError::TemplateError(format!("[{ctx}] {e}")))
    }
}
