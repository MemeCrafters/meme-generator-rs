use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    ext::IdentExt, punctuated::Punctuated, Data, DeriveInput, Error, Expr, ExprLit, Field, Fields,
    Ident, Lit, Meta, MetaNameValue, Token, Type,
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
        impl crate::meme::MemeOptions for #name {
            fn to_options(&self) -> Vec<crate::meme::MemeOption> {
                Vec::from([
                    #(#options),*
                ])
            }
        }
    };

    let default_values = default_value_tokens(&options);
    let default_impl = quote! {
        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#default_values),*
                }
            }
        }
    };

    let fields = field_tokens(&options);
    let wrapper_name = Ident::new(&format!("{}Wrapper", name), name.span());
    let struct_wrapper = quote! {
        #[derive(serde::Deserialize)]
        #[serde(default)]
        struct #wrapper_name {
            #(#fields),*
        }
    };

    let default_impl_wrapper = quote! {
        impl Default for #wrapper_name {
            fn default() -> Self {
                Self {
                    #(#default_values),*
                }
            }
        }
    };

    let checkers = checker_tokens(&options);
    let setters = setter_tokens(&options);
    let deserialize_impl = quote! {
        impl<'de> serde::Deserialize<'de> for #name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                let wrapper = #wrapper_name::deserialize(deserializer)?;
                #(#checkers)*
                Ok(Self {
                    #(#setters),*
                })
            }
        }
    };

    let expanded = quote! {
        #meme_options_impl
        #default_impl
        #struct_wrapper
        #default_impl_wrapper
        #deserialize_impl
    };

    Ok(TokenStream::from(expanded))
}

