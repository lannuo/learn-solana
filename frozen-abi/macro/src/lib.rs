extern crate proc_macro;

// This file littered with these essential cfgs so ensure them.
#[cfg(not(any(RUSTC_WITH_SPECIALIZATION, RUSTC_WITHOUT_SPECIALIZATION)))]
compile_error!("rustc version is mission in build dependency and build.rs is not specified");

#[cfg(any(RUSTC_WITHOUT_SPECIALIZATION, RUSTC_WITHOUT_SPECIALIZATION))]
use proc_macro::TokenStream;

// Define dummy macro_attribute and macro_derive for stable rustc

#[cfg(RUSTC_WITHOUT_SPECIALIZATION)]
#[proc_macro_attribute]
pub fn frozen_abi(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[cfg(RUSTC_WITHOUT_SPECIALIZATION)]
#[proc_macro_derive(AbiExample)]
pub fn derive_abi_sample(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

#[cfg(RUSTC_WITHOUT_SPECIALIZATION)]
#[proc_macro_derive(AbiEnumVisitor)]
pub fn derive_abi_enum_visitor(_item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}

#[cfg(RUSTC_WITH_SPECIALIZATION)]
use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
#[cfg(RUSTC_WITH_SPECIALIZATION)]
use quote::{quote, ToTokens};
#[cfg(RUSTC_WITH_SPECIALIZATION)]
use syn::{
    parse_macro_input, Attribute, Error, Fields, Ident, Item, ItemEnum, ItemStruct, ItemType,
    LitStr, Variant,
};

#[cfg(RUSTC_WITH_SPECIALIZATION)]
fn filter_serde_attrs(attrs: &[Attribute]) -> bool {
    fn contains_skip(tokens: TokenStream2) -> bool {
        for token in tokens.into_iter() {
            match token {
                TokenTree::Group(group) => {
                    if contains_skip(group.stream()) {
                        return true;
                    }
                }
                TokenTree::Ident(ident) => {
                    if ident == "skip" {
                        return true;
                    }
                }
                TokenTree::Punct(_) | TokenTree::Literal(_) => (),
            }
        }

        false
    }

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        if contains_skip(attr.to_token_stream()) {
            return true;
        }
    }

    false
}