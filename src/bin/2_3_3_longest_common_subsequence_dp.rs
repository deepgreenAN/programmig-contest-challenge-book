use challenge_book::max;
use challenge_book::utils::ch_max;

fn lcm(s: &str, t: &str) -> usize {
    let s_vec = s.chars().collect::<Vec<char>>();
    let t_vec = t.chars().collect::<Vec<char>>();

    let mut dp = vec![vec![0_usize; t_vec.len() + 1]; s_vec.len() + 1]; // +1が斜めに遷移するから縦横一つ大きくないといけない

    for i in 0..s_vec.len() {
        for j in 0..t_vec.len() {
            if s_vec[i] == t_vec[j] {
                let new_value = dp[i][j] + 1;
                ch_max(&mut dp[i + 1][j + 1], new_value);
            } else {
                let new_value = max!(dp[i][j + 1], dp[i + 1][j]);
                ch_max(&mut dp[i + 1][j + 1], new_value);
            }
        }
    }

    dp[s_vec.len()][t_vec.len()] // +1が斜めに遷移するから縦横一つ大きくないといけない
}

fn main() {
    let s = "abcd";
    let t = "becd";

    println!("s: {s}, t: {t}, ans: {}", lcm(s, t));
}
