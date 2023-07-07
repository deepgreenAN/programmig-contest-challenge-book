use challenge_book_macros::EnumIter;

#[derive(Debug, PartialEq, EnumIter)]
enum BloodType {
    A,
    B,
    AB,
    O,
}

fn main() {
    use challenge_book::enum_ext::IntoEnumIterator;
    use BloodType::*;

    assert_eq!(vec![A, B, AB, O], BloodType::iter().collect::<Vec<_>>())
}
