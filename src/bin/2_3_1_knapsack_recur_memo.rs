use challenge_book::max;
use challenge_book_macros::{memorize, profile};

/// i番目以降の品物から重さの総和がw以下となるように選んだときの価値．i:(0 -> N), w:(W -> 0)
#[profile]
#[memorize(skip_args = "w_array, v_array")]
fn knapsack_recur(i: usize, w: usize, w_array: &[usize], v_array: &[usize]) -> usize {
    if i >= w_array.len() {
        0 // 選べる品物が無い
    } else {
        if let Some(sub_w) = w.checked_sub(w_array[i]) {
            max!(
                knapsack_recur(i + 1, w, w_array, v_array), // i番目を入れない場合
                knapsack_recur(i + 1, sub_w, w_array, v_array) + v_array[i]  // i番目を入れる場合
            )
        } else {
            knapsack_recur(i + 1, w, w_array, v_array)
        }
    }
}

fn main() {
    let w_array = vec![2_usize, 1, 3, 2];
    let v_array = vec![3_usize, 2, 4, 2];

    let w_lim = 5;

    let ans = knapsack_recur(0, w_lim, &w_array, &v_array);
    println!("ans: {ans}");

    println!("{:?}", challenge_book_macros::get_profile!(knapsack_recur));
}
