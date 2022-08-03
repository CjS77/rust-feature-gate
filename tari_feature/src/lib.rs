//#![feature(proc_macro_hygiene)]

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Item, Ident};

#[proc_macro_attribute]
pub fn tari_feature(attr: TokenStream, item: TokenStream) -> TokenStream {
    //println!("attr: {}", attr);
    //println!("item: {}", item);
    let attr_name = parse_macro_input!(attr as Ident);
    let content = parse_macro_input!(item as Item);
    let cfg_attr = format_ident!("tari_feature_{}", attr_name);
    let rust = quote! {
        #[cfg(#cfg_attr)]
        #content
    };
    //println!("rust: {}", rust);
    rust.into()
}
