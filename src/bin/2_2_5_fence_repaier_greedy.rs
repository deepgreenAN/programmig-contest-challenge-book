fn swap_min_first_second(array: &mut [u32]) {
    let mut min_opt = Option::<u32>::None;

    for i in 0..array.len() {
        match min_opt.as_mut() {
            None => {
                min_opt = Some(array[i]);
            }
            Some(min) => {
                if *min > array[i] {
                    array.swap(0, i); // 0に最小値
                    array.swap(1, i); // 1に二番目の値

                    *min = array[0] // minを更新
                }
            }
        }
    }
}

fn solve(l: &mut [u32]) -> u32 {
    let mut ans = 0_u32;

    for i in 0..(l.len() - 1) {
        let sub_slice = &mut l[i..]; // くっついていない板を表すサブスライス
        swap_min_first_second(sub_slice);

        ans += sub_slice[0] + sub_slice[1]; // 最も短い板二つをコストとして足す

        let first = sub_slice[0];
        sub_slice[1] += first; // 新しい板とする
    }
    ans
}

fn main() {
    let mut l = vec![8, 5, 8];
    println!("l: {l:?}");

    println!("ans: {:?}", solve(&mut l));
}
