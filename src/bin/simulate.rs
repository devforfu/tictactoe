use tictactoe::simulator::simulate;

fn main() {
    let boards = simulate(10, false);
    for board in boards.iter() {
        if board.impossible() {
            println!("Impossible board state encountered!");
        }
        println!("Status: {:?}", board.status());
        println!("{}", board);
    }
}