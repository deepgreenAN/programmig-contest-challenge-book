use crate::helper::LitStrOrIdent;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::{parse_quote, ExprClosure, ItemFn};
use syn::{Block, Expr, FnArg, Ident, Pat};

/// プロファイラの定義部分
fn profiler_def(profiler_name: &Ident) -> TokenStream {
    quote! {
        static #profiler_name: ::std::sync::OnceLock<
            ::std::sync::Mutex<
                ::challenge_book::utils::FuncProfile
            >
        > = ::std::sync::OnceLock::new()
    }
}

// ブロックの変更部分
fn modify_block(
    profiler_name: &Ident,
    fn_input_names: &Vec<Ident>,
    fn_block: &Block,
) -> syn::Result<Block> {
    // 引数名と`profile_start_xxx`が被っていないか調べる
    for ident in fn_input_names.iter() {
        if &ident.to_string() == "profile_start_xxx" {
            return Err(syn::Error::new(
                ident.span(),
                "Cannot use profile_start_xxx for arg name.",
            ));
        }
    }

    // 新しくブロックを作成
    Ok(parse_quote! {
        {
            let profile_start_xxx = ::std::time::Instant::now();

            let block_fn = |#(#fn_input_names),*| #fn_block;
            let res = block_fn(#(#fn_input_names),*);

            {
                let duration = profile_start_xxx.elapsed();
                let mut profiler_guard = #profiler_name
                    .get_or_init(|| ::std::sync::Mutex::new(Default::default()))
                    .lock()
                    .unwrap();

                profiler_guard.mean_t = ((profiler_guard.mean_t * profiler_guard.call_n) + duration)
                    / (profiler_guard.call_n + 1);
                profiler_guard.call_n += 1;
            }
            res
        }
    })
}

/// profileの関数についての部分．`ItemFn`を書き換えるため所有権を奪う．
pub fn profile_fn_inner(_args: TokenStream, mut item_fn: ItemFn) -> syn::Result<TokenStream> {
    let fn_name = &item_fn.sig.ident;
    let profiler_name = format_ident!("PROFILER_{}", fn_name.to_string().to_uppercase());

    // 引数名
    let mut fn_input_names: Vec<Ident> = Vec::new();
    for fn_arg in &item_fn.sig.inputs {
        match fn_arg {
            FnArg::Receiver(_) => {
                return Err(syn::Error::new(
                    fn_arg.span(),
                    "Function cannot have self arg.",
                ));
            }
            FnArg::Typed(pat_type) => match pat_type.pat.as_ref() {
                Pat::Ident(pat_ident) => {
                    fn_input_names.push(pat_ident.ident.clone());
                }
                _ => {
                    return Err(syn::Error::new(
                        pat_type.span(),
                        "Invalid arg for this macro.",
                    ));
                }
            },
        }
    }

    let fn_block = &item_fn.block;

    let global_profiler_def = profiler_def(&profiler_name);
    let new_block = modify_block(&profiler_name, &fn_input_names, fn_block)?;

    *item_fn.block = new_block;

    Ok(quote! {
        // グローバルキャッシュの定義
        #global_profiler_def;

        // 関数の展開
        #item_fn
    })
}

/// profileのクロージャーについての部分
pub fn profile_cl_inner(
    fn_name: &LitStrOrIdent,
    mut closure: ExprClosure,
) -> syn::Result<TokenStream> {
    let profiler_name = format_ident!("PROFILER_{}", fn_name.to_string().to_uppercase());

    // 引数名
    let mut fn_input_names: Vec<Ident> = Vec::new();
    for pat in &closure.inputs {
        match pat {
            Pat::Ident(pat_ident) => {
                fn_input_names.push(pat_ident.ident.clone());
            }
            _ => {
                return Err(syn::Error::new(pat.span(), "Invalid arg for this macro."));
            }
        }
    }

    let fn_block = match closure.body.as_ref() {
        Expr::Block(expr_block) => &expr_block.block,
        _ => {
            return Err(syn::Error::new(
                closure.body.span(),
                "The body of the closure must be a block to use this macro.",
            ));
        }
    };

    let new_block = modify_block(&profiler_name, &fn_input_names, fn_block)?;

    *closure.body = {
        parse_quote! {
            #new_block
        }
    };

    Ok(quote! {
        // クロージャー代入部分の展開
        #closure
    })
}

/// クロージャーのプロファイラの初期化
pub fn init_profile_cl_inner(fn_name: &LitStrOrIdent) -> TokenStream {
    let profiler_name = format_ident!("PROFILER_{}", fn_name.to_string().to_uppercase());
    let profiler_def = profiler_def(&profiler_name);
    quote! {
        #profiler_def;
    }
}

/// プロファイル結果の取得に利用するマクロの内側
pub fn get_profile_inner(fn_name: &LitStrOrIdent) -> TokenStream {
    let profiler_name = format_ident!("PROFILER_{}", fn_name.to_string().to_uppercase());
    quote! {
        match #profiler_name.get() {
            Some(profiler) => {
                profiler.lock().unwrap().clone()
            },
            None => {
                ::challenge_book::utils::FuncProfile::default()
            }
        }
    }
}
