pub const INPUT: &str = include_str!("../input/day00.txt");

pub type Input = String;

pub fn read(input: &str) -> Input {
    input
        .trim()
        .lines()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        /*
        let input = read(INPUT);

        let first = input.first().unwrap();
        assert_eq!(
            *first,
        );

        let last = input.last().unwrap();
        assert_eq!(
            *last,
        );
        */

        assert!(true)
    }

}
