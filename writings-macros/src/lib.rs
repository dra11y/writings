//! Procedural macros for the [writings](../writings) crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, parse_macro_input};

#[proc_macro_derive(WritingsTrait)]
pub fn derive_writings_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("WritingsTrait can only be derived for enums"),
    };

    let match_arms: Vec<_> = variants
        .iter()
        .map(|v| {
            let variant_name = &v.ident;
            quote! {
                #name::#variant_name(item) => item
            }
        })
        .collect();

    let expanded = quote! {
        impl WritingsTrait for #name {
            fn ty(&self) -> WritingsType {
                WritingsType::from(self)
            }

            fn ref_id(&self) -> String {
                match self {
                    #(#match_arms.ref_id(),)*
                }
            }

            fn title(&self) -> String {
                match self {
                    #(#match_arms.title(),)*
                }
            }

            fn subtitle(&self) -> Option<String> {
                match self {
                    #(#match_arms.subtitle(),)*
                }
            }

            fn author(&self) -> Author {
                match self {
                    #(#match_arms.author(),)*
                }
            }

            fn number(&self) -> Option<u32> {
                match self {
                    #(#match_arms.number(),)*
                }
            }

            fn paragraph(&self) -> u32 {
                match self {
                    #(#match_arms.paragraph(),)*
                }
            }

            fn text(&self) -> String {
                match self {
                    #(#match_arms.text(),)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
