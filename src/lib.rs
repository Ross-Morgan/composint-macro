extern crate proc_macro;

mod data;
mod field;
mod generate;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use data::Data;
use generate::{generate_struct_definition, generate_struct_impl};

#[proc_macro]
pub fn composite_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Data);

    let struct_definition = generate_struct_definition(&input);
    let impl_definition = generate_struct_impl(&input);

    let stream = quote! {
        #struct_definition
        #impl_definition
    };

    TokenStream::from(stream)
}
