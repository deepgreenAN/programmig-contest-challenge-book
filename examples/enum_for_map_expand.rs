use challenge_book::data_structures::enum_map::EnumForMap;
use challenge_book_macros::EnumIter;

#[derive(EnumIter, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Blue,
    Green,
    Black,
}

impl EnumForMap for Color {}

fn main() {
    use challenge_book::data_structures::EnumMap;
    use std::collections::HashMap;

    // 以下もマクロにする
    let mut x = HashMap::<Color, (u8, u8, u8)>::new();

    x.insert(Color::Red, (255, 0, 0));
    x.insert(Color::Blue, (0, 0, 255));
    x.insert(Color::Green, (0, 255, 0));
    x.insert(Color::Black, (255, 255, 255));

    // 以下によって全てのバリアントがあることを保証する
    match Color::Red {
        Color::Red => {}
        Color::Blue => {}
        Color::Green => {}
        Color::Black => {}
    };

    let em: EnumMap<Color, (u8, u8, u8)> = x.try_into().unwrap();

    assert_eq!(em.get(&Color::Red), &(255, 0, 0));
    assert_eq!(em.get(&Color::Blue), &(0, 0, 255));
    assert_eq!(em.get(&Color::Green), &(0, 255, 0));
    assert_eq!(em.get(&Color::Black), &(255, 255, 255));
}
