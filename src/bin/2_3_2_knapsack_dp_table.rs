use challenge_book::utils::ch_max;

fn knapsack_dp_table(w_lim: usize, w_array: &[usize], v_array: &[usize]) -> usize {
    let mut dp = vec![vec![0; w_lim + 1]; w_array.len() + 1];

    for i in 0..w_array.len() {
        for w in 0..w_lim + 1 {
            // i番目の品物を選ぶ場合
            if let Some(next_weight) = w.checked_sub(w_array[i]) {
                let hold_value = dp[i][next_weight] + v_array[i];
                ch_max(&mut dp[i + 1][w], hold_value);
            }

            // i番目の品物を選ばない場合
            let no_hold_value = dp[i][w];
            ch_max(&mut dp[i + 1][w], no_hold_value);
        }
    }
    dp[w_array.len()][w_lim]
}

fn main() {
    let w_array = vec![2_usize, 1, 3, 2];
    let v_array = vec![3_usize, 2, 4, 2];

    let w_lim = 5;

    let ans = knapsack_dp_table(w_lim, &w_array, &v_array);
    println!("ans: {ans}");
}
