//! Template loading, discovery, and Meme trait implementation.

use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    sync::LazyLock,
};

use chrono::{DateTime, Local, NaiveDate, TimeZone};
use rand::seq::IndexedRandom;
use tracing::{info, warn};

use meme_generator_core::{
    config::MEME_HOME,
    error::Error,
    meme::{self, Meme, MemeInfo, MemeOption, MemeParams, MemeShortcut, OptionValue, ParserFlags},
    registry::MemeRegistry,
};
use meme_generator_utils::{builder::InputImage, encoder::encode_png, tools::GRID_PATTERN_IMAGE};

use crate::{
    executor::execute_template,
    schema::{OptionDef, OptionType, ShortcutDef, Template},
};

pub static TEMPLATES_DIR: LazyLock<PathBuf> = LazyLock::new(|| MEME_HOME.join("templates"));

// ── TemplateMeme: implements Meme trait ──

pub struct TemplateMeme {
    template: Template,
}

impl TemplateMeme {
    pub fn new(template: Template) -> Self {
        Self { template }
    }

    fn build_info(&self) -> MemeInfo {
        let meta = &self.template.metadata;
        let params = &meta.params;

        let options = params
            .options
            .iter()
            .map(|o| convert_option_def(o))
            .collect();

        let shortcuts = meta
            .shortcuts
            .iter()
            .map(|s| convert_shortcut_def(s))
            .collect();

        MemeInfo {
            key: meta.key.clone(),
            params: MemeParams {
                min_images: params.min_images,
                max_images: params.max_images,
                min_texts: params.min_texts,
                max_texts: params.max_texts,
                default_texts: params.default_texts.clone(),
                options,
            },
            keywords: meta.keywords.clone(),
            shortcuts,
            tags: meta.tags.iter().cloned().collect::<HashSet<_>>(),
            date_created: parse_date(&meta.date_created),
            date_modified: parse_date(&meta.date_modified),
        }
    }
}

impl Meme for TemplateMeme {
    fn key(&self) -> String {
        self.template.metadata.key.clone()
    }

    fn info(&self) -> MemeInfo {
        self.build_info()
    }

    fn generate(
        &self,
        images: Vec<meme::Image>,
        texts: Vec<String>,
        options: HashMap<String, OptionValue>,
    ) -> Result<Vec<u8>, Error> {
        let info = self.info();
        let params = &info.params;

        // Validate image count
        if images.len() < params.min_images as usize || images.len() > params.max_images as usize {
            return Err(Error::ImageNumberMismatch(
                params.min_images,
                params.max_images,
                images.len() as u8,
            ));
        }

        // Validate text count
        if texts.len() < params.min_texts as usize || texts.len() > params.max_texts as usize {
            return Err(Error::TextNumberMismatch(
                params.min_texts,
                params.max_texts,
                texts.len() as u8,
            ));
        }

        // Apply default options
        let options = apply_default_options(&params.options, options);

        // Apply default_texts for missing entries
        let texts = apply_default_texts(texts, &params.default_texts);

        // Decode input images
        let input_images = images
            .into_iter()
            .map(InputImage::from)
            .collect::<Result<Vec<_>, _>>()?;

        execute_template(&self.template, input_images, texts, &options)
    }

    fn generate_preview(&self, options: HashMap<String, OptionValue>) -> Result<Vec<u8>, Error> {
        let info = self.info();
        let params = &info.params;
        let mut images = Vec::new();
        if params.min_images > 0 {
            let image_data = encode_png(GRID_PATTERN_IMAGE.clone())?;
            for i in 0..params.min_images {
                let name = if params.min_images == 1 {
                    "{name}".to_string()
                } else {
                    format!("{{name{}}}", i + 1)
                };
                images.push(meme::Image {
                    name,
                    data: image_data.clone(),
                });
            }
        }
        let texts = if params.default_texts.len() >= params.min_texts as usize
            && params.default_texts.len() <= params.max_texts as usize
        {
            params.default_texts.clone()
        } else {
            let mut texts = Vec::new();
            for i in 0..params.min_texts {
                let text = if params.min_texts == 1 {
                    "{text}".to_string()
                } else {
                    format!("{{text{}}}", i + 1)
                };
                texts.push(text);
            }
            texts
        };
        self.generate(images, texts, options)
    }
}

unsafe impl Send for TemplateMeme {}
unsafe impl Sync for TemplateMeme {}

// ── Conversion helpers ──

fn convert_option_def(opt: &OptionDef) -> MemeOption {
    let parser_flags = ParserFlags {
        short: opt.parser_flags.short,
        long: opt.parser_flags.long,
        short_aliases: opt
            .parser_flags
            .short_aliases
            .iter()
            .filter_map(|s| s.chars().next())
            .collect(),
        long_aliases: opt.parser_flags.long_aliases.clone(),
    };

    match opt.option_type {
        OptionType::Boolean => MemeOption::Boolean {
            name: opt.name.clone(),
            default: opt.default.as_ref().and_then(|v| v.as_bool()),
            description: opt.description.clone(),
            parser_flags,
        },
        OptionType::String => MemeOption::String {
            name: opt.name.clone(),
            default: opt
                .default
                .as_ref()
                .and_then(|v| v.as_str().map(|s| s.to_string())),
            choices: opt.choices.clone(),
            description: opt.description.clone(),
            parser_flags,
        },
        OptionType::Integer => MemeOption::Integer {
            name: opt.name.clone(),
            default: opt
                .default
                .as_ref()
                .and_then(|v| v.as_i64().map(|n| n as i32)),
            minimum: opt.minimum.map(|n| n as i32),
            maximum: opt.maximum.map(|n| n as i32),
            description: opt.description.clone(),
            parser_flags,
        },
        OptionType::Float => MemeOption::Float {
            name: opt.name.clone(),
            default: opt
                .default
                .as_ref()
                .and_then(|v| v.as_f64().map(|n| n as f32)),
            minimum: opt.minimum.map(|n| n as f32),
            maximum: opt.maximum.map(|n| n as f32),
            description: opt.description.clone(),
            parser_flags,
        },
    }
}

