use challenge_book::utils::eight_neighbors;

/// 地面の状態を示す列挙体
#[derive(Debug)]
enum GroundState {
    // 池
    Lake,
    // 地面
    Ground,
}

impl TryFrom<char> for GroundState {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'W' => Ok(GroundState::Lake),
            '.' => Ok(GroundState::Ground),
            _ => Err(format!("Ground Parse Error: {value}")),
        }
    }
}

/// groundsを読み取る関数
fn read_grounds(grounds: &str) -> Vec<Vec<GroundState>> {
    let mut res = Vec::<Vec<GroundState>>::new();

    // 最初の一行目はいらない
    for line in grounds.lines().skip(1) {
        let mut line_states = Vec::<GroundState>::new();

        for c in line.chars() {
            line_states.push(c.try_into().unwrap());
        }
        res.push(line_states);
    }
    res
}

/// 再帰する関数
fn dfs_recur(i: usize, j: usize, grounds: &mut Vec<Vec<GroundState>>) {
    // 今いる位置をGroundに置き換える
    grounds[j][i] = GroundState::Ground;

    // 八近傍に再帰
    for (n_i, n_j) in eight_neighbors(i, j, 0..grounds[j].len(), 0..grounds.len()) {
        if let GroundState::Lake = grounds[n_j][n_i] {
            dfs_recur(n_i, n_j, grounds);
        }
    }
}

/// 池の個数を数える．最も外側のdfs_recurの呼び出しによってひとまとまりの池を消せるため，その呼び出し回数を数えている．
fn count_lake(mut grounds: Vec<Vec<GroundState>>) -> usize {
    let mut ans = 0_usize;

    for j in 0..grounds.len() {
        for i in 0..grounds[j].len() {
            if let GroundState::Lake = grounds[j][i] {
                ans += 1;
                dfs_recur(i, j, &mut grounds);
            }
        }
    }
    ans
}

fn main() {
    let grounds = {
        let grounds_str = r#"
W........WW.
.WWW.....WWW
....WW...WW.
.........WW.
.........W..
..W......W..
.W.W.....WW.
W.W.W.....W.
.W.W......W.
..W.......W.
"#;
        println!("grounds: \n{grounds_str}");
        read_grounds(grounds_str)
    };

    println!("ans: {}", count_lake(grounds));
}
