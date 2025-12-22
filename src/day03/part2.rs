use crate::day03::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let out = input
        .into_iter()
        .map(|x| get_max_joltage(x, 12))
        .sum();

    Some(out)
}

fn get_max_joltage(bank: &Vec<usize>, size: usize) -> usize {

    let bank_length = bank.len();

    let mut nums = Vec::new();

    let mut first_position_possible = 0;

    for s in (0..size).rev() {
        let last_position_possible = bank_length - s;
        let x: Vec<usize> = bank.clone()[first_position_possible..last_position_possible].to_vec();
        let digit = x.clone().into_iter().max().unwrap();
        first_position_possible += x.into_iter().position(|a| a == digit).unwrap() + 1;
        nums.push(digit);
    };

    nums
        .into_iter()
        .reduce(|a, b| concatenate_nums(a, b))
        .unwrap_or(0)
}

fn concatenate_nums(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::day03::input::read;

    #[test]
    fn test_get_max_joltage() {
        assert_eq!(
            get_max_joltage(&vec![1, 2, 3, 4], 2),
            34
        );

        assert_eq!(
            get_max_joltage(&vec![4,2,3,1], 3),
            431
        );
    }

    #[test]
    fn test_solve() {
        let input_str = "\
987654321111111
811111111111119
234234234234278
818181911112111";

        let input = read(input_str);

        let out = solve(&input);

        assert_eq!(
            out,
            Some(3121910778619)
        );
    }
}
