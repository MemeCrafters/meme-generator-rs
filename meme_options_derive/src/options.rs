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
                .map(generate_option)
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

fn generate_option(field: &Field) -> Result<proc_macro2::TokenStream, syn::Error> {
    let field_name = field.ident.as_ref().unwrap();
    let field_type = &field.ty;
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
                        default = parse_value(&value)?;
                    } else if path.is_ident("maximum") {
                        maximum = parse_value(&value)?;
                    } else if path.is_ident("minimum") {
                        minimum = parse_value(&value)?;
                    } else if path.is_ident("choices") {
                        choices = parse_choices(&value)?;
                    } else if path.is_ident("short_aliases") {
                        short_aliases = parse_short_aliases(&value)?;
                    } else if path.is_ident("long_aliases") {
                        long_aliases = parse_long_aliases(&value)?;
                    }
                }
                _ => return Err(Error::new_spanned(attr, "Unsupported attribute format")),
            }
        }
    }

    let arg_type = match quote!(#field_type).to_string().as_str() {
        "String" => quote!(ArgType::String),
        "i32" => quote!(ArgType::Integer),
        "f32" => quote!(ArgType::Float),
        "bool" => quote!(ArgType::Boolean),
        _ => panic!("Unsupported type"),
    };

    let parser_flags = quote! {
        ParserFlags {
            short: #short,
            long: #long,
            short_aliases: #short_aliases,
            long_aliases: #long_aliases,
        }
    };

    Ok(quote! {
        MemeOption {
            name: stringify!(#field_name).to_string(),
            r#type: #arg_type,
            default: #default,
            maximum: #maximum,
            minimum: #minimum,
            choices: #choices,
            description: #description,
            parser_flags: #parser_flags,
        }
    })
}

fn parse_value(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Lit(lit) => Ok(match &lit.lit {
            Lit::Str(s) => quote!(Some(ArgValue::String(#s.to_string()))),
            Lit::Int(i) => quote!(Some(ArgValue::Integer(#i))),
            Lit::Float(f) => quote!(Some(ArgValue::Float(#f))),
            Lit::Bool(b) => quote!(Some(ArgValue::Boolean(#b))),
            _ => return Err(Error::new_spanned(lit, "Unsupported ArgValue type")),
        }),
        _ => Ok(quote!(Some(#expr))),
    }
}

fn parse_choices(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
    match expr {
        Expr::Array(array) => {
            let values = array
                .elems
                .iter()
                .map(|expr| {
                    Ok(match expr {
                        Expr::Lit(lit) => match &lit.lit {
                            Lit::Str(s) => quote!(ArgValue::String(#s.to_string())),
                            Lit::Int(i) => quote!(ArgValue::Integer(#i)),
                            Lit::Float(f) => quote!(ArgValue::Float(#f)),
                            Lit::Bool(b) => quote!(ArgValue::Boolean(#b)),
                            _ => return Err(Error::new_spanned(lit, "Unsupported ArgValue type")),
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

fn parse_long_aliases(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
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

fn parse_short_aliases(expr: &Expr) -> Result<proc_macro2::TokenStream, Error> {
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
