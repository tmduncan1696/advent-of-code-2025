pub const INPUT: &str = include_str!("../input/day08.txt");

pub type Input = Vec<String>;

pub fn read(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|s| s.to_owned())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        let input = read(INPUT);

        let first = input.first().unwrap();
        assert_eq!(
            *first,
            "59149,72072,58720".to_string()
        );

        let last = input.last().unwrap();
        assert_eq!(
            *last,
            "41387,24892,7873".to_string()
        );

        assert!(true)
    }

}
