//! YAML template schema types with custom deserialization.

use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, SeqAccess, Visitor},
};

// ── Top-level template ──

#[derive(Debug, Clone, Deserialize)]
pub struct Template {
    pub version: String,
    pub metadata: Metadata,
    #[serde(default)]
    pub config: Config,
    #[serde(default)]
    pub resources: IndexMap<String, ResourceDef>,
    pub frames: Option<FramesDef>,
    #[serde(default)]
    pub elements: IndexMap<String, ElementDef>,
    pub canvas: CanvasDef,
    pub layers: Vec<LayerDef>,
}

// ── Metadata ──

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub key: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub date_created: String,
    pub date_modified: String,
    #[serde(default)]
    pub params: Params,
    #[serde(default)]
    pub shortcuts: Vec<ShortcutDef>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Params {
    #[serde(default)]
    pub min_images: u8,
    #[serde(default)]
    pub max_images: u8,
    #[serde(default)]
    pub min_texts: u8,
    #[serde(default)]
    pub max_texts: u8,
    #[serde(default)]
    pub default_texts: Vec<String>,
    #[serde(default)]
    pub options: Vec<OptionDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OptionDef {
    pub name: String,
    #[serde(rename = "type")]
    pub option_type: OptionType,
    pub default: Option<yaml_serde::Value>,
    pub description: Option<String>,
    pub choices: Option<Vec<String>>,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    #[serde(default)]
    pub parser_flags: ParserFlagsDef,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OptionType {
    Boolean,
    String,
    Integer,
    Float,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ParserFlagsDef {
    #[serde(default)]
    pub short: bool,
    #[serde(default)]
    pub long: bool,
    #[serde(default)]
    pub short_aliases: Vec<String>,
    #[serde(default)]
    pub long_aliases: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ShortcutDef {
    pub pattern: String,
    pub humanized: Option<String>,
    #[serde(default)]
    pub texts: Vec<String>,
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub options: HashMap<String, yaml_serde::Value>,
}

// ── Config ──

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub gif_input: bool,
}

// ── Resources ──

#[derive(Debug, Clone)]
pub enum ResourceDef {
    Single(String),
    Sequence {
        pattern: String,
        count: usize,
        start: usize,
    },
}

impl<'de> Deserialize<'de> for ResourceDef {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ResourceVisitor;
        impl<'de> Visitor<'de> for ResourceVisitor {
            type Value = ResourceDef;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a string path or a sequence definition object")
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<ResourceDef, E> {
                Ok(ResourceDef::Single(v.to_string()))
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<ResourceDef, A::Error> {
                let mut pattern = None;
                let mut count = None;
                let mut start = 0usize;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "pattern" => pattern = Some(map.next_value::<String>()?),
                        "count" => count = Some(map.next_value::<usize>()?),
                        "start" => start = map.next_value::<usize>()?,
                        _ => {
                            let _ = map.next_value::<yaml_serde::Value>()?;
                        }
                    }
                }
                Ok(ResourceDef::Sequence {
                    pattern: pattern.ok_or_else(|| de::Error::missing_field("pattern"))?,
                    count: count.ok_or_else(|| de::Error::missing_field("count"))?,
                    start,
                })
            }
        }
        deserializer.deserialize_any(ResourceVisitor)
    }
}

// ── Frames ──

#[derive(Debug, Clone, Deserialize)]
pub struct FramesDef {
    pub duration: f32,
    pub count: usize,
    pub align: Option<String>,
    #[serde(default)]
    pub vars: IndexMap<String, FrameVarDef>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FrameVarDef {
    #[serde(rename = "type")]
    pub var_type: FrameVarType,
    pub values: Vec<yaml_serde::Value>,
}

/// Frame variable type supporting base types and nested arrays with optional lengths.
///
/// Parsed from strings like `"number"`, `"number[2]"`, `"number[]"`, `"number[4][2]"`.
#[derive(Debug, Clone, PartialEq)]
pub struct FrameVarType {
    pub base: FrameVarBase,
    /// Array dimensions, outermost first.
    /// Empty = scalar, `[Some(2)]` = `T[2]`, `[None]` = `T[]`, `[Some(4), Some(2)]` = `T[4][2]`.
    pub dims: Vec<Option<usize>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FrameVarBase {
    Number,
    String,
    Bool,
}

impl<'de> Deserialize<'de> for FrameVarType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = std::string::String::deserialize(deserializer)?;
        let mut rest = s.as_str();
        let mut dims = Vec::new();
        while let Some(bracket) = rest.rfind('[') {
            let tail = &rest[bracket..];
            if !tail.ends_with(']') {
                break;
            }
            let inner = &tail[1..tail.len() - 1];
            let dim = if inner.is_empty() {
                None
            } else {
                Some(inner.parse::<usize>().map_err(|_| {
                    serde::de::Error::custom(format!("invalid array length in '{s}'"))
                })?)
            };
            dims.push(dim);
            rest = &rest[..bracket];
        }
        dims.reverse(); // outermost first
        let base = match rest {
            "number" => FrameVarBase::Number,
            "string" => FrameVarBase::String,
            "bool" => FrameVarBase::Bool,
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "unknown frame var type: '{s}'"
                )));
            }
        };
        Ok(FrameVarType { base, dims })
    }
}

