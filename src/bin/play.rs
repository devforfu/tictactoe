use tictactoe::interactive::{game::Game, input::Keyboard};
use tictactoe::render::ConsoleRender;

fn main() {
    let mut input = Keyboard;
    let mut game = Game::new();
    let output = ConsoleRender;
    game.play(&mut input, &output);
}