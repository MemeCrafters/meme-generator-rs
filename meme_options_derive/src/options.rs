use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, Data, DeriveInput, Error, Expr, ExprLit, Field, Fields, Ident, Lit,
    Meta, MetaNameValue, Token,
};

pub fn derive_options(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;

    let options = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(|field| Ok(parse_option(field)?))
                .collect::<Result<Vec<_>, Error>>()?
        } else {
            return Err(Error::new_spanned(
                &input,
                "Only named fields are supported",
            ));
        }
    } else {
        return Err(Error::new_spanned(&input, "Only structs are supported"));
    };

    let meme_options_impl = quote! {
        impl crate::meme::ToMemeOptions for #name {
            fn to_options(&self) -> Vec<crate::meme::MemeOption> {
                Vec::from([
                    #(#options),*
                ])
            }
        }
    };

    let default_values = options.iter().map(|option| {
        if let MemeOption::Boolean {
            field_name,
            default,
            ..
        } = option
        {
            let default = default.unwrap_or(false);
            quote! {#field_name: #default}
        } else if let MemeOption::String {
            field_name,
            default,
            ..
        } = option
        {
            let default = default.clone().unwrap_or(String::new());
            quote! {#field_name: #default.to_string()}
        } else if let MemeOption::Integer {
            field_name,
            default,
            ..
        } = option
        {
            let default = default.unwrap_or(0);
            quote! {#field_name: #default}
        } else if let MemeOption::Float {
            field_name,
            default,
            ..
        } = option
        {
            let default = default.unwrap_or(0.0);
            quote! {#field_name: #default}
        } else {
            unreachable!()
        }
    });
    let default_impl = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#default_values),*
                }
            }
        }
    };

    let expanded = quote! {
        #meme_options_impl
        #default_impl
    };

    Ok(TokenStream::from(expanded))
}