fn parse_option(field: &Field) -> Result<MemeOption, Error> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let field_type_string = quote!(#field_type).to_string();
    let field_type_str = field_type_string.as_str();
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
                        if field_type_str != "i32" && field_type_str != "f32" {
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
                        if field_type_str != "i32" && field_type_str != "f32" {
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
                        if field_type_str != "String" {
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

    match field_type_str {
        "bool" => {
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
                field_type: field_type.clone(),
                default,
                description,
                parser_flags,
            })
        }
        "String" => {
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
                field_type: field_type.clone(),
                default,
                choices,
                description,
                parser_flags,
            })
        }
        "i32" => {
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
                field_type: field_type.clone(),
                default,
                minimum,
                maximum,
                description,
                parser_flags,
            })
        }
        "f32" => {
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
                field_type: field_type.clone(),
                default,
                minimum,
                maximum,
                description,
                parser_flags,
            })
        }
        _ => Err(Error::new_spanned(field, "Unsupported field type")),
    }
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
        field_type: Type,
        default: Option<bool>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    String {
        field_name: Ident,
        field_type: Type,
        default: Option<String>,
        choices: Option<Vec<String>>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    Integer {
        field_name: Ident,
        field_type: Type,
        default: Option<i32>,
        minimum: Option<i32>,
        maximum: Option<i32>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
    Float {
        field_name: Ident,
        field_type: Type,
        default: Option<f32>,
        minimum: Option<f32>,
        maximum: Option<f32>,
        description: Option<String>,
        parser_flags: ParserFlags,
    },
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

impl ToTokens for MemeOption {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            MemeOption::Boolean {
                field_name,
                field_type: _,
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
                let field_name_str = field_name.unraw().to_string();
                tokens.extend(quote! {
                    crate::meme::MemeOption::Boolean {
                        name: #field_name_str.to_string(),
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
                field_type: _,
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
                let field_name_str = field_name.unraw().to_string();
                tokens.extend(quote! {
                    crate::meme::MemeOption::String {
                        name: #field_name_str.to_string(),
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
                field_type: _,
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
                let field_name_str = field_name.unraw().to_string();
                tokens.extend(quote! {
                    crate::meme::MemeOption::Integer {
                        name: #field_name_str.to_string(),
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
                field_type: _,
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
                let field_name_str = field_name.unraw().to_string();
                tokens.extend(quote! {
                    crate::meme::MemeOption::Float {
                        name: #field_name_str.to_string(),
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

fn default_value_tokens(options: &Vec<MemeOption>) -> Vec<proc_macro2::TokenStream> {
    options
        .iter()
        .map(|option| {
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
        })
        .collect::<Vec<_>>()
}

fn field_tokens(options: &Vec<MemeOption>) -> Vec<proc_macro2::TokenStream> {
    options
        .iter()
        .map(|option| {
            if let MemeOption::Boolean {
                field_name,
                field_type,
                ..
            } = option
            {
                quote! {#field_name: #field_type}
            } else if let MemeOption::String {
                field_name,
                field_type,
                ..
            } = option
            {
                quote! {#field_name: #field_type}
            } else if let MemeOption::Integer {
                field_name,
                field_type,
                ..
            } = option
            {
                quote! {#field_name: #field_type}
            } else if let MemeOption::Float {
                field_name,
                field_type,
                ..
            } = option
            {
                quote! {#field_name: #field_type}
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<_>>()
}

fn checker_tokens(options: &Vec<MemeOption>) -> Vec<proc_macro2::TokenStream> {
    options
        .iter()
        .map(|option| {
            if let MemeOption::String {
                field_name,
                choices,
                ..
            } = option
            {
                if let Some(choices) = choices {
                    let choices = choices.iter().map(|choice| quote!(#choice));
                    return quote! {
                        if !Vec::from([#(#choices),*]).contains(&wrapper.#field_name.as_str()) {
                            return Err(serde::de::Error::custom(format!(
                                "Invalid value for {}: {}",
                                stringify!(#field_name),
                                wrapper.#field_name
                            )));
                        }
                    };
                }
            } else if let MemeOption::Integer {
                field_name,
                minimum,
                maximum,
                ..
            } = option
            {
                if let Some(minimum) = minimum {
                    if let Some(maximum) = maximum {
                        return quote! {
                            if wrapper.#field_name < #minimum || wrapper.#field_name > #maximum {
                                return Err(serde::de::Error::custom(format!(
                                    "Value for {} must be between {} and {}",
                                    stringify!(#field_name),
                                    #minimum,
                                    #maximum
                                )));
                            }
                        };
                    } else {
                        return quote! {
                            if wrapper.#field_name < #minimum {
                                return Err(serde::de::Error::custom(format!(
                                    "Value for {} must be greater than or equal to {}",
                                    stringify!(#field_name),
                                    #minimum
                                )));
                            }
                        };
                    }
                }
                if let Some(maximum) = maximum {
                    return quote! {
                        if wrapper.#field_name > #maximum {
                            return Err(serde::de::Error::custom(format!(
                                "Value for {} must be less than or equal to {}",
                                stringify!(#field_name),
                                #maximum
                            )));
                        }
                    };
                }
            } else if let MemeOption::Float {
                field_name,
                minimum,
                maximum,
                ..
            } = option
            {
                if let Some(minimum) = minimum {
                    if let Some(maximum) = maximum {
                        return quote! {
                            if wrapper.#field_name < #minimum || wrapper.#field_name > #maximum {
                                return Err(serde::de::Error::custom(format!(
                                    "Value for {} must be between {} and {}",
                                    stringify!(#field_name),
                                    #minimum,
                                    #maximum
                                )));
                            }
                        };
                    } else {
                        return quote! {
                            if wrapper.#field_name < #minimum {
                                return Err(serde::de::Error::custom(format!(
                                    "Value for {} must be greater than or equal to {}",
                                    stringify!(#field_name),
                                    #minimum
                                )));
                            }
                        };
                    }
                }
                if let Some(maximum) = maximum {
                    return quote! {
                        if wrapper.#field_name > #maximum {
                            return Err(serde::de::Error::custom(format!(
                                "Value for {} must be less than or equal to {}",
                                stringify!(#field_name),
                                #maximum
                            )));
                        }
                    };
                }
            }
            quote! {}
        })
        .collect::<Vec<_>>()
}

fn setter_tokens(options: &Vec<MemeOption>) -> Vec<proc_macro2::TokenStream> {
    options
        .iter()
        .map(|option| {
            if let MemeOption::Boolean { field_name, .. } = option {
                quote! {#field_name: wrapper.#field_name}
            } else if let MemeOption::String { field_name, .. } = option {
                quote! {#field_name: wrapper.#field_name}
            } else if let MemeOption::Integer { field_name, .. } = option {
                quote! {#field_name: wrapper.#field_name}
            } else if let MemeOption::Float { field_name, .. } = option {
                quote! {#field_name: wrapper.#field_name}
            } else {
                unreachable!()
            }
        })
        .collect::<Vec<_>>()
}
