use challenge_book::utils::{binary_search, combination_2d};

/// 4重ループを2重ループを展開してメモライズ + 2重ループを二分探索することによってn^2 log nで計算する
fn lottery_bin_search_memorize(k: &[i32], m: i32) -> bool {
    // kの要素二つの組み合わせを展開
    let mut kk = combination_2d(0..k.len())
        .map(|(i, j)| k[i] + k[j])
        .collect::<Vec<i32>>();
    kk.sort(); // 二分探索をするため

    let mut ans = false;

    // 2重ループ+二分探索
    for a in 0..k.len() {
        for b in 0..k.len() {
            if let Some(_) = binary_search(&kk, &(m - k[a] - k[b])) {
                ans = true
            }
        }
    }

    ans
}

fn main() {
    {
        let k = vec![1, 3, 5];
        let m = 10;
        println!(
            "k: {k:?}, m: {m:?}, ans: {:?}",
            lottery_bin_search_memorize(&k, m)
        );
    }
    {
        let k = vec![1, 3, 5];
        let m = 9;
        println!(
            "k: {k:?}, m: {m:?}, ans: {:?}",
            lottery_bin_search_memorize(&k, m)
        );
    }
}
