use tictactoe::interactive::{game::Game, input::Keyboard};

fn main() {
    let mut input = Keyboard;
    let mut game = Game::new();
    game.play(&mut input);
}