use std::ops::RangeInclusive;

use crate::day05::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let inventory = InventorySystem::from(input);

    let out = inventory
        .get_fresh_ids()
        .len();

    Some(out)
}

#[derive(Clone)]
pub struct InventorySystem {
    pub fresh: Vec<RangeInclusive<usize>>,
    pub available: Vec<usize>
}

impl InventorySystem {
    pub fn get_fresh_ids(&self) -> Vec<usize> {
        self.available
            .clone()
            .into_iter()
            .filter(|x| self.is_fresh(*x))
            .collect()
    }

    fn is_fresh(&self, id: usize) -> bool {
        self.fresh
            .iter()
            .any(|x| x.contains(&id))
    }
}

impl From<&Input> for InventorySystem {
    fn from(input: &Input) -> Self {
        let (fresh_input, avail_input) = input.split_once("\n\n").unwrap();

        let fresh: Vec<RangeInclusive<usize>> = fresh_input
            .trim()
            .lines()
            .map(|s| str_to_range(s))
            .collect();
            
        let available: Vec<usize> = avail_input
            .trim()
            .lines()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        InventorySystem {
            fresh,
            available
        }

    }
}

fn str_to_range(s: &str) -> RangeInclusive<usize> {
    let (start_str, end_str) = s
        .split_once("-")
        .unwrap();

    let start = start_str.parse::<usize>().unwrap();
    let end = end_str.parse::<usize>().unwrap();

    RangeInclusive::new(start, end)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::day05::input::read;

    #[test]
    fn test_solve() {
        let input_str = "\
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let input = read(&input_str);

        let out = solve(&input);
        
        assert_eq!(
            out,
            Some(3)
        );

    }
}