fn convert_shortcut_def(s: &ShortcutDef) -> MemeShortcut {
    let mut options = HashMap::new();
    for (k, v) in &s.options {
        let val = match v {
            yaml_serde::Value::Bool(b) => OptionValue::Boolean(*b),
            yaml_serde::Value::String(s) => OptionValue::String(s.clone()),
            yaml_serde::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    OptionValue::Integer(i as i32)
                } else if let Some(f) = n.as_f64() {
                    OptionValue::Float(f as f32)
                } else {
                    continue;
                }
            }
            _ => continue,
        };
        options.insert(k.clone(), val);
    }

    MemeShortcut {
        pattern: s.pattern.clone(),
        humanized: s.humanized.clone(),
        names: s.names.clone(),
        texts: s.texts.clone(),
        options,
    }
}

fn apply_default_options(
    defs: &[MemeOption],
    mut provided: HashMap<String, OptionValue>,
) -> HashMap<String, OptionValue> {
    let mut rng = rand::rng();
    for def in defs {
        let name = match def {
            MemeOption::Boolean { name, .. }
            | MemeOption::String { name, .. }
            | MemeOption::Integer { name, .. }
            | MemeOption::Float { name, .. } => name,
        };
        if provided.contains_key(name) {
            continue;
        }

        match def {
            MemeOption::Boolean { default, .. } => {
                if let Some(v) = default {
                    provided.insert(name.clone(), OptionValue::Boolean(*v));
                }
            }
            MemeOption::String {
                default, choices, ..
            } => {
                if let Some(v) = default {
                    provided.insert(name.clone(), OptionValue::String(v.clone()));
                } else if let Some(choices) = choices {
                    if let Some(v) = choices.choose(&mut rng) {
                        provided.insert(name.clone(), OptionValue::String(v.clone()));
                    }
                }
            }
            MemeOption::Integer {
                default,
                minimum,
                maximum,
                ..
            } => {
                if let Some(v) = default {
                    provided.insert(name.clone(), OptionValue::Integer(*v));
                } else if let (Some(min), Some(max)) = (minimum, maximum) {
                    let v = rand::random_range(*min..=*max);
                    provided.insert(name.clone(), OptionValue::Integer(v));
                }
            }
            MemeOption::Float {
                default,
                minimum,
                maximum,
                ..
            } => {
                if let Some(v) = default {
                    provided.insert(name.clone(), OptionValue::Float(*v));
                } else if let (Some(min), Some(max)) = (minimum, maximum) {
                    let v = rand::random_range(*min..=*max);
                    provided.insert(name.clone(), OptionValue::Float(v));
                }
            }
        }
    }
    provided
}

fn apply_default_texts(mut texts: Vec<String>, defaults: &[String]) -> Vec<String> {
    for i in texts.len()..defaults.len() {
        texts.push(defaults[i].clone());
    }
    texts
}

fn parse_date(s: &str) -> DateTime<Local> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .ok()
        .and_then(|d| {
            d.and_hms_opt(0, 0, 0)
                .and_then(|dt| Local.from_local_datetime(&dt).single())
        })
        .unwrap_or_else(Local::now)
}

// ── Template loading ──

pub fn load_templates(registry: &mut dyn MemeRegistry) {
    let templates_dir = TEMPLATES_DIR.as_path();
    if !templates_dir.exists() {
        info!("Templates directory not found: {}", templates_dir.display());
        return;
    }

    let entries = match templates_dir.read_dir() {
        Ok(entries) => entries,
        Err(err) => {
            warn!("Failed to read templates directory: {err}");
            return;
        }
    };

    let mut count = 0;
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                warn!("Failed to read directory entry: {err}");
                continue;
            }
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !["yaml", "yml"].contains(&ext) {
            continue;
        }

        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(err) => {
                warn!(
                    "Failed to read template file {:?}: {err}",
                    entry.file_name()
                );
                continue;
            }
        };

        let template: Template = match yaml_serde::from_str(&content) {
            Ok(t) => t,
            Err(err) => {
                warn!("Failed to parse template {:?}: {err}", entry.file_name());
                continue;
            }
        };

        if template.version != "1.0" {
            warn!(
                "Unsupported template version {:?} in {:?}, expected \"1.0\"",
                template.version,
                entry.file_name()
            );
            continue;
        }

        let key = template.metadata.key.clone();
        let meme = Box::new(TemplateMeme::new(template));
        registry.register_meme(&key, meme);
        count += 1;
    }

    info!(
        "Loaded {count} template memes from {}",
        templates_dir.display()
    );
}
