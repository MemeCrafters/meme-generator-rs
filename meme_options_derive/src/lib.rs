extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod options;

#[proc_macro_derive(MemeOptions, attributes(option))]
pub fn derive_meme_options(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);
    options::derive_options(&input)
        .unwrap_or_else(|err| TokenStream::from(syn::Error::into_compile_error(err)))
        .into()
}
