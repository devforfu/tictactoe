use std::io::{stdin, BufRead};

use crate::board::{Board, Mark, Position, Status};

pub struct Game {
    board: Board,
    first_player_turn: bool,
}

impl Game {
    pub fn new() -> Self { Self {board: Board::new(), first_player_turn: true} }

    pub fn ongoing(&self) -> bool { self.board.status() == Status::Ongoing }

    pub fn play(&mut self) {
        println!("Welcome to the Tic-Tac-Toe game!\n\
                 Commands:\n\
                 (1) put mark at x row and y column: x,y\n\
                 (2) stop the game: [s]top (or Ctrl-C)\n");

        let mut curr_player = self.first_player_turn;

        while self.ongoing() {
            println!("{}", self.board);
            
            let mut buf = String::new();

            if let Err(error) = stdin().lock().read_line(&mut buf) {
                println!("failed to read input: {}", error);
                break;
            }

            match parse(&buf) {
                Some(Command::Stop) => { break; }
                Some(Command::Turn(pos)) => {
                    self.board[pos] = if curr_player {
                        Mark::First
                    } else {
                        Mark::Second
                    };
                    curr_player = !curr_player;
                }
                _ => println!("Unknown command: {:#?}; try again!", buf),
            }
        }

        println!("The game is over! The final state is:");
        println!("{}", self.board);
    }
}

#[derive(Debug)]
enum Command {
    Turn(Position),
    Stop,
}

#[derive(Debug)]
enum ParserState {
    Start,
    CoordX,
    CoordY,
}

fn parse(buf: &String) -> Option<Command> {
    let mut state = ParserState::Start;
    let mut number: usize = 0;
    let mut x: usize = 0;

    for char in buf.chars() {
        match state {
            ParserState::Start => {
                match char {
                    '0'..='9' => {
                        state = ParserState::CoordX;
                        number = 10*number + char.to_digit(10).unwrap() as usize;
                    },
                    's' => {
                        return Some(Command::Stop);
                    },
                    _ => {
                        return None;
                    }
                }
            },
            ParserState::CoordX => {
                match char {
                    '0'..='9' => {
                        number = 10*number + char.to_digit(10).unwrap() as usize;
                    },
                    ',' => {
                        x = number;
                        number = 0;
                        state = ParserState::CoordY;
                    },
                    _ => { return None; }
                }
            },
            ParserState::CoordY => {
                match char {
                    '0'..='9' => {
                        number = 10*number + char.to_digit(10).unwrap() as usize;
                    },
                    '\n' => {
                        let y = number;
                        return Some(Command::Turn((x, y)));
                    },
                    _ => { return None; }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();

        assert_eq!(game.ongoing(), true);
    }

    #[test]
    fn test_parsing_coordinate_command_one_digit() {
        let buf = String::from("1,2\n");

        let cmd = parse(&buf);

        assert_matches!(cmd, Some(Command::Turn((1, 2))));
    }

    #[test]
    fn test_parsing_coordinate_command_two_digits() {
        let buf = String::from("15,25\n");

        let cmd = parse(&buf);

        assert_matches!(cmd, Some(Command::Turn((15, 25))));
    }

    #[test]
    fn test_parsing_stop() {
        let buf = String::from("s");

        let cmd = parse(&buf);

        assert_matches!(cmd, Some(Command::Stop));
    }

    #[test]
    fn test_parsing_invalid_command() {
        let buf = String::from("unknown");

        let cmd = parse(&buf);

        assert_matches!(cmd, None);
    }
}