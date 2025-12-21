use crate::day01::input::{Input};
use crate::day01::{Direction, Movement};

pub fn solve(input: &Input) -> Option<usize> {
    let out = input
        .iter()
        .scan(50, |acc, m| {
            *acc = move_dial(*acc, m);
            Some(*acc)
        })
        .filter(|x| *x == 0)
        .count() as usize;

    Some(out)
}

fn move_dial(start: usize, movement: &Movement) -> usize {
    let new_loc: i32 = match movement.direction {
        Direction::Right => start as i32 + movement.amount as i32,
        Direction::Left => start as i32 - movement.amount as i32
    };

    let out: usize = new_loc.rem_euclid(100) as usize;

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
            Some(3)
        )

    }

    #[test]
    fn test_modulo() {
        let a = -18 as i32;
        let b = 100 as i32;

        let out = a.rem_euclid(b);
        assert_eq!(
            out,
            82
        )
    }
}
