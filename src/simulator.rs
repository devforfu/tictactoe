use std::collections::HashSet;

use rand::prelude::*;
use crate::board::{player_name, Board, Position, Mark, Status, BOARD_SIZE, SIDE_SIZE};

/// Simulates N games.
///
/// Each simulation runs a random sequence of turns for a newly created game board.
/// The results returns as a vector of final states.
pub fn simulate(n: usize, logged: bool) -> Vec<Board> {
    let mut rng = thread_rng();
    (0..n).map(move |_| simulate_one(&mut rng, logged)).collect()
}

/// Simulates a single game and returns the final state of a board.
pub fn simulate_one(rng: &mut ThreadRng, logged: bool) -> Board {
    let mut board = Board::new();
    let mut turns: HashSet<Position> = HashSet::new();
    let mut first = true;

    for turn in 1..=BOARD_SIZE {
        let pos = make_random_turn(&mut turns, rng);
        if logged { println!("Making turn #{}: {} at [{}, {}]", turn, player_name(first), pos.0, pos.1); }
        
        board[pos] = if first {Mark::First} else {Mark::Second};
        first = !first;
        if logged { println!("{}", board); }

        let status = board.status();
        if logged { println!("{}", status_message(&status)); }
        if status != Status::Ongoing { break; }
    }

    board
}

fn make_random_turn(turns: &mut HashSet<Position>, rng: &mut ThreadRng) -> Position {
    loop {
        let i = rng.gen_range(0..SIDE_SIZE);
        let j = rng.gen_range(0..SIDE_SIZE);
        let pos = (i, j);
        if turns.contains(&pos) { continue }
        turns.insert(pos);
        break pos
    }
}

fn status_message(status: &Status) -> String {
    match status {
        Status::Ongoing => String::from("The game keeps going..."),
        Status::Winner(player) => format!("The player {} wins!", player_name(*player)),
        Status::Tie => String::from("It is a tie, no winners."),
        Status::Impossible => String::from("Impossible board state!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation() {
        let boards = simulate(10, false);

        let impossible: Vec<&Board> = boards.iter().filter(|x| x.impossible()).collect();

        assert!(boards.len() == 10);
        assert!(impossible.len() == 0);
    }
}