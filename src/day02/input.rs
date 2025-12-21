pub const INPUT: &str = include_str!("../input/day02.txt");

pub type Input = Vec<(usize, usize)>;

pub fn read(input: &str) -> Input {
    input
        .trim()
        .split(",")
        .map(|s| str_to_num_pair(s))
        .collect()
}

fn str_to_num_pair(s: &str) -> (usize, usize) {
    let vec: Vec<_> = s
        .split("-")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    let out: (usize, usize) = (vec[0], vec[1]);
    out
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
            (655, 1102)
        );

        let last = input.last().unwrap();
        assert_eq!(
            *last,
            (752508, 837824)
        );
    }

}
