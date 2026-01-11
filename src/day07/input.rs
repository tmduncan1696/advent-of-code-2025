pub const INPUT: &str = include_str!("../input/day07.txt");

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
            "......................................................................S......................................................................".to_string()
        );

        let last = input.last().unwrap();
        assert_eq!(
            *last,
            ".............................................................................................................................................".to_string()
        );
    }

}
