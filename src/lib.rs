use anyhow::{Result, anyhow};

const SIZE: usize = 3;

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
    mboard: [[Option<Tile>; SIZE]; SIZE],
    state: MiniBoardState,
}

impl MiniBoard {
    fn new() -> Self {
        Self {
            mboard: [[None; SIZE]; SIZE],
            state: MiniBoardState::Play,
        }
    }

    fn play(&mut self, pos: Pos, tile: Tile) -> Result<()> {
        let Pos(x, y) = pos;
        if self.mboard[x][y].is_none() {
            self.mboard[x][y] = Some(tile);
            Ok(())
        } else {
            Err(anyhow!("Invalid Move (Occupied)"))
        }
    }

    fn update_state(&mut self) -> bool {
        todo!()
    }
}

pub struct Pos(usize, usize);

struct BigBoard {
    board: [[MiniBoard; SIZE]; SIZE],
    playable: Option<Pos>, //If None, can play anywhere
}

impl Pos {
    pub fn from(x: usize, y: usize) -> Self {
        if x >= SIZE || y >= SIZE {
            panic!("Position out of bounds")
        }
        Pos(x, y)
    }
}

impl BigBoard {
    fn new() -> Self {
        Self {
            board: [[MiniBoard::new(); SIZE]; SIZE],
            playable: None,
        }
    }

    fn play(&mut self, board_pos: Pos, pos: Pos, tile: Tile) -> Result<()> {
        todo!()
    }

    fn check_win(&self) -> Tile {
        todo!()
    }
}
