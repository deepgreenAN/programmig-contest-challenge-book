use challenge_book::error::ParseCharError;

#[derive(Debug, PartialEq)]
enum MazeState {
    Path,
    Wall,
}

impl TryFrom<char> for MazeState {
    type Error = ParseCharError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MazeState::Path),
            '#' => Ok(MazeState::Wall),
            _ => Err(ParseCharError(value)),
        }
    }
}

fn main() {
    {
        let path: MazeState = '.'.try_into().unwrap();
        assert_eq!(path, MazeState::Path);
    }
    {
        let wall: MazeState = '#'.try_into().unwrap();
        assert_eq!(wall, MazeState::Wall);
    }
    {
        let err: Result<MazeState, _> = 'a'.try_into();
        assert!(err.is_err());
    }
}
