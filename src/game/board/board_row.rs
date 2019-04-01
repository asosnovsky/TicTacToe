#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum BoardCellState {
    EMPTY,
    X,
    O,
}

pub fn new(size: usize) -> BoardRow {
    let mut cells = Vec::new();
    for _ in 0..size {
        cells.push(BoardCellState::EMPTY);
    };
    return BoardRow {
        size: size,
        cells: cells,
        owner: BoardCellState::EMPTY,
        win_counts: 0,
    }
}

pub struct BoardRow {
    cells: Vec<BoardCellState>,
    size: usize,
    owner: BoardCellState,
    win_counts: i32,
}

impl BoardRow {
    pub fn set(&mut self, cidx: usize, value: BoardCellState) {
        if cidx >= self.size {
            panic!("Column index is higher than allowed")
        } ;
        self.cells[cidx] = value;
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
    pub fn get(&self, cidx: usize) -> String {
        match self.cells[cidx] {
            BoardCellState::EMPTY => return String::from(" "),
            BoardCellState::X => return String::from("X"),
            BoardCellState::O => return String::from("O"),
        }
    }
    pub fn has_won(&self) -> bool {
        return self.win_counts == (self.size as i32);
    }
    pub fn get_winner(&self) -> BoardCellState {
        return self.owner;
    }
}
