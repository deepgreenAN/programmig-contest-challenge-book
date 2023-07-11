use darling::{ast::NestedMeta, FromMeta};
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

/// LitStrかIdentをパースできる列挙体
pub enum LitStrOrIdent {
    LitStr(syn::LitStr),
    Ident(syn::Ident),
}

impl ToString for LitStrOrIdent {
    fn to_string(&self) -> String {
        match self {
            LitStrOrIdent::LitStr(lit_str) => lit_str.value(),
            LitStrOrIdent::Ident(ident) => ident.to_string(),
        }
    }
}

impl syn::parse::Parse for LitStrOrIdent {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(LitStrOrIdent::LitStr(input.parse()?))
        } else if input.peek(syn::Ident) {
            Ok(LitStrOrIdent::Ident(input.parse()?))
        } else {
            Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "expected LitStr or Ident.",
            ))
        }
    }
}

/// メタアトリビュートの解析
pub fn get_meta<T: FromMeta>(args: proc_macro2::TokenStream) -> syn::Result<T> {
    let attr_args = NestedMeta::parse_meta_list(args)?;
    let value = T::from_list(&attr_args)?;
    Ok(value)
}