fn parse_option(field: &Field) -> Result<MemeOption, Error> {
    let field_name = field.ident.as_ref().unwrap();
    let arg_type = parse_arg_type(field)?;
    let mut description = None;
    let mut parser_flags = ParserFlags::default();
    let mut default_lit = None;
    let mut minimum_lit = None;
    let mut maximum_lit = None;
    let mut choices = None;

    for attr in &field.attrs {
        if !(attr.path().is_ident("option") || attr.path().is_ident("doc")) {
            continue;
        }
        if attr.path().is_ident("doc") {
            match &attr.meta {
                Meta::NameValue(MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(s), ..
                        }),
                    ..
                }) => {
                    description = Some(s.value().trim().to_string());
                }
                _ => {}
            }
            continue;
        }
        for attr in attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)? {
            match attr {
                Meta::Path(path) => {
                    if path.is_ident("short") {
                        parser_flags.short = true;
                    } else if path.is_ident("long") {
                        parser_flags.long = true;
                    }
                }
                Meta::NameValue(MetaNameValue { path, value, .. }) => {
                    if path.is_ident("short_aliases") {
                        parser_flags.short_aliases = parse_char_array(&value)?;
                    } else if path.is_ident("long_aliases") {
                        parser_flags.long_aliases = parse_string_array(&value)?;
                    } else if path.is_ident("default") {
                        match value {
                            Expr::Lit(lit) => default_lit = Some(lit.lit),
                            _ => return Err(Error::new_spanned(value, "Expected literal")),
                        }
                    } else if path.is_ident("minimum") {
                        if arg_type != ArgType::Integer && arg_type != ArgType::Float {
                            return Err(Error::new_spanned(
                                path,
                                "Minimum is only supported for integer and float types",
                            ));
                        }
                        match value {
                            Expr::Lit(lit) => minimum_lit = Some(lit.lit),
                            _ => return Err(Error::new_spanned(value, "Expected literal")),
                        }
                    } else if path.is_ident("maximum") {
                        if arg_type != ArgType::Integer && arg_type != ArgType::Float {
                            return Err(Error::new_spanned(
                                path,
                                "Maximum is only supported for integer and float types",
                            ));
                        }
                        match value {
                            Expr::Lit(lit) => maximum_lit = Some(lit.lit),
                            _ => return Err(Error::new_spanned(value, "Expected literal")),
                        }
                    } else if path.is_ident("choices") {
                        if arg_type != ArgType::String {
                            return Err(Error::new_spanned(
                                path,
                                "Choices are only supported for string types",
                            ));
                        }
                        choices = Some(parse_string_array(&value)?);
                    }
                }
                _ => return Err(Error::new_spanned(attr, "Unsupported attribute format")),
            }
        }
    }

    match arg_type {
        ArgType::Boolean => {
            let mut default = Some(false);
            if let Some(lit) = default_lit {
                match &lit {
                    Lit::Bool(b) => {
                        default = Some(b.value);
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected boolean")),
                }
            }
            Ok(MemeOption::Boolean {
                field_name: field_name.clone(),
                default,
                description,
                parser_flags,
            })
        }
        ArgType::String => {
            let mut default = Some(String::new());
            if let Some(lit) = default_lit {
                match &lit {
                    Lit::Str(s) => {
                        default = Some(s.value());
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected string")),
                }
            }
            Ok(MemeOption::String {
                field_name: field_name.clone(),
                default,
                choices,
                description,
                parser_flags,
            })
        }
        ArgType::Integer => {
            let mut default = Some(0);
            if let Some(lit) = default_lit {
                match &lit {
                    Lit::Int(i) => {
                        default = Some(i.base10_parse()?);
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected integer")),
                }
            }
            let mut minimum = None;
            if let Some(lit) = minimum_lit {
                match &lit {
                    Lit::Int(i) => {
                        minimum = Some(i.base10_parse()?);
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected integer")),
                }
            }
            let mut maximum = None;
            if let Some(lit) = maximum_lit {
                match &lit {
                    Lit::Int(i) => {
                        maximum = Some(i.base10_parse()?);
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected integer")),
                }
            }
            Ok(MemeOption::Integer {
                field_name: field_name.clone(),
                default,
                minimum,
                maximum,
                description,
                parser_flags,
            })
        }
        ArgType::Float => {
            let mut default = Some(0.0);
            if let Some(lit) = default_lit {
                match &lit {
                    Lit::Float(f) => {
                        default = Some(f.base10_parse()?);
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected float")),
                }
            }
            let mut minimum = None;
            if let Some(lit) = minimum_lit {
                match &lit {
                    Lit::Float(f) => {
                        minimum = Some(f.base10_parse()?);
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected float")),
                }
            }
            let mut maximum = None;
            if let Some(lit) = maximum_lit {
                match &lit {
                    Lit::Float(f) => {
                        maximum = Some(f.base10_parse()?);
                    }
                    _ => return Err(Error::new_spanned(lit, "Expected float")),
                }
            }
            Ok(MemeOption::Float {
                field_name: field_name.clone(),
                default,
                minimum,
                maximum,
                description,
                parser_flags,
            })
        }
    }
}

#[derive(PartialEq)]
enum ArgType {
    String,
    Integer,
    Float,
    Boolean,
}

struct ParserFlags {
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

enum MemeOption {
    Boolean {
        field_name: Ident,
        default: Option<bool>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    String {
        field_name: Ident,
        default: Option<String>,
        choices: Option<Vec<String>>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    Integer {
        field_name: Ident,
        default: Option<i32>,
        minimum: Option<i32>,
        maximum: Option<i32>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    Float {
        field_name: Ident,
        default: Option<f32>,
        minimum: Option<f32>,
        maximum: Option<f32>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
}

impl ToTokens for MemeOption {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            MemeOption::Boolean {
                field_name,
                default,
                description,
                parser_flags:
                    ParserFlags {
                        short,
                        long,
                        short_aliases,
                        long_aliases,
                    },
            } => {
                let default = match default {
                    Some(default) => quote!(Some(#default)),
                    None => quote!(None),
                };
                let description = match description {
                    Some(description) => quote!(Some(#description.to_string())),
                    None => quote!(None),
                };
                tokens.extend(quote! {
                    crate::meme::MemeOption::Boolean {
                        name: stringify!(#field_name).to_string(),
                        default: #default,
                        description: #description,
                        parser_flags: crate::meme::ParserFlags {
                            short: #short,
                            long: #long,
                            short_aliases: Vec::from([#(#short_aliases),*]),
                            long_aliases: Vec::from([#(#long_aliases.to_string()),*]),
                        },
                    }
                });
            }
            MemeOption::String {
                field_name,
                default,
                choices,
                description,
                parser_flags:
                    ParserFlags {
                        short,
                        long,
                        short_aliases,
                        long_aliases,
                    },
            } => {
                let default = match default {
                    Some(default) => quote!(Some(#default.to_string())),
                    None => quote!(None),
                };
                let description = match description {
                    Some(description) => quote!(Some(#description.to_string())),
                    None => quote!(None),
                };
                let choices = match choices {
                    Some(choices) => quote!(Some(Vec::from([#(#choices.to_string()),*]))),
                    None => quote!(None),
                };
                tokens.extend(quote! {
                    crate::meme::MemeOption::String {
                        name: stringify!(#field_name).to_string(),
                        default: #default,
                        choices: #choices,
                        description: #description,
                        parser_flags: crate::meme::ParserFlags {
                            short: #short,
                            long: #long,
                            short_aliases: Vec::from([#(#short_aliases),*]),
                            long_aliases: Vec::from([#(#long_aliases.to_string()),*]),
                        },
                    }
                });
            }
            MemeOption::Integer {
                field_name,
                default,
                minimum,
                maximum,
                description,
                parser_flags:
                    ParserFlags {
                        short,
                        long,
                        short_aliases,
                        long_aliases,
                    },
            } => {
                let default = match default {
                    Some(default) => quote!(Some(#default)),
                    None => quote!(None),
                };
                let description = match description {
                    Some(description) => quote!(Some(#description.to_string())),
                    None => quote!(None),
                };
                let minimum = match minimum {
                    Some(minimum) => quote!(Some(#minimum)),
                    None => quote!(None),
                };
                let maximum = match maximum {
                    Some(maximum) => quote!(Some(#maximum)),
                    None => quote!(None),
                };
                tokens.extend(quote! {
                    crate::meme::MemeOption::Integer {
                        name: stringify!(#field_name).to_string(),
                        default: #default,
                        minimum: #minimum,
                        maximum: #maximum,
                        description: #description,
                        parser_flags: crate::meme::ParserFlags {
                            short: #short,
                            long: #long,
                            short_aliases: Vec::from([#(#short_aliases),*]),
                            long_aliases: Vec::from([#(#long_aliases.to_string()),*]),
                        },
                    }
                });
            }
            MemeOption::Float {
                field_name,
                default,
                minimum,
                maximum,
                description,
                parser_flags:
                    ParserFlags {
                        short,
                        long,
                        short_aliases,
                        long_aliases,
                    },
            } => {
                let default = match default {
                    Some(default) => quote!(Some(#default)),
                    None => quote!(None),
                };
                let description = match description {
                    Some(description) => quote!(Some(#description.to_string())),
                    None => quote!(None),
                };
                let minimum = match minimum {
                    Some(minimum) => quote!(Some(#minimum)),
                    None => quote!(None),
                };
                let maximum = match maximum {
                    Some(maximum) => quote!(Some(#maximum)),
                    None => quote!(None),
                };
                tokens.extend(quote! {
                    crate::meme::MemeOption::Float {
                        name: stringify!(#field_name).to_string(),
                        default: #default,
                        minimum: #minimum,
                        maximum: #maximum,
                        description: #description,
                        parser_flags: crate::meme::ParserFlags {
                            short: #short,
                            long: #long,
                            short_aliases: Vec::from([#(#short_aliases),*]),
                            long_aliases: Vec::from([#(#long_aliases.to_string()),*]),
                        },
                    }
                });
            }
        }
    }
}

fn parse_arg_type(field: &Field) -> Result<ArgType, Error> {
    let field_type = &field.ty;
    match quote!(#field_type).to_string().as_str() {
        "String" => Ok(ArgType::String),
        "i32" => Ok(ArgType::Integer),
        "f32" => Ok(ArgType::Float),
        "bool" => Ok(ArgType::Boolean),
        _ => Err(Error::new_spanned(field, "Unsupported field type")),
    }
}

fn parse_string_array(expr: &Expr) -> Result<Vec<String>, Error> {
    if let Expr::Array(array) = expr {
        array
            .elems
            .iter()
            .map(|expr| {
                if let Expr::Lit(lit) = expr {
                    if let Lit::Str(s) = &lit.lit {
                        return Ok(s.value());
                    }
                }
                Err(Error::new_spanned(expr, "Expected string"))
            })
            .collect::<Result<Vec<_>, Error>>()
    } else {
        Err(Error::new_spanned(expr, "Expected array"))
    }
}

fn parse_char_array(expr: &Expr) -> Result<Vec<char>, Error> {
    if let Expr::Array(array) = expr {
        array
            .elems
            .iter()
            .map(|expr| {
                if let Expr::Lit(lit) = expr {
                    if let Lit::Char(c) = &lit.lit {
                        return Ok(c.value());
                    }
                }
                Err(Error::new_spanned(expr, "Expected char"))
            })
            .collect::<Result<Vec<_>, Error>>()
    } else {
        Err(Error::new_spanned(expr, "Expected array"))
    }
}
