use challenge_book::{max, min};

/// (min, max)
fn drop_ants_time(x_array: &[i32], l: i32) -> Option<(i32, i32)> {
    // 最小の時間を計算
    let min_t = x_array.iter().fold(Option::<i32>::None, |acc, x| {
        match acc {
            Some(acc) => {
                Some(max!(acc, min!(*x, l - x))) // 近い方の端までの距離が最大の物を求める
            }
            // 最初の場合
            None => Some(min!(*x, l - x)),
        }
    });

    // 最大の時間を計算
    let max_t = x_array.iter().fold(Option::<i32>::None, |acc, x| {
        match acc {
            Some(acc) => {
                Some(max!(acc, max!(*x, l - x))) // 遠い方の端までの距離が最大の物を求める
            }
            None => Some(max!(*x, l - x)),
        }
    });

    match (min_t, max_t) {
        (Some(min_t), Some(max_t)) => Some((min_t, max_t)),
        _ => None,
    }
}

fn main() {
    let l = 10;
    let x = vec![2, 6, 7];

    println!("l: {l}, x: {x:?}, ans: {:?}", drop_ants_time(&x, l));
}
