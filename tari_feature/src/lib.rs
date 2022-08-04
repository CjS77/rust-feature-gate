//#![feature(proc_macro_hygiene)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Item, Token};

struct AttrMarker {
    pub not_sign: Option<Token![!]>,
    pub ident: Ident,
}

impl Parse for AttrMarker {
    fn parse(input: ParseStream) -> Result<Self> {
        let not_sign = input.parse::<Token![!]>().ok();
        let ident = input.parse::<Ident>()?;
        Ok(Self { not_sign, ident })
    }
}

#[proc_macro_attribute]
pub fn tari_feature(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {}", attr);
    println!("item: {}", item);
    let attr_name = parse_macro_input!(attr as AttrMarker);
    let content = parse_macro_input!(item as Item);
    let cfg_attr = format_ident!("tari_feature_{}", attr_name.ident);
    let cfg = if attr_name.not_sign.is_none() {
        quote! { #[cfg(#cfg_attr)] }
    } else {
        quote! { #[cfg(not(#cfg_attr))] }
    };
    let rust = quote! {
        #cfg
        #content
    };
    println!("rust: {}", rust);
    rust.into()
}
