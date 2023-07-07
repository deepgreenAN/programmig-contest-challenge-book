use challenge_book_macros::{EnumForMap, EnumIter};

#[derive(EnumForMap, EnumIter, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Blue,
    Green,
    Black,
}

fn main() {
    use challenge_book::enum_map;

    let em = enum_map! {
        Color::Red => (255_u8, 0_u8, 0_u8),
        Color::Blue => (0, 0, 255),
        Color::Green => (0, 255, 0),
        Color::Black => (255, 255, 255)
    };

    assert_eq!(em.get(&Color::Red), &(255, 0, 0));
    assert_eq!(em.get(&Color::Blue), &(0, 0, 255));
    assert_eq!(em.get(&Color::Green), &(0, 255, 0));
    assert_eq!(em.get(&Color::Black), &(255, 255, 255));
}
