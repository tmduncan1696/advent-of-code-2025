use std::sync::LazyLock;

use fancy_regex::Regex;

use crate::day02::input::Input;


static PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\1+$").unwrap());

pub fn solve(input: &Input) -> Option<usize> {
    let out = input
        .into_iter()
        .flat_map(|x| get_invalid_ids(*x))
        .sum();

    Some(out)
}

fn is_invalid(x: usize) -> bool {
    let s = x.to_string();
    PATTERN.is_match(&s).unwrap()
}

fn get_invalid_ids(range: (usize, usize)) -> Vec<usize> {
    (range.0..=range.1)
        .filter(|x| is_invalid(*x))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::day02::input::read;

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(123123123));
        assert!(is_invalid(1324132413241324));
        assert!(is_invalid(11111));
        assert!(!is_invalid(1234));
        assert!(!is_invalid(998));
    }

    #[test]
    fn test_get_invalid_ids() {
        let input_str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let input = read(input_str);

        let out: Vec<usize> = input
            .into_iter()
            .flat_map(|x| get_invalid_ids(x))
            .collect();

        assert_eq!(
            out,
            vec![11, 22, 99, 111, 999, 1010, 1188511885, 222222, 446446, 38593859, 565656, 824824824, 2121212121]
        );
    }

    #[test]
    fn test_solve() {
        let input_str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let input = read(input_str);

        let out = solve(&input);

        assert_eq!(
            out,
            Some(4174379265)
        );
        
    }
}
