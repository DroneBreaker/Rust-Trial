extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn debug_me(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_fn = parse_macro_input!(item as ItemFn);
    let id = &item_fn.sig.ident;

    item_fn.block.stmts.insert(
        0,
        syn::parse_quote!(println!("Entering function: {}", stringify!(#id))),
    );

    TokenStream::from(quote!(
        #item_fn
    ))
}

