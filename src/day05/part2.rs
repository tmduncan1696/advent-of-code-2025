use std::collections::HashSet;

use rangemap::RangeInclusiveSet;

use crate::day05::input::Input;
use crate::day05::part1::InventorySystem;

pub fn solve(input: &Input) -> Option<usize> {
    let inventory = InventorySystem::from(input);

    let out = inventory.count_possible_fresh_ids();

    Some(out)
}

impl InventorySystem {
    pub fn count_fresh_ids(&self) -> usize {
        self.fresh
            .clone()
            .into_iter()
            .map(|x| x.collect::<Vec<_>>())
            .flatten()
            .collect::<HashSet<_>>()
            .len()
    }

    pub fn count_possible_fresh_ids(&self) -> usize {
        let mut ranges = RangeInclusiveSet::new();
        
        for range in self.fresh.clone() {
            ranges.insert(range);
        };

        let range_tuples: Vec<(usize, usize)> = ranges
            .into_iter()
            .map(|x| x.into_inner())
            .collect();

        range_tuples
            .into_iter()
            .map(|x| x.1 - x.0 + 1)
            .sum()
    }
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
            Some(14)
        );
    }
}
