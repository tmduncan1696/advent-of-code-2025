use std::collections::HashMap;

use crate::day07::input::Input;
use crate::day07::part1::TachyonManifold;

pub fn solve(input: &Input) -> Option<usize> {
    let tachyon_manifold = TachyonManifold::from(input);
    let out = tachyon_manifold.count_timelines();
    Some(out)
}

impl TachyonManifold {
    fn count_timelines(&self) -> usize {
        let mut beam_locs = HashMap::new();

        for line in self.manifold.iter() {
            let start_idx = line.get_start_position();
            if let Some(idx) = start_idx {
                beam_locs.insert(idx, 1);
                continue;
            }

            let mut new_beam_locs = HashMap::new();
            for (loc, count) in beam_locs.iter() {
                let val: char = line.get(*loc);
                match val {
                    '.' => {
                        new_beam_locs.insert(*loc, *count + new_beam_locs.get(loc).unwrap_or(&0));
                    },
                    '^' => {
                        new_beam_locs.insert(*loc - 1, *count + new_beam_locs.get(&(loc - 1)).unwrap_or(&0));
                        new_beam_locs.insert(*loc + 1, *count + new_beam_locs.get(&(loc + 1)).unwrap_or(&0));
                    },
                    _ => panic!("Unexpected char in line")
                }
            }

            beam_locs = new_beam_locs;
        }

        beam_locs
            .iter()
            .map(|(_, v)| v)
            .sum()
    }
}


#[cfg(test)]
mod test {
    use super::*;

   use crate::day07::input::read;

    #[test]
    fn test_solve() {
        let input_str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        let input = read(&input_str);

        let out = solve(&input);

        assert_eq!(
            out,
            Some(40)
        );
    }
}
