use crate::board::Board;

pub enum Message<'a> {
    Welcome,
    BoardState(&'a Board),
    UnknownCommand,
    GameOver(&'a Board, String),
}

pub trait Render {
    fn draw(&self, message: Message);
}

pub struct ConsoleRender;

impl Render for ConsoleRender {
    fn draw(&self, message: Message) {
        println!("{}", match message {
            Message::Welcome => String::from(
                "Welcome to the Tic-Tac-Toe game!\n\
                 Commands:\n\
                 (1) put mark at x row and y column: x,y\n\
                 (2) stop the game: [s]top (or Ctrl-C)\n"
            ),
            Message::BoardState(board) => format!("{}", board),
            Message::UnknownCommand => String::from("Unknown command, try again!"),
            Message::GameOver(board, outcome) => format!(
                "The game is over: {}\nThe final board's state:\n{}",
                outcome, board,
            ),
        });
    }
}