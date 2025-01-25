//! This crate provides proc macros for the main library crate

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro_derive(VariantsToStr)]
pub fn variants(item: TokenStream) -> TokenStream {
    let syn_item: syn::DeriveInput = syn::parse(item).unwrap();

    let variants = match syn_item.data {
        syn::Data::Enum(enum_item) => {
            enum_item.variants.into_iter().map(|v| v.ident)
        }
        _ => panic!("VariantsToStr only works on enums"),
    };

    let variants_str = variants.clone().map(|ident| ident.to_string());
    let variants_str_lower = variants_str.clone().map(|str| str.to_lowercase());
    let variants_lens = variants_str.clone().map(|s| s.len());

    let enum_name = syn_item.ident;
    let num = variants.clone().count();

    let expanded = quote! {
        impl #enum_name {
            const fn all_variants_str() -> [&'static str; #num] 
            {
                [ #(#variants_str),* ]
            }

            fn str_to_variant(s: &str) -> Option<(#enum_name, usize)> {
                #(
                    if s.len() >= #variants_lens && &s[..#variants_lens] == #variants_str_lower {
                        return Some((Self::#variants, #variants_lens));
                    }
                )*
                return None;
            }
        }
    };
    expanded.into()
}