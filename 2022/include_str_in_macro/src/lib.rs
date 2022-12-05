extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use std;
use std::fs::File;
use std::io::Read;
use syn;
use syn::parse::{Parse, ParseStream, Result};

#[derive(Debug)]
struct MacroArgs {
    macro_name: String,
    file_name: String,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let macro_name_literal: syn::LitStr = input.parse()?;
        let _: syn::Token!(,) = input.parse()?;
        let file_name_literal: syn::LitStr = input.parse()?;
        Ok(Self {
            macro_name: macro_name_literal.value(),
            file_name: file_name_literal.value(),
        })
    }
}

#[proc_macro]
pub fn include_str_in_macro(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as MacroArgs);

    let cwd = std::env::current_dir().unwrap();

    let file_path = cwd.join(&input.file_name);

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let tokens: proc_macro2::TokenStream = contents.parse().unwrap();

    let macro_name = format_ident!("{}", &input.macro_name);

    let result = quote!(
        #macro_name ! (#tokens)
    );

    TokenStream::from(result)
}

