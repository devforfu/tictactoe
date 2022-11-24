use std::fmt;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};

pub const SIDE_SIZE: usize = 3;
pub const BOARD_SIZE: usize = SIDE_SIZE * SIDE_SIZE;

pub type Position = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Mark {
    First,
    Second,
    Empty
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    Winner(bool),
    Ongoing,
    Tie,
    Impossible,
}

pub struct Board([Mark; BOARD_SIZE]);

impl Board {
    pub fn new() -> Self { Self {0: [Mark::Empty; BOARD_SIZE]} } 

    pub fn status(&self) -> Status {
        if self.impossible() {
            return Status::Impossible;
        }
        
        for i in 0..SIDE_SIZE {
            for rows in [true, false] {
                if let status @ Status::Winner(_) = self.check(i, rows) {
                    return status
                }
            }
        }

        if let status @ Status::Winner(_) = self.diagonals() {
            return status;
        }
        
        if self.0.iter().any(|x| *x == Mark::Empty) {
            Status::Ongoing
        } else {
            Status::Tie
        }
    }

    pub fn empty(&self, pos: &Position) -> bool { self[*pos] == Mark::Empty }

    pub fn impossible(&self) -> bool {
        let (fst, snd) = self.0.iter().fold((0i8, 0i8), |acc, val| {
            let (i, j) = match val {
                Mark::First => (1, 0),
                Mark::Second => (0, 1),
                _ => (0, 0),
            };
            (acc.0 + i, acc.1 + j)
        });
        (fst - snd).abs() > 1
    }

    fn check(&self, i: usize, rows: bool) -> Status {
        let mut count: HashMap<Mark, usize> = HashMap::new();
        for j in 0..3 {
            let pos = if rows { (i, j) } else { (j, i) };
            let val = self[pos];
            count.entry(val).and_modify(|c| *c += 1).or_insert(1);
        }
        if let Some(3) = count.get(&Mark::First) {
            return Status::Winner(true);
        }
        if let Some(3) = count.get(&Mark::Second) {
            return Status::Winner(false);
        }
        Status::Ongoing
    }

    fn diagonals(&self) -> Status {
        for diagonal in vec![
            vec![(0, 0), (1, 1), (2, 2)],
            vec![(2, 0), (1, 1), (0, 2)],
        ] {
            let mut count: HashMap<Mark, usize> = HashMap::new();
            for pos in diagonal {
                count.entry(self[pos]).and_modify(|c| *c += 1).or_insert(1);
            }
            if let Some(3) = count.get(&Mark::First) {
                return Status::Winner(true);
            }
            if let Some(3) = count.get(&Mark::Second) {
                return Status::Winner(false);
            }
        }
        Status::Ongoing
    }
}

impl TryFrom<&str> for Board {
    type Error = &'static str;
    
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut board = Board::new();
        let mut iter = value.split('|').take(SIDE_SIZE);
        let mut line_no = 0;
        while let Some(line) = iter.next() {
            if line.len() != SIDE_SIZE {
                return Err("failed");
            }
            for (i, char) in line.chars().enumerate() {
                board[(line_no, i)] = match char {
                    'x' => Mark::First,
                    'o' => Mark::Second,
                    '.' => Mark::Empty,
                    _   => return Err("failed"),
                };
            }
            line_no += 1;
        }
        Ok(board)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("+---+\n")?;
        for i in 0..=2 {
            formatter.write_str("|")?;
            for j in 0..=2 {
                let char = match self[(i, j)] {
                    Mark::Empty => '.',
                    Mark::First => 'x',
                    Mark::Second => 'o',
                };
                write!(formatter, "{}", char)?;
            }
            write!(formatter, "|\n")?;
        }
        formatter.write_str("+---+\n")?;
        Ok(())
    }
}

impl Index<Position> for Board {
    type Output = Mark;
    fn index(&self, pos: Position) -> &Self::Output {
        let (i, j) = check_bounds(pos);
        &self.0[i*SIDE_SIZE + j]
    }
}

impl IndexMut<Position> for Board {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        let (i, j) = check_bounds(pos);
        &mut self.0[i*SIDE_SIZE + j]
    }
}

fn check_bounds(pos: Position) -> Position {
    if pos.0 >= SIDE_SIZE || pos.1 >= SIDE_SIZE { 
        panic!("wrong board index: {:#?}", pos);
    }
    pos
}

pub fn player_name(first: bool) -> String {
    String::from(if first {"X"} else {"O"})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let empty = Board::new();
        
        let formatted = format!("{}", empty);

        assert_eq!(formatted, 
                   "+---+\n\
                    |...|\n\
                    |...|\n\
                    |...|\n\
                    +---+\n");
    }

    #[test]
    fn test_board_from_string() {
        let board = Board::try_from("...|.x.|..o").unwrap();

        assert_eq!(board[(0, 0)], Mark::Empty);
        assert_eq!(board[(0, 1)], Mark::Empty);
        assert_eq!(board[(0, 2)], Mark::Empty);
        assert_eq!(board[(1, 0)], Mark::Empty);
        assert_eq!(board[(1, 1)], Mark::First);
        assert_eq!(board[(1, 2)], Mark::Empty);
        assert_eq!(board[(2, 0)], Mark::Empty);
        assert_eq!(board[(2, 1)], Mark::Empty);
        assert_eq!(board[(2, 2)], Mark::Second);
    }

    #[test]
    fn test_board_manually_set() {
        let mut board = Board::new();

        board[(0, 0)] = Mark::First;
        board[(1, 1)] = Mark::Second;
        board[(2, 2)] = Mark::First;

        assert_eq!(format!("{}", board),
                  "+---+\n\
                   |x..|\n\
                   |.o.|\n\
                   |..x|\n\
                   +---+\n");
    }

    #[test]
    fn test_board_fails_to_create_from_invalid_strings() {
        let strings = vec!["......", "x|x|x", "...|xyz|...", "xxx...ooo"];

        let mut results = strings.iter().map(|x| Board::try_from(*x));

        assert!(results.all(|r| r.is_err()));
    }

    #[test]
    fn test_board_in_impossible_state() {
        let board = Board::try_from("xxx|xxx|xxx").unwrap();

        assert_eq!(board.status(), Status::Impossible);
    }
}