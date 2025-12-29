pub const INPUT: &str = include_str!("../input/day05.txt");

pub type Input = String;

pub fn read(input: &str) -> Input {
    input.to_owned()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        let input = read(INPUT);

        let first = input.lines().collect::<Vec<_>>()[0];
        assert_eq!(
            first,
            "474206951121632-478696506672479"
        );

        let last = input.lines().last().unwrap();
        assert_eq!(
            last,
            "339024629171797"
        );
    }

}
