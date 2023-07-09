fn solve(s: &String) -> String {
    let s_vec: Vec<char> = s.chars().collect();

    let mut ans = Vec::<char>::new();
    let mut increment_counter = 0_usize;
    let mut decrement_counter = s_vec.len() - 1;

    while increment_counter <= decrement_counter {
        // 辞書比較の小さい方か左であるかどうかのフラッグ
        let mut is_left = Option::<bool>::None;

        // 前側からみた文字列と後ろ側からみた文字列の辞書比較
        for i in 0..((decrement_counter + 1) - increment_counter) {
            if s_vec[increment_counter + i] < s_vec[decrement_counter - i] {
                // 前側から見た文字列の方が小さい場合
                is_left = Some(true);
                break;
            } else if s_vec[increment_counter + i] > s_vec[decrement_counter - i] {
                // 後ろ側から見た文字列の方が小さい場合
                is_left = Some(false);
                break;
            }
        }

        // 辞書比較の小さい方から文字列を取得(最後まで同じだった場合は左からとる)
        if is_left.unwrap_or(true) {
            ans.push(s_vec[increment_counter]);
            increment_counter += 1;
        } else {
            ans.push(s_vec[decrement_counter]);
            decrement_counter -= 1;
        }
    }

    ans.into_iter().collect()
}

fn main() {
    let s = "ACDBCB".to_string();
    let ans = solve(&s);
    println!("s: {s}, ans: {ans}");
}
