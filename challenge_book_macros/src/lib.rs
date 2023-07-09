mod data_structures;
mod enum_ext;
mod func_attrs;
mod helper;

use data_structures::{enum_for_map_inner, enum_map_inner};
use enum_ext::{enum_iter_inner, enum_try_from_char_inner};
use func_attrs::{
    get_profile_inner, init_profile_cl_inner, memorize_cl_inner, memorize_fn_inner,
    profile_cl_inner, profile_fn_inner,
};
use helper::LitStrOrIdent;

use syn::{parse_macro_input, parse_quote, DeriveInput, ExprMatch};

/// 列挙体からイテレータ―を作成する．対象となる列挙体はバリアントにフィールドを持つことはできない．
/// このマクロは`challenge_book::IntoEnumIterator`トレイトを実装する．
/// ```rust
/// use challenge_book::IntoEnumIterator;
/// use challenge_book_macros::EnumIter;
///
/// #[derive(PartialEq, Debug, EnumIter)]
/// enum Color {
///     Red,
///     Blue,
///     Green,
///     Yellow
/// }
///
/// use Color::*;
///
/// assert_eq!(vec![Red, Blue, Green, Yellow], Color::iter().collect::<Vec<_>>());
/// ```
#[proc_macro_derive(EnumIter)]
pub fn enum_iter(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let token_stream = enum_iter_inner(&input_ast).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// EnumMapが利用可能な列挙体にする．
/// ```rust
/// use challenge_book_macros::{EnumIter, EnumForMap, PartialEq, Eq, Hash};
/// use challenge_book::enum_map;
///
/// #[derive(EnumForMap, EnumIter, PartialEq, Eq, Hash)]
/// enum Color {
///     Red,
///     Blue,
///     Green,
///     Black
/// }
///
/// use Color::*;
///
/// let em = enum_map! {
///     Red => (255_u8, 0_u8, 0_u8),
///     Blue => (0, 0, 255),
///     Green => (0, 255, 0),
///     Black => (255, 255, 255)
/// };
///
/// assert_eq!(em.get(&Red), &(255, 0, 0));
/// assert_eq!(em.get(&Blue), &(0, 0, 255));
/// assert_eq!(em.get(&Green), &(0, 255, 0));
/// assert_eq!(em.get(&Black), &(255, 255, 255));
/// ```
#[proc_macro_derive(EnumForMap)]
pub fn enum_for_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let token_stream = enum_for_map_inner(&input_ast).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// enum_mapマクロ．match式としてバリアントをチェックするため手続き型マクロである必要がある．
/// ```rust
/// use challenge_book_macros::{EnumIter, EnumForMap, PartialEq, Eq, Hash};
/// use challenge_book::enum_map;
///
/// #[derive(EnumForMap, EnumIter, PartialEq, Eq, Hash)]
/// enum Color {
///     Red,
///     Blue,
///     Green,
///     Black
/// }
///
/// use Color::*;
///
/// let em = enum_map! {
///     Red => (255_u8, 0_u8, 0_u8),
///     Blue => (0, 0, 255),
///     Green => (0, 255, 0),
///     Black => (255, 255, 255)
/// };
///
/// assert_eq!(em.get(&Red), &(255, 0, 0));
/// assert_eq!(em.get(&Blue), &(0, 0, 255));
/// assert_eq!(em.get(&Green), &(0, 255, 0));
/// assert_eq!(em.get(&Black), &(255, 255, 255));
/// ```
#[proc_macro]
pub fn enum_map(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let match_ast: ExprMatch = parse_quote! {
        match X {  // Xは利用しないためなんでもよい
            #input
        }
    };

    let token_stream = enum_map_inner(&match_ast).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// charから列挙体に変換する方法を簡単に定義できるマクロ．TryFrom<chr, Error=ParseCharError>を実装する．
/// ```rust
/// use challenge_book_macros::EnumTryFromChar;
///
/// #[derive(Debug, PartialEq, EnumTryFromChar)]
/// enum MazeState {
///     #[cbook(char_lit = '.')]
///     Path,
///     #[cbook(char_lit = '#')]
///     Wall
/// }
///
///
/// let path: MazeState = '.'.try_into().unwrap();
/// assert_eq!(path, MazeState::Path);
///
/// let wall: MazeState = '#'.try_into().unwrap();
/// assert_eq!(wall, MazeState::Wall);
///
/// let err: Result<MazeState, _> = 'a'.try_into();
/// assert!(err.is_err());
/// ```
#[proc_macro_derive(EnumTryFromChar, attributes(cbook))]
pub fn enum_try_from_char(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input_ast = parse_macro_input!(input as DeriveInput);
    let token_stream =
        enum_try_from_char_inner(&input_ast).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// 関数のメモ化を行う．
/// ```rust
/// use challenge_book_macros::memorize;
/// #[memorize]
/// fn fibo(n: usize) -> u32 {
///     if n == 0 {
///         0
///     } else if n == 1 {
///         1
///     } else {
///         fibo(n - 1).saturating_add(fibo(n - 2))
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn memorize(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = parse_macro_input!(item as syn::ItemFn);
    let token_stream =
        memorize_fn_inner(args.into(), item_fn).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// memorize_closureの引数用のパースを実装した型
struct MemorizeClosureInput {
    fn_name: LitStrOrIdent,
    closure: syn::ExprClosure,
}

impl syn::parse::Parse for MemorizeClosureInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fn_name = input.parse()?;
        input.parse::<syn::token::Comma>()?;
        let closure = input.parse()?;

        Ok(Self { fn_name, closure })
    }
}

/// クロージャーのメモ化を行う．第一引数としてグローバルキャッシュの名前をとる．
/// ```rust
/// use challenge_book_macros::memorize_closure;
///
/// let x = memorize_closure!("a", |a: usize| -> u32 { a as u32 });
/// x(10);
/// ```
#[proc_macro]
pub fn memorize_closure(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let macro_input = parse_macro_input!(input as MemorizeClosureInput);
    let MemorizeClosureInput { fn_name, closure } = macro_input;

    let token_stream =
        memorize_cl_inner(&fn_name, closure).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// 簡単な関数のプロファイリングを行えるように関数を修正する．
/// ```rust
/// use challenge_book_macros::{get_profile, profile};
///
/// #[profile]
/// fn fibo(n: usize) -> u32 {
///     if n == 0 {
///         0
///     } else if n == 1 {
///         1
///     } else {
///         fibo(n - 1).saturating_add(fibo(n - 2))
///     }
/// }
///
/// println!("fibo(30): {}", fibo(30));
/// let profile = get_profile!(fibo);
/// println!("{profile:?}");
/// ```
#[proc_macro_attribute]
pub fn profile(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_fn = parse_macro_input!(item as syn::ItemFn);
    let token_stream =
        profile_fn_inner(args.into(), item_fn).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// profile_closureの引数
struct ProfileClosureInput {
    fn_name: LitStrOrIdent,
    closure: syn::ExprClosure,
}

impl syn::parse::Parse for ProfileClosureInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let fn_name = input.parse()?;
        input.parse::<syn::token::Comma>()?;
        let closure = input.parse()?;

        Ok(Self { fn_name, closure })
    }
}

/// クロージャーのプロファイラの初期化
/// ```rust
/// use challenge_book_macro::{init_profiler_closure, profile_closure, get_profile};
/// init_profiler_closure!("x_closure");
/// let x = profile_closure!("x_closure", |a| { a + 20 });
/// x(10);
/// println!("{:?}", get_profile!("x_closure"));
/// ```
#[proc_macro]
pub fn init_profiler_closure(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let fn_name = parse_macro_input!(input as LitStrOrIdent);

    init_profile_cl_inner(&fn_name).into()
}

/// クロージャーのプロファイリングを行えるようにクロージャーを修正
#[proc_macro]
/// ```rust
/// use challenge_book_macro::{init_profiler_closure, profile_closure, get_profile};
/// init_profiler_closure!("x_closure");
/// let x = profile_closure!("x_closure", |a| { a + 20 });
/// x(10);
/// println!("{:?}", get_profile!("x_closure"));
/// ```
pub fn profile_closure(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ProfileClosureInput { fn_name, closure } = parse_macro_input!(input as ProfileClosureInput);

    let token_stream =
        profile_cl_inner(&fn_name, closure).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// プロファイラから結果を取得するマクロ
/// ```rust
/// use challenge_book_macro::{init_profiler_closure, profile_closure, get_profile};
/// init_profiler_closure!("x_closure");
/// let x = profile_closure!("x_closure", |a| { a + 20 });
/// x(10);
/// println!("{:?}", get_profile!("x_closure"));
/// ```
#[proc_macro]
pub fn get_profile(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lit_str_or_ident = parse_macro_input!(input as LitStrOrIdent);

    get_profile_inner(&lit_str_or_ident).into()
}
