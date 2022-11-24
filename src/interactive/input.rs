use std::io::{stdin, BufRead};
    
use crate::board::Position;

#[derive(Debug)]
pub enum Command {
    Turn(Position),
    Stop,
}

#[derive(Debug)]
pub enum ParserState {
    Start,
    CoordX,
    CoordY,
}

pub trait UserInput {
    fn read(&mut self) -> Option<Command>;
}

pub struct BufferedInput<T> {
    buffer: T
}

impl<T: BufRead> BufferedInput<T> {
    pub fn new(buffer: T) -> Self { Self { buffer } }
    
    fn read_from_buffer(&mut self) -> Option<String> {
        let mut buf = String::new();
        if let Err(error) = self.buffer.read_line(&mut buf) {
            println!("failed to read input: {}", error);
            None
        } else {
            Some(buf)
        }    
    }
}

impl<T: BufRead> UserInput for BufferedInput<T> {
    fn read(&mut self) -> Option<Command> {
        self.read_from_buffer().map(|buf| parse(&buf)).flatten()
    }
}

pub struct Keyboard;

impl UserInput for Keyboard {
    fn read(&mut self) -> Option<Command> {
        BufferedInput::new(stdin().lock()).read()
    }
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
    use std::io::Cursor;

    #[test]
    fn test_parsing_valid_command() {
        let valid_turns = strings(vec!["1,2\n", "0,0\n", "stop", "s"]);

        let parsed_commands: Vec<Command> = valid_turns.iter().map(parse).flatten().collect();
        
        assert_eq!(parsed_commands.len(), valid_turns.len());
    }

    #[test]
    fn test_parsing_invalid_input_into_none() {
        let invalid_values = strings(vec!["1.2", "1,1,1", "0", "x"]);

        let parsed_commands: Vec<Command> = invalid_values.iter().map(parse).flatten().collect();

        assert_eq!(parsed_commands.len(), 0);
    }

    #[test]
    fn test_reading_from_buffer() {
        let expected = String::from("abc");
        let mut keyboard = BufferedInput::new(Cursor::new(expected.clone()));

        let result = keyboard.read_from_buffer();

        assert_eq!(result, Some(expected));
    }

    #[test]
    fn test_reading_command() {
        let mut keyboard = BufferedInput::new(Cursor::new(String::from("0,1\n")));

        let command = keyboard.read();

        assert_matches!(command, Some(Command::Turn((0, 1))));
    }

    fn strings(vec: Vec<&str>) -> Vec<String> {
        vec.into_iter().map(String::from).collect()
    }
}