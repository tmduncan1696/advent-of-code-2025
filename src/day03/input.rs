pub const INPUT: &str = include_str!("../input/day03.txt");

pub type Input = Vec<Vec<usize>>;

pub fn read(input: &str) -> Input {
    input
        .trim()
        .lines()
        .map(|x| string_to_num_vec(x))
        .collect()
}

fn string_to_num_vec(s: &str) -> Vec<usize> {
    s.chars().map(|x| x.to_digit(10).unwrap() as usize).collect()
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
            vec![3,2,3,3,4,3,4,2,2,3,3,5,2,2,5,3,3,2,2,2,4,4,3,2,3,5,6,2,4,1,3,2,2,2,3,2,2,4,2,2,5,2,2,6,2,2,3,1,2,4,2,2,3,3,2,1,2,3,2,2,3,1,2,3,2,3,5,4,2,2,2,1,2,1,9,6,3,2,3,2,2,2,3,3,2,2,3,2,3,3,2,2,4,2,2,2,2,2,1,1]
        );

        let last = input.last().unwrap();
        assert_eq!(
            *last,
            vec![1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,2,1,2,2,3,2,2,1,2,2,2,1,2,2,3,2,1,1,2,2,2,3,1,2,2,2,2,2,2,2,2,2,2,1,3,2,2,2,1,2,2,3,3,2,6,4,1,2,2,4,4,2,2,2,2,1,7,2,2,3,2,1,2,3,3,2,1,2,1,2,2,9,2,3,2,2,2,2,5,2,2,2]
        );
    }

}
