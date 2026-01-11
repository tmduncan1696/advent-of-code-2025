use std::ops::Add;
use std::iter::Sum;

use crate::day08::junction_box::JunctionBox;
use crate::day08::distance::Distance;

#[derive(PartialEq, Clone, Hash, Eq)]
pub struct Circuit {
    boxes: Vec<JunctionBox>
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            boxes: Vec::new()
        }
    }

    pub fn size(&self) -> usize {
        self.boxes.len()
    }

    pub fn contains(&self, junction_box: JunctionBox) -> bool {
        self.boxes.contains(&junction_box)
    }

}

impl Distance for Circuit {
    fn sq_distance(&self, other: &Circuit) -> usize {
        let mut distances = Vec::new();

        for box1 in &self.boxes {
            for box2 in &other.boxes {
                distances.push(box1.sq_distance(box2))
            }
        }

        let out = distances
            .into_iter()
            .min()
            .unwrap();

        out
    }
}

impl From<Vec<JunctionBox>> for Circuit {
    fn from(boxes: Vec<JunctionBox>) -> Self {
        Circuit {
            boxes
        }
    }
}

impl Add for Circuit {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut comb_boxes = Vec::new();
        comb_boxes.push(self.boxes);
        comb_boxes.push(other.boxes);

        let boxes: Vec<JunctionBox> = comb_boxes
            .into_iter()
            .flatten()
            .collect();

        Circuit::from(boxes)
    }
}

impl Sum for Circuit {
    fn sum<I>(iter: I) -> Circuit
    where
        I: Iterator<Item = Self>
    {
        iter
            .fold(
                Circuit::new(),
                |acc, circuit| acc + circuit
            )
    }
}
