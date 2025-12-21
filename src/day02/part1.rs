use crate::day02::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let out = input
        .into_iter()
        .flat_map(|x| get_invalid_ids(*x))
        .sum();

    Some(out)
}

fn is_invalid(x: usize) -> bool {
    let s = x.to_string();
    let s_half_len = &s.len() / 2;
    let s_half = &s[0..s_half_len];
    s == s_half.to_owned() + s_half
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
    fn test_solve() {
        let input_str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let input = read(input_str);

        let out = solve(&input);

        assert_eq!(
            out,
            Some(1227775554)
        );
        
    }
}
