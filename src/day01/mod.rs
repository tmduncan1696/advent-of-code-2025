use std::fmt;
use std::str::FromStr;

pub mod input;
pub mod part1;
pub mod part2;

#[derive(Debug, PartialEq)]
pub enum Direction {
    Right,
    Left
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            _ => Err(ParseDirectionError)
        }
    }
}

#[derive(Debug)]
pub struct ParseDirectionError;

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid value for Direction")
    }
}

#[derive(Debug, PartialEq)]
pub struct Movement {
    pub direction: Direction,
    pub amount: usize
}

impl FromStr for Movement {
    type Err = ParseMovementError;

    fn from_str(input: &str) -> Result<Movement, Self::Err> {
        let data = input.split_at(1);
        let dir = data.0;
        let direction = match Direction::from_str(dir) {
            Ok(val) => val,
            Err(_) => return Err(ParseMovementError)
        };
        let amount = match input[1..].parse::<usize>() {
            Ok(val) => val,
            Err(_) => return Err(ParseMovementError)
        };

        let out = Movement {
            direction,
            amount
        };

        Ok(out)
    }
}

#[derive(Debug)]
pub struct ParseMovementError;

impl fmt::Display for ParseMovementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid value for Movement")
    }
}

pub fn run() -> (Option<usize>, Option<usize>) {
    let input = input::read();
    (
        part1::solve(&input),
        part2::solve(&input)
    )
}
