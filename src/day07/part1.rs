use crate::day07::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let tachyon_manifold = TachyonManifold::from(input);
    let out = tachyon_manifold.count_splits();
    Some(out)
}

pub struct TachyonLine(Vec<char>);

impl From<&String> for TachyonLine {
    fn from(s: &String) -> Self {
        let chars: Vec<char> = s.chars().collect();

        TachyonLine(chars)
    }
}

impl TachyonLine {
    pub fn get_start_position(&self) -> Option<usize> {
        self.0
            .iter()
            .position(|c| *c == 'S')
    }
    
    pub fn get(&self, idx: usize) -> char {
        self.0[idx]
    }
}

pub struct TachyonManifold {
    pub manifold: Vec<TachyonLine>
}

impl From<&Input> for TachyonManifold {
    fn from(input: &Input) -> Self {
        let manifold: Vec<TachyonLine> = input
            .into_iter()
            .map(|s| TachyonLine::from(s))
            .collect();

        TachyonManifold {
            manifold
        }
    }
}

impl TachyonManifold {
    fn count_splits(&self) -> usize {
        let mut split_count: usize = 0;

        let mut beam_locs = Vec::new();

        for line in self.manifold.iter() {
            let start_idx = line.get_start_position();
            if let Some(idx) = start_idx {
                beam_locs.push(idx);
                continue;
            }

            let mut new_beam_locs = Vec::new();
            for loc in beam_locs.iter() {
                let val: char = line.get(*loc);
                match val {
                    '.' => new_beam_locs.push(*loc),
                    '^' => {
                        new_beam_locs.push(*loc - 1);
                        new_beam_locs.push(*loc + 1);
                        split_count += 1;
                    },
                    _ => panic!("Unexpected char in line")
                }
            }

            new_beam_locs.sort();
            new_beam_locs.dedup();

            beam_locs = new_beam_locs;
        }

        split_count
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
            Some(21)
        );
    }
}
