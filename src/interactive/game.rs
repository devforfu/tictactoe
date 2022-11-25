use crate::board::{SIDE_SIZE, Board, BoardIterator, Mark, Position, Status, player_name};
use crate::interactive::input::{Command, UserInput};

pub struct Game {
    board: Board,
    curr_player: bool,
    stopped: bool,
}

impl Game {
    pub fn new() -> Self { Self {board: Board::new(), curr_player: true, stopped: false} }

    pub fn ongoing(&self) -> bool { !self.stopped && self.board.status() == Status::Ongoing }

    pub fn positions(&self, player: bool) -> Vec<Position> {
        let mut iter = BoardIterator::new(&self.board);
        let mut positions = vec![];
        while let Some((pos, mark)) = iter.next() {
            if (player && mark == Mark::First) || (!player && mark == Mark::Second) {
                positions.push(pos);
            }
        }
        positions
    }

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
        if pos.0 >= SIDE_SIZE || pos.1 >= SIDE_SIZE {
            Err(String::from(format!("each coordinate should be < {}", SIDE_SIZE)))
        } else if !self.board.empty(&pos) {
            Err(String::from("cell is not empty"))
        } else {
            Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use crate::interactive::input::BufferedInput;

    struct Replay(Game);

    impl Replay {
        fn new() -> Self { Self { 0: Game::new() } }
        fn run(&mut self, turns: &str) -> String {
            let mut buf = BufferedInput::new(Cursor::new(String::from(turns)));
            self.0.play(&mut buf);
            self.0.status_string()
        }
    }
    
    #[test]
    fn test_new_game() {
        let game = Game::new();

        assert!(game.ongoing());
        assert!(game.curr_player);
    }

    #[test]
    fn test_stopped_early() {
        let mut replay = Replay::new();
        
        replay.run("0,0\n1,1\n2,2\ns");

        assert_eq!(replay.0.status_string(), String::from("stopped early."));
        assert_eq!(replay.0.positions(true), vec![(0, 0), (2, 2)]);
        assert_eq!(replay.0.positions(false), vec![(1, 1)]);
    }

    #[test]
    fn test_tie() {
        assert_eq!(
            run(
                "0,0\n0,1\n0,2\n\
                 1,1\n1,0\n2,0\n\
                 1,2\n2,2\n2,1\n"
            ), 
            String::from("it is a tie!")
        );
    }

    #[test]
    fn test_the_first_player_wins() {
        assert_eq!(run("0,0\n1,0\n0,1\n1,1\n0,2\n"), String::from("the player X wins!"));
    }

    #[test]
    fn test_the_second_player_wins() {
        assert_eq!(run("1,0\n0,0\n1,1\n0,1\n2,2\n0,2\n"), String::from("the player O wins!"));
    }

    #[test]
    fn test_no_turns() {
        let mut replay = Replay::new();

        replay.run("s");

        assert_eq!(replay.0.positions(true), vec![]);
        assert_eq!(replay.0.positions(false), vec![]);
    }

    fn run(turns: &str) -> String {
        let mut replay = Replay::new();
        replay.run(turns)
    }
}