// ── Elements ──

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ElementDef {
    Image(ImageElementDef),
    Text(TextElementDef),
    Canvas(CanvasElementDef),
}

#[derive(Debug, Clone, Deserialize)]
pub struct ImageElementDef {
    pub from: String,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub operations: Vec<OperationDef>,
    #[serde(rename = "if")]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextElementDef {
    pub text: String,
    pub font_size: FontSizeDef,
    pub bound: Option<Vec<ValueExpr>>,
    pub color: Option<String>,
    pub stroke: Option<StrokeDef>,
    pub font_families: Option<Vec<String>>,
    pub font_style: Option<String>,
    pub text_align: Option<String>,
    #[serde(default)]
    pub bbcode: bool,
    #[serde(default)]
    pub scope: Scope,
    #[serde(rename = "if")]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StrokeDef {
    pub color: String,
    pub width: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CanvasElementDef {
    pub size: ValueExpr,
    pub background: Option<String>,
    pub layers: Vec<LayerDef>,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub operations: Vec<OperationDef>,
    #[serde(rename = "if")]
    pub condition: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    #[default]
    Static,
    Dynamic,
}

// ── FontSize ──

#[derive(Debug, Clone)]
pub enum FontSizeDef {
    Fixed(ValueExpr),
    Range(f64, f64),
}

impl<'de> Deserialize<'de> for FontSizeDef {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FontSizeVisitor;
        impl<'de> Visitor<'de> for FontSizeVisitor {
            type Value = FontSizeDef;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a number, string expression, or [min, max] array")
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<FontSizeDef, E> {
                Ok(FontSizeDef::Fixed(ValueExpr::Number(v as f64)))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<FontSizeDef, E> {
                Ok(FontSizeDef::Fixed(ValueExpr::Number(v as f64)))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> Result<FontSizeDef, E> {
                Ok(FontSizeDef::Fixed(ValueExpr::Number(v)))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<FontSizeDef, E> {
                Ok(FontSizeDef::Fixed(ValueExpr::String(v.to_string())))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<FontSizeDef, A::Error> {
                let min: f64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &"2 elements"))?;
                let max: f64 = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &"2 elements"))?;
                Ok(FontSizeDef::Range(min, max))
            }
        }
        deserializer.deserialize_any(FontSizeVisitor)
    }
}

// ── Canvas ──

#[derive(Debug, Clone)]
pub struct CanvasDef {
    pub size: ValueExpr,
    pub background: Option<String>,
}

impl<'de> Deserialize<'de> for CanvasDef {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CanvasVisitor;
        impl<'de> Visitor<'de> for CanvasVisitor {
            type Value = CanvasDef;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "an array [w, h], a string expression, or {{size, background}}"
                )
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<CanvasDef, E> {
                Ok(CanvasDef {
                    size: ValueExpr::String(v.to_string()),
                    background: None,
                })
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<CanvasDef, A::Error> {
                let w: ValueExpr = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &"2 elements"))?;
                let h: ValueExpr = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &"2 elements"))?;
                Ok(CanvasDef {
                    size: ValueExpr::Array(vec![w, h]),
                    background: None,
                })
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<CanvasDef, A::Error> {
                let mut size = None;
                let mut background = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "size" => size = Some(map.next_value::<ValueExpr>()?),
                        "background" => background = Some(map.next_value::<String>()?),
                        _ => {
                            let _ = map.next_value::<yaml_serde::Value>()?;
                        }
                    }
                }
                Ok(CanvasDef {
                    size: size.ok_or_else(|| de::Error::missing_field("size"))?,
                    background,
                })
            }
        }
        deserializer.deserialize_any(CanvasVisitor)
    }
}

