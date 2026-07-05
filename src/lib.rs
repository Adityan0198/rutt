use anyhow::{Result, anyhow};

const SIZE: usize = 3;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    X,
    O,
}

pub struct Pos(usize, usize);

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

    fn play(&mut self, pos: Pos, tile: Tile) -> Result<MiniBoardState> {
        let Pos(x, y) = pos;
        if self.mboard[x][y].is_none() {
            self.mboard[x][y] = Some(tile);
            Ok(self.update_state(tile))
        } else {
            Err(anyhow!("Invalid Move (Occupied)"))
        }
    }

    fn update_state(&mut self, tile: Tile) -> MiniBoardState {
        //Update win/tie state based on last move, returns the state

        let mut fwd_diag = 0; //Diag \
        let mut bkwd_diag = 0; //Diag /
        let mut along_y = [0 as usize; SIZE];
        let mut along_x = [0 as usize; SIZE];

        let mut any_none = false;

        for x in 0..SIZE {
            for y in 0..SIZE {
                let Some(entry) = self.mboard[x][y] else {
                    any_none = true;
                    continue;
                };
                if entry != tile {
                    continue;
                }
                if x == y {
                    fwd_diag += 1;
                }
                if x == (SIZE - 1 - y) {
                    bkwd_diag += 1
                }
                along_y[x] += 1;
                along_x[y] += 1;
            }
        }

        if fwd_diag == SIZE
            || bkwd_diag == SIZE
            || along_x.contains(&SIZE)
            || along_y.contains(&SIZE)
        {
            self.state = MiniBoardState::Win(tile);
        } else if !any_none {
            self.state = MiniBoardState::Tie;
        }

        self.state
    }
}

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
