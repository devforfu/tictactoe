use crate::board::{Board, Mark, Position, Status, player_name};
use crate::interactive::input::{Command, UserInput};

pub struct Game {
    board: Board,
    curr_player: bool,
    stopped: bool,
}

impl Game {
    pub fn new() -> Self { Self {board: Board::new(), curr_player: true, stopped: false} }

    pub fn ongoing(&self) -> bool { !self.stopped && self.board.status() == Status::Ongoing }

    pub fn play(&mut self, input: &mut dyn UserInput) {
        println!("Welcome to the Tic-Tac-Toe game!\n\
                 Commands:\n\
                 (1) put mark at x row and y column: x,y\n\
                 (2) stop the game: [s]top (or Ctrl-C)\n");

        println!("{}", self.board);
        
        while self.ongoing() {
            if let Some(cmd) = input.read() {
               self.turn(&cmd);
            } else {
                println!("Unknown command, try again!");
            }
        }

        println!("The game is over: {}\n\
                 The final board's state:\n{}", 
                 self.status_string(), self.board);
    }
    
    fn turn(&mut self, cmd: &Command) {
        let next_player = match cmd {
            Command::Stop => { 
                self.stopped = true;
                false
            }
            Command::Turn(pos) => {
                if let Err(error) = self.valid_turn(&pos) {
                    println!("Impossible turn [{}, {}]: {}; try again!", pos.0, pos.1, error);
                    false
                } else {
                    self.board[*pos] = if self.curr_player {
                        Mark::First
                    } else {
                        Mark::Second
                    };
                    true
                }
            }
        };
        if next_player {
            self.curr_player = !self.curr_player;
            println!("{}", self.board);
        };
    }

    fn valid_turn(&self, pos: &Position) -> Result<(), String> { 
        if self.board.empty(&pos) {
            Ok(())
        } else {
            Err(String::from("cell is not empty"))
        }
    }

    fn status_string(&self) -> String {
        match self.board.status() {
            Status::Winner(player) => String::from(format!("the player {} wins!", player_name(player))),
            Status::Tie => String::from("it is a tie!"),
            Status::Ongoing => String::from("stopped early."),
            _ => unreachable!(),
        }
    }
}
