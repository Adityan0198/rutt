use crate::MiniBoardState::Play;

#[derive(Copy, Clone, Debug)]
enum Tile {
    X,
    O,
}

#[derive(Copy, Clone, Debug)]
enum MiniBoardState {
    Play,
    Win(Tile),
    Tie,
}

#[derive(Copy, Clone, Debug)]
struct MiniBoard {
    mboard: [[Option<Tile>; 3]; 3],
    state: MiniBoardState,
}

impl MiniBoard {
    fn new() -> Self {
        Self {
            mboard: [[None; 3]; 3],
            state: Play,
        }
    }
}

struct BigBoard {
    board: [[MiniBoard; 3]; 3],
}

impl BigBoard {
    fn new() -> Self {
        Self {
            board: [[MiniBoard::new(); 3]; 3],
        }
    }
}
