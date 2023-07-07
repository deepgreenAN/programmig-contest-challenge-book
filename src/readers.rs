use crate::error::{ParseCharError, ParseError};

/// 2dのさいの目状のボードの読み込みを行う関数
pub fn read_2d_board<T>(board: &str) -> Result<Vec<Vec<T>>, ParseError>
where
    T: TryFrom<char, Error = ParseCharError>,
{
    let mut res = Vec::<Vec<T>>::new();

    // 最初の一行目はいらない
    for line in board.lines().skip(1) {
        let mut line_values = Vec::<T>::new();

        for c in line.chars() {
            line_values.push(c.try_into()?);
        }
        res.push(line_values);
    }
    Ok(res)
}

#[cfg(test)]
mod test {
    use super::read_2d_board;
    use crate::error::ParseCharError;

    #[derive(PartialEq, Eq, Debug)]
    enum Othello {
        Black,
        White,
        Empty,
    }

    impl TryFrom<char> for Othello {
        type Error = ParseCharError;
        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                '○' => Ok(Othello::White),
                '●' => Ok(Othello::Black),
                '.' => Ok(Othello::Empty),
                _ => Err(ParseCharError(value)),
            }
        }
    }

    #[test]
    fn test_read_2d_board() {
        use Othello::{Black as B, Empty as E, White as W};

        let board = r#"
..●.
○○○●
●..●
●...
"#;
        let othello_board = read_2d_board::<Othello>(board).unwrap();
        let ans = vec![
            vec![E, E, B, E],
            vec![W, W, W, B],
            vec![B, E, E, B],
            vec![B, E, E, E],
        ];
        assert_eq!(othello_board, ans);
    }
}
