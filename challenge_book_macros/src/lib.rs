mod data_structures;
mod enum_ext;
mod utils;

use data_structures::{enum_for_map_inner, enum_map_inner};
use enum_ext::{enum_iter_inner, enum_try_from_char_inner};

use syn::{parse_macro_input, DeriveInput, ExprMatch};

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

    let match_expr: proc_macro::TokenStream = quote::quote! {
        match X {  // Xは利用しないためなんでもよい
            #input
        }
    }
    .into();

    let match_ast = parse_macro_input!(match_expr as ExprMatch);
    let token_stream = enum_map_inner(&match_ast).unwrap_or_else(|e| e.into_compile_error());
    token_stream.into()
}

/// charから列挙体に変換する方法を簡単に定義できるマクロ
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
