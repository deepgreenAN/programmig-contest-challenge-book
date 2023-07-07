use crate::utils::{non_enum_error, variant_has_field_error};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput};

pub fn enum_iter_inner(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;
    let enum_visibility = &input.vis;

    let variants = match &input.data {
        Data::Enum(e) => &e.variants,
        _ => return Err(non_enum_error()),
    };

    // 作成するイテレータ―名
    let iter_name = format_ident!("{}Iter", enum_name);

    // 作成するイテレータ―内でindex => バリアントにmatchさせる処理を記述するアーム
    let mut arms = Vec::<TokenStream>::new();
    let mut index = 0_usize;

    for variant in variants {
        variant_has_field_error(variant)?;

        let variant_ident = &variant.ident;
        arms.push(quote! {
            #index => {
                self.index += 1;
                ::core::option::Option::Some(#enum_name::#variant_ident)
            }
        });
        index += 1;
    }

    arms.push(quote! {
        _ => {
            ::core::option::Option::None
        }
    });

    Ok(quote! {
        #enum_visibility struct #iter_name {
            index: usize
        }

        // 列挙体のバリアントをイテレートするイテレータ―
        impl Iterator for #iter_name {
            type Item = #enum_name;

            fn next(&mut self) -> Option<Self::Item> {
                match self.index {
                    #(#arms),*
                }
            }
        }

        impl ::core::fmt::Debug for #iter_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#iter_name))
                    .field("index", &self.index)
                    .finish()
            }
        }

        impl Clone for #iter_name {
            fn clone(&self) -> Self {
                #iter_name {
                    index: self.index
                }
            }
        }

        /// IntoEnumIterator
        impl ::challenge_book::enum_ext::IntoEnumIterator for #enum_name {
            type Iterator = #iter_name;
            fn iter() -> <Self as ::challenge_book::enum_ext::IntoEnumIterator>::Iterator {
                #iter_name {
                    index: 0
                }
            }
        }
    })
}
