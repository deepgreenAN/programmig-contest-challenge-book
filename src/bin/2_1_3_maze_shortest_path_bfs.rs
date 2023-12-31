use challenge_book::utils::four_neighbors;
use challenge_book_macros::EnumTryFromChar;

use std::collections::VecDeque;

#[derive(Debug, EnumTryFromChar)]
enum MazeElement {
    #[cbook(char_lit = 'S')]
    Start,
    #[cbook(char_lit = 'G')]
    Goal,
    #[cbook(char_lit = '#')]
    Wall,
    #[cbook(char_lit = '.')]
    Path,
}

/// 幅優先探索で最短経路を求める
fn bfs_shortest_path(maze: &Vec<Vec<MazeElement>>, s_i: usize, s_j: usize) -> Option<usize> {
    let j_range = 0..maze.len();
    let i_range = 0..maze.first()?.len();

    let mut dist = vec![vec![Option::<usize>::None; i_range.end]; j_range.end]; // 距離を入れるための配列
    let mut queue = VecDeque::<(usize, usize)>::new(); // これから探索すべき頂点につながる訪問済みの頂点を入れるキュー

    // 初期条件
    dist[s_j][s_i] = Some(0);
    queue.push_back((s_i, s_j));

    // ゴールのインデックス
    let (mut g_i, mut g_j) = (Option::<usize>::None, Option::<usize>::None);

    // bfsの開始
    while let Some((i, j)) = queue.pop_front() {
        // 取り出した値がゴールだった場合は探索を終了
        if let MazeElement::Goal = maze[j][i] {
            (g_i, g_j) = (Some(i), Some(j));
            break;
        }

        // 四近傍について探索
        for (x, y) in four_neighbors(i, j, i_range.clone(), j_range.clone()) {
            // すでに探索済みの場合はなにもせず次へ
            if dist[y][x].is_some() {
                continue;
            }

            // (x, y)について距離の更新・キューへの追加
            match maze[y][x] {
                // 壁の場合は何もしない
                MazeElement::Wall => {}
                // 壁以外の場合
                _ => {
                    dist[y][x] = Some(dist[j][i].unwrap() + 1);
                    queue.push_back((x, y));
                }
            }
        }
    }

    dist[g_j?][g_i?]
}

/// スタート地点を取得する
fn get_start(maze: &Vec<Vec<MazeElement>>) -> Option<(usize, usize)> {
    let mut s: Option<(usize, usize)> = None;
    for j in 0..maze.len() {
        for i in 0..maze[j].len() {
            if let MazeElement::Start = maze[j][i] {
                s = Some((i, j))
            }
        }
    }
    s
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use challenge_book::error::CustomError;
    use challenge_book::readers::read_2d_board;

    let maze_board = {
        let board_str = r#"
#S######.#
......#..#
.#.##.##.#
.#........
##.##.####
....#....#
.#######.#
....#.....
.####.###.
....#...G#
"#;
        println!("maze:\n{board_str}");
        read_2d_board::<MazeElement>(board_str)?
    };

    let (s_i, s_j) = get_start(&maze_board).ok_or(CustomError::new("Cannot get start point."))?;

    println!("ans: {:?}", bfs_shortest_path(&maze_board, s_i, s_j));

    Ok(())
}
