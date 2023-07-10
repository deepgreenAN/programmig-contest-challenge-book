use challenge_book::{data_structures::EnumMap, enum_ext::IntoEnumIterator, min};
use challenge_book_macros::{EnumForMap, EnumIter};

#[derive(Clone, Copy, Debug, EnumForMap, EnumIter, Hash, PartialEq, Eq)]
enum Coin {
    C500,
    C100,
    C50,
    C10,
    C5,
    C1,
}

use Coin::*;

impl Coin {
    fn value(&self) -> u32 {
        match self {
            C500 => 500,
            C100 => 100,
            C50 => 50,
            C10 => 10,
            C5 => 5,
            C1 => 1,
        }
    }
}

fn coin_greedy(amount: u32, coin_number: &EnumMap<Coin, usize>) -> Vec<Coin> {
    let mut ans = Vec::<Coin>::new();
    let mut amount = amount;

    for coin in Coin::iter() {
        let n = min!(amount / coin.value(), *coin_number.get(&coin) as u32); // 上限と比較して小さい方
        amount -= n * coin.value();
        ans.extend((0..n).map(|_| coin));
    }
    ans
}

fn main() {
    use challenge_book_macros::enum_map;

    let coin_number = enum_map! {
        C500 => 2_usize,
        C100 => 0,
        C50 => 3,
        C10 => 1,
        C5 => 2,
        C1 => 3
    };

    let amount = 620_u32;

    println!(
        "coin_number: {coin_number:?}, amount: {amount}, ans: {:?}",
        coin_greedy(amount, &coin_number)
    );
}
