use crate::utils::non_enum_error;

use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DeriveInput, ExprMatch, Pat};

pub fn enum_for_map_inner(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_name = &input.ident;

    let _ = match &input.data {
        Data::Enum(_) => {}
        _ => return Err(non_enum_error()),
    };

    Ok(quote! {
        impl ::challenge_book::data_structures::enum_map::EnumForMap for #enum_name {}
    })
}

pub fn enum_map_inner(input_expr: &ExprMatch) -> syn::Result<TokenStream> {
    let mut inserts: Vec<TokenStream> = Vec::new(); // データの挿入処理を表すトークン
    let mut check_arms: Vec<TokenStream> = Vec::new(); // match式をたてるために使う`E::A => {}`のようなトークンのVec
    let mut first_pattern = Option::<&Pat>::None; // match式をたてるために使うmatch E::Aのように添える部分

    for arm in &input_expr.arms {
        let pattern = &arm.pat;
        let body = &arm.body;

        if let None = first_pattern {
            first_pattern = Some(pattern);
        }

        check_arms.push(quote! {
            #pattern => {}
        });

        inserts.push(quote_spanned! {arm.span()=>
            map.insert(#pattern, #body);
        });
    }

    let match_token = first_pattern.map(|first_pattern| {
        quote! {
            match #first_pattern {
                #(#check_arms)*
            }
        }
    });

    Ok(quote! {
        {
            #match_token

            let mut map = ::std::collections::HashMap::new();

            #(#inserts)*

            TryInto::<::challenge_book::data_structures::EnumMap<_, _>>::try_into(map).unwrap()
        }
    })
}
