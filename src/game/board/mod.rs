use rand::distributions::{Distribution, Uniform};

pub use board_row::BoardCellState;
mod board_row;

pub fn new(size: usize) -> Board {
    return Board::new(size);
}

pub struct Board {
    rows: Vec<board_row::BoardRow>,
    pub size: usize,
    winstates_col: Vec<WinState>,
    winstate_ldiag: WinState,
    winstate_rdiag: WinState,
    open_spots: Vec<[usize; 2]>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut rows = Vec::new();
        let mut col_ws = Vec::new();
        for _ in 0..size {
            rows.push(board_row::new(size));
            col_ws.push(WinState{ owner: BoardCellState::EMPTY, win_counts: 0 });
        };
        let mut b = Board{
            rows: rows,
            size: size,
            winstates_col: col_ws,
            winstate_ldiag: WinState{ owner: BoardCellState::EMPTY, win_counts: 0 },
            winstate_rdiag: WinState{ owner: BoardCellState::EMPTY, win_counts: 0 },
            open_spots: vec![],
        };
        b.recompute_open_spots();
        return b;
    }
    pub fn print(&self) {
        for ridx in 0..self.size {
            let r = &self.rows[ridx];
            for cidx in 0..self.size {
                print!(" ");
                print!("{}", r.get(cidx));
                if cidx < self.size-1 {
                    print!(" |");
                }
            }
            println!();
            if ridx < self.size-1 {
                println!("---|---|---");
            }
        }
        println!();
    }
    pub fn set(&mut self, ridx: usize, cidx: usize, value: BoardCellState) -> Option<BoardCellState> {
        if ridx >= self.size {
            panic!("Row index is higher than allowed")
        };
        self.rows[ridx].set(cidx, value);
        if self.rows[ridx].has_won() {
            return Some(self.rows[ridx].get_winner());
        }
        self.winstates_col[cidx].update(value);
        if self.winstates_col[cidx].has_won(self.size as i32) {
            return Some(self.winstates_col[cidx].get_winner());
        }
        if cidx == ridx {
            self.winstate_ldiag.update(value);
            if self.winstate_ldiag.has_won(self.size as i32) {
                return Some(self.winstate_ldiag.get_winner());
            }
        }
        if cidx == self.size-ridx-1 {
            self.winstate_rdiag.update(value);
            if self.winstate_rdiag.has_won(self.size as i32) {
                return Some(self.winstate_rdiag.get_winner());
            }
        }
        return None;
    }
    pub fn set_random(&mut self, value: BoardCellState) -> Option<BoardCellState> {
        let between = Uniform::from(0..self.open_spots.len());
        let mut rng = rand::thread_rng();
        let sidx = between.sample(&mut rng) as usize;
        let [ridx, cidx] = self.open_spots[sidx];
        self.open_spots.remove(sidx);
        return self.set(ridx, cidx, value);
    }
    pub fn get_num_open_spots(&self) -> usize {
        return self.open_spots.len();
    }
    pub fn check_legal_move_then_remove(&mut self, ridx: usize, cidx: usize) -> bool {
        let mut lidx: Option<usize> = None;
        for idx in 0..self.open_spots.len() {
            if (self.open_spots[idx][0] == ridx) & (self.open_spots[idx][1] == cidx) {
                lidx = Some(idx);
                break;
            }
        }
        match lidx {
            Some(i) => {
                self.open_spots.remove(i);
                return true;
            },
            None => return false,
        }
    }
    fn recompute_open_spots(&mut self) {
        let mut coords: Vec<[usize; 2]> = Vec::new();
        for ridx in 0..self.size {
            for cidx in 0..self.size {
                if self.rows[ridx].get(cidx) == " " {
                    coords.push([ridx as usize, cidx as usize]);
                }
            }
        }
        self.open_spots = coords;
    }
}

struct WinState {
    owner: BoardCellState,
    win_counts: i32,
}

impl WinState {
    fn update(&mut self, value: BoardCellState) {
        if self.win_counts > -1 {
            if self.owner == BoardCellState::EMPTY {
                self.owner = value;
                self.win_counts = 1;
            } else if self.owner == value {
                self.win_counts += 1;
            } else if self.owner != value {
                self.win_counts = -1;
            }
        }
    }
    pub fn has_won(&self, size: i32) -> bool {
        return self.win_counts == size;
    }
    pub fn get_winner(&self) -> BoardCellState {
        return self.owner;
    }
}