use crate::utils::non_enum_error;

use darling::FromVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

/// バリアントのアトリビュート
#[derive(Debug, FromVariant)]
#[darling(attributes(cbook))]
struct VariantReceiver {
    char_lit: char,
}

pub fn enum_try_from_char_inner(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;

    let variants = match &input.data {
        Data::Enum(e) => &e.variants,
        _ => return Err(non_enum_error()),
    };

    // パースの時に必要なarms
    let mut arms: Vec<TokenStream> = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;
        let VariantReceiver { char_lit } = VariantReceiver::from_variant(variant)?;

        arms.push(quote! {
            #char_lit => {::core::result::Result::Ok(#enum_name::#variant_ident)}
        });
    }

    arms.push(quote! {
        _ => ::core::result::Result::Err(::challenge_book::error::ParseCharError(value))
    });

    Ok(quote! {
        impl TryFrom<char> for #enum_name {
            type Error = ::challenge_book::error::ParseCharError;
            fn try_from(value: char) -> Result<Self, Self::Error> {
                match value {
                    #(#arms)*
                }
            }
        }
    })
}
