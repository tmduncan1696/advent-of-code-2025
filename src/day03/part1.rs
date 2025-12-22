use crate::day03::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let out = input
        .into_iter()
        .map(|x| get_max_joltage(x))
        .sum();

    Some(out)
}

fn get_max_joltage(bank: &Vec<usize>) -> usize {
    let mut x: Vec<usize> = bank.clone();
    x.remove(x.len() - 1);
    
    let first_digit = x.iter().max().unwrap();
    let first_digit_position = x.iter().position(|a| a == first_digit).unwrap() + 1;

    let y: Vec<usize> = bank.clone()[first_digit_position..].to_vec();

    let second_digit = y.iter().max().unwrap();

    first_digit * 10 + second_digit
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::day03::input::read;

    #[test]
    fn test_get_max_joltage() {
        assert_eq!(
            get_max_joltage(&vec![1, 2, 3, 4]),
            34
        );

        assert_eq!(
            get_max_joltage(&vec![4,2,3,1]),
            43
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
            Some(357)
        );
    }
}
