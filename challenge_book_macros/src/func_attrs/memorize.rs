use crate::helper::{get_meta, LitStrOrIdent};

use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_quote, ExprClosure, ItemFn};
use syn::{punctuated::Punctuated, token::Comma, Block, Expr, FnArg, Ident, Pat, ReturnType, Type};

/// 共通箇所の返り値の型
struct MemorizeContext {
    /// 引数部分のアサーション
    fn_input_assertion: TokenStream,
    /// 返り値部分のアサーション
    fn_output_assertion: TokenStream,
    /// キャッシュのハッシュマップの定義部分
    global_map_def: TokenStream,
    /// 新しい関数ブロック
    new_block: Block,
}

/// メタアトリビュート
#[derive(Debug, FromMeta)]
struct MetaInput {
    #[darling(default)]
    skip_args: Option<String>,
}

/// クロージャー・関数に共通の処理．
fn memorize_share_inner(
    fn_name: &Ident,
    fn_inputs: &Punctuated<FnArg, Comma>,
    fn_output: &ReturnType,
    fn_block: &Block,
    skip_args: &[Ident],
) -> syn::Result<MemorizeContext> {
    // 引数の名前と型
    let mut fn_input_args: Vec<(Box<Pat>, Box<Type>)> = Vec::new();
    for fn_arg in fn_inputs {
        match fn_arg {
            // selfなどの予約語
            FnArg::Receiver(_) => {
                return Err(syn::Error::new(
                    fn_arg.span(),
                    "Function cannot have self arg.",
                ));
            }
            // 引数:型
            FnArg::Typed(pat_type) => {
                fn_input_args.push((pat_type.pat.clone(), pat_type.ty.clone()));
            }
        }
    }
    // スキップしたい変数名が存在するかチェック
    {
        for skip_arg in skip_args.iter() {
            if !fn_input_args
                .iter()
                .filter_map(|(pat, _)| match pat.as_ref() {
                    Pat::Ident(pat_ident) => Some(&pat_ident.ident),
                    _ => None,
                })
                .any(|fn_input_ident| fn_input_ident == skip_arg)
            {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "Contains invalid skip args.",
                ));
            }
        }
    }

    // スキップした後の引数の名前と型
    let mut skipped_fn_inputs_args: Vec<(&Ident, &Box<Type>)> = Vec::new();
    for (pat, ty) in fn_input_args.iter() {
        match pat.as_ref() {
            Pat::Ident(pat_ident) => {
                if !skip_args.contains(&pat_ident.ident) {
                    skipped_fn_inputs_args.push((&pat_ident.ident, ty));
                }
            }
            _ => {
                return Err(syn::Error::new(
                    pat.span(),
                    "This macro cannot use destructuring or ref binding.",
                ));
            }
        }
    }

    // 返り値の型．型注釈されていない場合はエラーを返す．
    let fn_output_ty = match fn_output {
        ReturnType::Type(_, ty) => ty,
        ReturnType::Default => {
            return Err(syn::Error::new(
                fn_output.span(),
                "Return type must not be (). If you yse closure, it must have type annotation.",
            ));
        }
    };

    // スキップしていない引数が全てハッシュ可能であるかどうかをチェックする部分
    let fn_input_assertion = {
        let fn_input_assertion_iter = skipped_fn_inputs_args.iter().map(|(ident, ty)| {
            let assert_ty = format_ident!("_AssertInput{}", ident.to_string().to_uppercase());
            quote_spanned! {ty.span()=>
                struct #assert_ty where #ty: ::std::hash::Hash + Eq;
            }
        });
        quote!(#(#fn_input_assertion_iter)*)
    };

    // 返り値がクローン可能であるかどうかをチェックする部分
    let fn_output_assertion = quote_spanned! {fn_output_ty.span()=>
        struct _AssertionClone where #fn_output_ty: Clone + 'static;
    };

    // グローバルのハッシュマップの名前
    let global_map_name = format_ident!("MEMORIZE_MAP_{}", fn_name.to_string().to_uppercase());

    // item_fnの関数ブロックを書き換える．(他はそのままにできる)
    let new_block: Block = {
        let fn_input_names_and_types = {
            let token_iter = fn_input_args.iter().map(|(pat, ty)| quote!(#pat: #ty));
            quote! {#(#token_iter),*}
        };
        let fn_input_names = {
            let token_iter = fn_input_args.iter().map(|(pat, _)| pat);
            quote! {#(#token_iter),*}
        };
        let global_map_input_names = {
            let token_iter = skipped_fn_inputs_args.iter().map(|(pat, _)| pat);
            quote! {#(#token_iter),*}
        };

        parse_quote! {
            {
                // キャッシュの中にあったらそれを返す
                {
                    if let Some(value) = #global_map_name
                        .get_or_init(|| ::std::sync::Mutex::new(::std::collections::HashMap::new()))
                        .lock()
                        .unwrap()
                        .get(&(#global_map_input_names))
                    {
                        return value.clone();
                    }
                }

                //元の関数を実行(早期リターンを防ぐ)
                let block_fn = |#fn_input_names_and_types| #fn_block;
                let ret: #fn_output_ty = block_fn(#fn_input_names);

                // キャッシュに追加する
                {
                    #global_map_name
                        .get_or_init(|| ::std::sync::Mutex::new(::std::collections::HashMap::new()))
                        .lock()
                        .unwrap()
                        .insert((#global_map_input_names), ret.clone());
                }
                ret
            }
        }
    };

    let global_map_def = {
        let skipped_input_types = skipped_fn_inputs_args.iter().map(|(_, ty)| ty);
        quote! {
            // グローバルマップ
            static #global_map_name: ::std::sync::OnceLock<
            ::std::sync::Mutex<
                ::std::collections::HashMap<
                    (#(#skipped_input_types),*), #fn_output_ty
                    >
                >
            > = ::std::sync::OnceLock::new();
        }
    };

    Ok(MemorizeContext {
        fn_input_assertion,
        fn_output_assertion,
        global_map_def,
        new_block,
    })
}

/// memorizeの関数に対しての処理．関数ブロックを直接書き換えるため，所有権を奪う．
pub fn memorize_fn_inner(args: TokenStream, mut item_fn: ItemFn) -> syn::Result<TokenStream> {
    // メタアトリビュートの解析
    let skip_args = {
        let MetaInput { skip_args } = get_meta::<MetaInput>(args)?;
        match skip_args {
            Some(skip_args_str) => skip_args_str
                .split(",")
                .map(|ident_str| format_ident!("{}", ident_str.trim()))
                .collect::<Vec<_>>(),
            None => Vec::new(),
        }
    };

    // ジェネリクスがあった場合にエラーを返す
    if !item_fn.sig.generics.params.is_empty() {
        return Err(syn::Error::new(
            item_fn.sig.generics.params.span(),
            "Function must not have generic params for this macro",
        ));
    }

    let fn_name = &item_fn.sig.ident; // 関数名
    let fn_inputs = &item_fn.sig.inputs; // 関数の引数部分
    let fn_output = &item_fn.sig.output; // 関数の返り値部分
    let fn_block = &item_fn.block; // 関数ブロック

    let MemorizeContext {
        fn_input_assertion,
        fn_output_assertion,
        global_map_def,
        new_block,
    } = memorize_share_inner(fn_name, fn_inputs, fn_output, fn_block, &skip_args)?;

    *item_fn.block = new_block;

    Ok(quote! {
        // アサーション
        #fn_input_assertion
        #fn_output_assertion

        // グローバルキャッシュの定義
        #global_map_def

        // 関数の展開
        #item_fn
    })
}

/// memorizeのクロージャーに対しての処理
pub fn memorize_cl_inner(
    fn_name: &LitStrOrIdent,
    mut closure: ExprClosure,
) -> syn::Result<TokenStream> {
    let fn_name = format_ident!("{}", fn_name.to_string());

    // 引数が型注釈されていることを確認しながら，`Punctuated<FnArg, Comma>`を作成．
    let mut fn_inputs: Punctuated<FnArg, Comma> = Punctuated::new();
    for input_pat in &closure.inputs {
        match input_pat {
            Pat::Type(pat_type) => {
                fn_inputs.push(FnArg::Typed(pat_type.clone()));
            }
            _ => {
                return Err(syn::Error::new(
                    input_pat.span(),
                    "Closure needs type annotation for args and return.",
                ))
            }
        }
    }

    let fn_output = &closure.output;

    // bodyがブロックで囲われていることを確認しながら取得
    let fn_block = match closure.body.as_ref() {
        Expr::Block(expr_block) => &expr_block.block,
        _ => {
            return Err(syn::Error::new(
                closure.body.span(),
                "Closure body must be block",
            ));
        }
    };

    let MemorizeContext {
        fn_input_assertion,
        fn_output_assertion,
        global_map_def,
        new_block,
    } = memorize_share_inner(&fn_name, &fn_inputs, fn_output, fn_block, &Vec::new())?;

    *closure.body = {
        parse_quote! {
            #new_block
        }
    };

    Ok(quote! {
        {
            // アサーション
            #fn_input_assertion
            #fn_output_assertion

            // グローバルキャッシュの定義
            #global_map_def

            // クロージャーの展開
            #closure
        }

    })
}
