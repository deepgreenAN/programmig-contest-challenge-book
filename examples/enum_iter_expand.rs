use challenge_book::enum_ext::IntoEnumIterator;

#[derive(Debug, PartialEq)]
enum BloodType {
    A,
    B,
    AB,
    O,
}

struct BloodTypeIter {
    index: usize,
}

impl Iterator for BloodTypeIter {
    type Item = BloodType;
    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(BloodType::A)
            }
            1 => {
                self.index += 1;
                Some(BloodType::B)
            }
            2 => {
                self.index += 1;
                Some(BloodType::AB)
            }
            3 => {
                self.index += 1;
                Some(BloodType::O)
            }
            _ => None,
        }
    }
}

impl IntoEnumIterator for BloodType {
    type Iterator = BloodTypeIter;
    fn iter() -> <Self as IntoEnumIterator>::Iterator {
        BloodTypeIter { index: 0 }
    }
}

fn main() {
    let variants = BloodType::iter().collect::<Vec<_>>();
    assert_eq!(
        variants,
        vec![BloodType::A, BloodType::B, BloodType::AB, BloodType::O]
    );
}
