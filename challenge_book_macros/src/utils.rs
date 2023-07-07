use proc_macro2::Span;
use syn::{spanned::Spanned, Variant};

/// 列挙体用のマクロに構造体やユニオンを利用した場合
pub fn non_enum_error() -> syn::Error {
    syn::Error::new(Span::call_site(), "This macro only supports enums.")
}

/// バリアントがフィールドを持っては行けない場合
pub fn variant_has_field_error(variant: &Variant) -> Result<(), syn::Error> {
    if variant.fields.len() != 0 {
        Err(syn::Error::new(
            variant.span(),
            "This macro enum must not have variant fields.",
        ))
    } else {
        Ok(())
    }
}