// ── Layers ──

#[derive(Debug, Clone, Deserialize)]
pub struct LayerDef {
    #[serde(rename = "use")]
    pub use_ref: String,
    #[serde(default)]
    pub position: Option<ValueExpr>,
    pub opacity: Option<ValueExpr>,
    #[serde(rename = "if")]
    pub condition: Option<String>,
}

// ── Operations ──

#[derive(Debug, Clone)]
pub struct OperationDef {
    pub name: String,
    pub args: Option<yaml_serde::Value>,
    pub condition: Option<String>,
}

impl<'de> Deserialize<'de> for OperationDef {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct OpVisitor;
        impl<'de> Visitor<'de> for OpVisitor {
            type Value = OperationDef;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    f,
                    "an operation: string, single-key map, or {{op, args?, if?}}"
                )
            }
            // bare string: "square"
            fn visit_str<E: de::Error>(self, v: &str) -> Result<OperationDef, E> {
                Ok(OperationDef {
                    name: v.to_string(),
                    args: None,
                    condition: None,
                })
            }
            // map: either {op: ..., args: ..., if: ...} or {resize_exact: [...]}
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<OperationDef, A::Error> {
                let mut entries: Vec<(String, yaml_serde::Value)> = Vec::new();
                while let Some((k, v)) = map.next_entry::<String, yaml_serde::Value>()? {
                    entries.push((k, v));
                }
                // Check if it's a conditional form (has "op" key)
                let has_op = entries.iter().any(|(k, _)| k == "op");
                if has_op {
                    let mut name = None;
                    let mut args = None;
                    let mut condition = None;
                    for (k, v) in entries {
                        match k.as_str() {
                            "op" => {
                                name = Some(
                                    v.as_str()
                                        .ok_or_else(|| de::Error::custom("'op' must be a string"))?
                                        .to_string(),
                                );
                            }
                            "args" => args = Some(v),
                            "if" => {
                                condition = Some(
                                    v.as_str()
                                        .ok_or_else(|| de::Error::custom("'if' must be a string"))?
                                        .to_string(),
                                );
                            }
                            _ => {}
                        }
                    }
                    Ok(OperationDef {
                        name: name.ok_or_else(|| de::Error::missing_field("op"))?,
                        args,
                        condition,
                    })
                } else if entries.len() == 1 {
                    let (name, args) = entries.into_iter().next().unwrap();
                    Ok(OperationDef {
                        name,
                        args: Some(args),
                        condition: None,
                    })
                } else {
                    Err(de::Error::custom(
                        "operation map must have 'op' key or exactly one key",
                    ))
                }
            }
        }
        deserializer.deserialize_any(OpVisitor)
    }
}

// ── ValueExpr: a value that might contain expressions ──

#[derive(Debug, Clone)]
pub enum ValueExpr {
    Number(f64),
    Bool(bool),
    String(String),
    Array(Vec<ValueExpr>),
}

impl<'de> Deserialize<'de> for ValueExpr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ValueExprVisitor;
        impl<'de> Visitor<'de> for ValueExprVisitor {
            type Value = ValueExpr;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a number, bool, string, or array")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> Result<ValueExpr, E> {
                Ok(ValueExpr::Bool(v))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> Result<ValueExpr, E> {
                Ok(ValueExpr::Number(v as f64))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> Result<ValueExpr, E> {
                Ok(ValueExpr::Number(v as f64))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> Result<ValueExpr, E> {
                Ok(ValueExpr::Number(v))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> Result<ValueExpr, E> {
                Ok(ValueExpr::String(v.to_string()))
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<ValueExpr, A::Error> {
                let mut items = Vec::new();
                while let Some(item) = seq.next_element::<ValueExpr>()? {
                    items.push(item);
                }
                Ok(ValueExpr::Array(items))
            }
        }
        deserializer.deserialize_any(ValueExprVisitor)
    }
}
