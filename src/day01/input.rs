use std::str::FromStr;

use crate::day01::Movement;

pub type Input = Vec<Movement>;

const INPUT: &str = include_str!("../input/day01.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split("\n")
        .map(|input| Movement::from_str(input))
        .collect::<Result<_, _>>()
        .expect("Could not parse input")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::day01::Direction;

    #[test]
    fn test_read() {
        let input = read();

        let first = input.first().unwrap();
        assert_eq!(
            *first,
            Movement {
                direction: Direction::Right,
                amount: 32
            }
        );

        let last = input.last().unwrap();
        assert_eq!(
            *last,
            Movement {
                direction: Direction::Right,
                amount: 40
            }
        );

    }
}
