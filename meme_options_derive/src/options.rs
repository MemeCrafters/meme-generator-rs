use proc_macro::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated, Data, DeriveInput, Error, Expr, Field, Fields, Lit, Meta,
    MetaNameValue, Token,
};

pub fn derive_options(input: &DeriveInput) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let options = if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            fields
                .named
                .iter()
                .map(parse_option)
                .collect::<Result<Vec<_>, Error>>()?
        } else {
            return Err(Error::new_spanned(&input, "Unsupported fields"));
        }
    } else {
        return Err(Error::new_spanned(&input, "Unsupported data type"));
    };

    let expanded = quote! {
        impl IntoMemeOptions for #name {
            fn into_options(&self) -> Vec<MemeOption> {
                Vec::from([
                    #(#options),*
                ])
            }
        }
    };

    Ok(TokenStream::from(expanded))
}

enum ArgType {
    String,
    Integer,
    Float,
    Boolean,
}

fn parse_option(field: &Field) -> Result<proc_macro2::TokenStream, syn::Error> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
    let arg_type = match quote!(#field_type).to_string().as_str() {
        "String" => ArgType::String,
        "i32" => ArgType::Integer,
        "f32" => ArgType::Float,
        "bool" => ArgType::Boolean,
        _ => return Err(Error::new_spanned(field, "Unsupported field type")),
    };

    let mut default = quote!(None);
    let mut maximum = quote!(None);
    let mut minimum = quote!(None);
    let mut choices = quote!(None);
    let mut description = quote!(None);

    let mut short = quote!(false);
    let mut long = quote!(false);
    let mut short_aliases = quote!(None);
    let mut long_aliases = quote!(None);

    for attr in &field.attrs {
        if !(attr.path().is_ident("option") || attr.path().is_ident("doc")) {
            continue;
        }
        if attr.path().is_ident("doc") {
            match &attr.meta {
                Meta::NameValue(MetaNameValue {
                    value:
                        syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }),
                    ..
                }) => {
                    description = quote!(Some(#s.trim().to_string()));
                }
                _ => {}
            }
            continue;
        }
        for attr in attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)? {
            match attr {
                Meta::Path(path) => {
                    if path.is_ident("short") {
                        short = quote!(true);
                    } else if path.is_ident("long") {
                        long = quote!(true);
                    }
                }
                Meta::NameValue(MetaNameValue { path, value, .. }) => {
                    if path.is_ident("default") {
                        default = parse_value(&value, &arg_type)?;
                    } else if path.is_ident("maximum") {
                        maximum = match arg_type {
                            ArgType::Integer => parse_value(&value, &arg_type)?,
                            ArgType::Float => parse_value(&value, &arg_type)?,
                            _ => {
                                return Err(Error::new_spanned(
                                    path,
                                    "Maximum value is not supported for this type",
                                ))
                            }
                        };
                    } else if path.is_ident("minimum") {
                        minimum = match arg_type {
                            ArgType::Integer => parse_value(&value, &arg_type)?,
                            ArgType::Float => parse_value(&value, &arg_type)?,
                            _ => {
                                return Err(Error::new_spanned(
                                    path,
                                    "Minimum value is not supported for this type",
                                ))
                            }
                        };
                    } else if path.is_ident("choices") {
                        choices = match arg_type {
                            ArgType::String => parse_string_array(&value)?,
                            _ => {
                                return Err(Error::new_spanned(
                                    path,
                                    "Choices are not supported for this type",
                                ))
                            }
                        };
                    } else if path.is_ident("short_aliases") {
                        short_aliases = parse_char_array(&value)?;
                    } else if path.is_ident("long_aliases") {
                        long_aliases = parse_string_array(&value)?;
                    }
                }
                _ => return Err(Error::new_spanned(attr, "Unsupported attribute format")),
            }
        }
    }

    let parser_flags = quote! {
        ParserFlags {
            short: #short,
            long: #long,
            short_aliases: #short_aliases,
            long_aliases: #long_aliases,
        }
    };

    match arg_type {
        ArgType::Boolean => Ok(quote! {
            MemeOption::Boolean {
                name: stringify!(#field_name).to_string(),
                default: #default,
                description: #description,
                parser_flags: #parser_flags,
            }
        }),
        ArgType::String => Ok(quote! {
            MemeOption::String {
                name: stringify!(#field_name).to_string(),
                default: #default,
                choices: #choices,
                description: #description,
                parser_flags: #parser_flags,
            }
        }),
        ArgType::Integer => Ok(quote! {
            MemeOption::Integer {
                name: stringify!(#field_name).to_string(),
                default: #default,
                maximum: #maximum,
                minimum: #minimum,
                description: #description,
                parser_flags: #parser_flags,
            }
        }),
        ArgType::Float => Ok(quote! {
            MemeOption::Float {
                name: stringify!(#field_name).to_string(),
                default: #default,
                maximum: #maximum,
                minimum: #minimum,
                description: #description,
                parser_flags: #parser_flags,
            }
        }),
    }
}

fn parse_value(expr: &Expr, arg_type: &ArgType) -> Result<proc_macro2::TokenStream, Error> {
    match arg_type {
        ArgType::String => parse_string(expr),
        ArgType::Integer => parse_integer(expr),
        ArgType::Float => parse_float(expr),
        ArgType::Boolean => parse_boolean(expr),
    }
}

fn parse_string(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Str(s) => Ok(quote!(Some(#s.to_string()))),
            _ => Err(Error::new_spanned(lit, "Expected string")),
        },
        _ => Ok(quote!(Some(#expr))),
    }
}

fn parse_integer(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Int(i) => Ok(quote!(Some(#i))),
            _ => Err(Error::new_spanned(lit, "Expected integer")),
        },
        _ => Ok(quote!(Some(#expr))),
    }
}

fn parse_float(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Float(f) => Ok(quote!(Some(#f))),
            _ => Err(Error::new_spanned(lit, "Expected float")),
        },
        _ => Ok(quote!(Some(#expr))),
    }
}

fn parse_boolean(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Lit(lit) => match &lit.lit {
            Lit::Bool(b) => Ok(quote!(Some(#b))),
            _ => Err(Error::new_spanned(lit, "Expected boolean")),
        },
        _ => Ok(quote!(Some(#expr))),
    }
}

fn parse_string_array(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Array(array) => {
            let values = array
                .elems
                .iter()
                .map(|expr| {
                    Ok(match expr {
                        Expr::Lit(lit) => match &lit.lit {
                            Lit::Str(s) => quote!(String::from(#s)),
                            _ => return Err(Error::new_spanned(lit, "Expected string")),
                        },
                        _ => quote!(#expr),
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            Ok(quote!(Some(Vec::from([#(#values),*]))))
        }
        _ => Ok(quote!(Some(#expr))),
    }
}

fn parse_char_array(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Array(array) => {
            let values = array
                .elems
                .iter()
                .map(|expr| {
                    Ok(match expr {
                        Expr::Lit(lit) => match &lit.lit {
                            Lit::Char(c) => quote!(#c),
                            _ => return Err(Error::new_spanned(lit, "Expected char")),
                        },
                        _ => quote!(#expr),
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            Ok(quote!(Some(Vec::from([#(#values),*]))))
        }
        _ => Ok(quote!(Some(#expr))),
    }
}
