use crate::day01::{Direction, Movement};
use crate::day01::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let out = input
        .into_iter()
        .scan((50, 0), |acc, m| {
            let start = acc.0;
            *acc = move_dial(start, m);
            Some(*acc)
        })
        .map(|x| x.1)
        .sum::<usize>();

    Some(out)
}

fn move_dial(start: usize, movement: &Movement) -> (usize, usize) {
    let new_loc: isize = match movement.direction {
        Direction::Right => start as isize + movement.amount as isize,
        Direction::Left => start as isize - movement.amount as isize
    };

    let mut num_passes_zero = if new_loc <= 0 {
        ((&new_loc.abs() + 100) / 100) as usize
    } else {
        (&new_loc / 100) as usize
    };

    num_passes_zero -= if start == 0 && movement.direction == Direction::Left && num_passes_zero > 0 { 1 } else { 0 };

    let out: (usize, usize) = (new_loc.rem_euclid(100) as usize, num_passes_zero);

    out
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_solve() {
        let input_string = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let input: Input = input_string
            .trim()
            .split("\n")
            .map(|input| Movement::from_str(input))
            .collect::<Result<_, _>>()
            .expect("Could not parse input");

        let out = solve(&input);

        assert_eq!(
            out,
            Some(6)
        )

    }

    #[test]
    fn test_integer_division() {
        assert_eq!(
            101 / 100,
            1
        );

        assert_eq!(
            82 / 100,
            0
        );
    }

    #[test]
    fn test_multiple_rounds() {
        let input = vec![
            Movement {
                direction: Direction::Right,
                amount: 1000
            }
        ];

        let out = solve(&input);

        assert_eq!(
            out,
            Some(10)
        );
    }

    #[test]
    fn test_negative_rounds() {
        let input = vec![
            Movement {
                direction: Direction::Left,
                amount: 200
            }
        ];

        let out = solve(&input);

        assert_eq!(
            out,
            Some(2)
        );
    }

    #[test]
    fn test_land_on_zero1() {
        let input = vec![
            Movement {
                direction: Direction::Right,
                amount: 50
            }
        ];

        let out = solve(&input);

        assert_eq!(
            out,
            Some(1)
        );
    }

    #[test]
    fn test_land_on_zero2() {
        let input = vec![
            Movement {
                direction: Direction::Left,
                amount: 50
            }
        ];

        let out = solve(&input);

        assert_eq!(
            out,
            Some(1)
        );
    }

    #[test]
    fn test_start_land_zero() {
        let input = vec![
            Movement {
                direction: Direction::Left,
                amount: 50
            },
            Movement {
                direction: Direction::Right,
                amount: 100
            }
        ];

        let out = solve(&input);

        assert_eq!(
            out,
            Some(2)
        );
    }

    #[test]
    fn another_test() {
        let input = vec![
            Movement {
                direction: Direction::Left,
                amount: 50
            },
            Movement {
                direction: Direction::Right,
                amount: 101
            }
        ];

        let out = solve(&input);

        assert_eq!(
            out,
            Some(2)
        );
    }
}
