use challenge_book_macros::EnumTryFromChar;

#[derive(Debug, PartialEq, EnumTryFromChar)]
enum MazeState {
    #[cbook(char_lit = '.')]
    Path,
    #[cbook(char_lit = '#')]
    Wall,
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
