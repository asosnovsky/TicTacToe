use std::io;
use board::BoardCellState;
pub mod utils;
pub mod board;

pub fn single_player() {
    start_game("Multi Player", 1);
}

pub fn multi_player() {
    start_game("Multi Player", 2);
}

pub fn humanless() {
    start_game("AI Game", 0);
}

fn start_game(game_title: &str, num_humans: i32) {
    
    let mut p1 = "PC1".to_string();
    let mut p2 = "PC2".to_string();
    if num_humans == 1 {
        p1 = utils::input("Please input name for player 1: ".to_string());
        p2 = "PC".to_string();
    }   else if num_humans == 2 {
        p1 = utils::input("Please input name for player 1: ".to_string());
        p2 = utils::input("Please input name for player 2: ".to_string());
    }
    
    utils::clear_console();
    let players = flip_coin(p1, p2, num_humans > 0, num_humans == 2);
    println!("Starting {} Mode",game_title);
    println!("  - Player 1: {} ({})", players[0].name, players[0].get_text_sym());
    println!("  - Player 2: {} ({})", players[1].name, players[1].get_text_sym());
    println!();
    utils::sleep(500);
    
    let mut game = Game::new(
        players[0].clone(),
        players[1].clone(),
        3,
    );
    game.board.print();
    utils::sleep(500);
    game.next_move();
}

fn flip_coin(p1name: String, p2name: String, p1is_human: bool, p2is_human: bool) -> [Player; 2] {
    print!("Fliping coin to see who goes first"); io::Write::flush(&mut io::stdout()).expect("flush failed!");
    utils::sleep(400); 
    print!("."); io::Write::flush(&mut io::stdout()).expect("flush failed!");
    utils::sleep(400);
    print!("."); io::Write::flush(&mut io::stdout()).expect("flush failed!");
    utils::sleep(400);
    println!("."); io::Write::flush(&mut io::stdout()).expect("flush failed!");
    utils::sleep(400);
    if rand::random() {
        return [
            Player{ name: p1name, is_human: p1is_human, sym: BoardCellState::X },
            Player{ name: p2name, is_human: p2is_human, sym: BoardCellState::O },
        ]
    }   else {
        return [
            Player{ name: p2name, is_human: p2is_human, sym: BoardCellState::X },
            Player{ name: p1name, is_human: p1is_human, sym: BoardCellState::O },
        ]
    }
} 

struct Game {
    board: board::Board,
    player1: Player,
    player2: Player,
    current_player: Player,
}

#[derive(Clone)]
struct Player {
    name: String,
    sym: BoardCellState,
    is_human: bool,
}

impl Player {
    pub fn get_text_sym(&self) -> String {
        match self.sym {
            BoardCellState::EMPTY => return String::from("ERROR"),
            BoardCellState::X => return String::from("X"),
            BoardCellState::O => return String::from("O"),
        }
    }
}

impl Game {
    pub fn new(player1: Player, player2: Player, size: usize) -> Self {
        if rand::random() {
            return Game {
                player1: player1.clone(),
                player2: player2.clone(),
                current_player: player1.clone().clone(),
                board: board::new(size),
            }
        }   else {
            return Game {
                player1: player1.clone(),
                player2: player2.clone(),
                current_player: player2.clone(),
                board: board::new(size),
            }
        }
    }
    pub fn next_move(&mut self) {
        if self.board.get_num_open_spots() == 0 {
            utils::clear_console();
            println!(" > Its a tie!");
            return;
        }
        if self.current_player.is_human {
            let coords = self.ask_coords();
            let res = self.board.set(
                coords[0], 
                coords[1], 
                self.current_player.sym
            );
            utils::clear_console();
            self.board.print();
            utils::sleep(500);
            self.switch_players(res);
        }   else {
            println!("{} is thinking... 🤔", self.current_player.name);
            utils::sleep(1000);
            let res = self.board.set_random(self.current_player.sym);
            utils::clear_console();
            self.board.print();
            utils::sleep(500);
            self.switch_players(res);
        }
    }
    fn ask_coords(&mut self) -> [usize; 2] {
        let uinput = utils::input(format!("{} > ({}) Enter where (row,col): ", self.current_player.name, self.current_player.get_text_sym()));
        if uinput.contains(',') {
            let coords: Vec<&str> = uinput.split(',').collect();
            let ridx = coords[0].parse::<usize>();
            let cidx = coords[1].parse::<usize>();
            if ridx.is_ok() & cidx.is_ok() {
                let coords = [ridx.unwrap(), cidx.unwrap()];
                if (coords[0] < self.board.size) & (coords[1] < self.board.size) {
                    if self.board.check_legal_move_then_remove(coords[0], coords[1]) {
                        return coords;
                    }   else {
                        println!("Spot already occupied!");
                    }
                }   else {
                    println!("Coordinates must be between 0 and {}", self.board.size-1);
                }
            }   else {
                println!("Invalid coordinates, must be numeric integers");
            }
        }   else {
            println!("Invalid coordinates, please enter the command in the following form `row, col`")
        };
        utils::sleep(500);
        utils::clear_console();
        self.board.print();
        return self.ask_coords();
    }
    fn switch_players(&mut self, res: Option<BoardCellState>) {
        match res {
            Some(board::BoardCellState::X) => {
                utils::print_baloons();
                println!("{} (X) has won the game!", self.current_player.name);
                utils::sleep(2000);
            },
            Some(board::BoardCellState::O) => {
                utils::print_baloons();
                println!("{} (O) has won the game!", self.current_player.name);
                utils::sleep(2000);
            },
            _ => {
                if self.current_player.sym == self.player1.sym {
                    self.current_player = self.player2.clone();
                }   else {
                    self.current_player = self.player1.clone();
                }
                self.next_move();
            }
        }
    } 
}
