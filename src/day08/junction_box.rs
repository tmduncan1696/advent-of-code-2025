use std::cmp::Ordering;

use crate::day08::distance::Distance;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JunctionBox(pub usize, pub usize, pub usize);

impl From<&String> for JunctionBox {
    fn from(s: &String) -> Self {
        let coords: Vec<usize> = s.split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        JunctionBox(coords[0], coords[1], coords[2])
    }
}

impl Distance for JunctionBox {
    fn sq_distance(&self, other: &Self) -> usize {
        ((self.0 as i64 - other.0 as i64).pow(2) + (self.1 as i64 - other.1 as i64).pow(2) + (self.2 as i64 - other.2 as i64).pow(2)) as usize
    }
}

impl Ord for JunctionBox {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .cmp(&other.0)
            .then(
                self.1
                    .cmp(&other.1)
            )
            .then(
                self.2
                    .cmp(&other.2)
            )
    }
}

impl PartialOrd for JunctionBox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

