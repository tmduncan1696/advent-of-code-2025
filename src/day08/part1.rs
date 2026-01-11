use crate::day08::input::Input;
use crate::day08::distance::get_distances;
use crate::day08::junction_box::JunctionBox;
use crate::day08::circuit::Circuit;

use indicatif::ProgressBar;

pub fn solve(input: &Input, n: usize, progress: bool) -> Option<usize> {
    let boxes: Vec<JunctionBox> = input
        .into_iter()
        .map(|s| JunctionBox::from(s))
        .collect();

    let circuits = create_circuits(boxes, n, progress);

    let mut circuit_sizes: Vec<usize> = circuits
        .into_iter()
        .map(|c| c.size())
        .collect::<Vec<usize>>();

    circuit_sizes.sort_by(|a, b| b.cmp(a));

    let out: usize = circuit_sizes
        .into_iter()
        .take(3)
        .product();

    Some(out)
}

fn create_circuits(boxes: Vec<JunctionBox>, n: usize, progress: bool) -> Vec<Circuit> {
    let mut distances = get_distances(&boxes);

    let mut circuits: Vec<Circuit> = boxes
        .into_iter()
        .map(|b| Circuit::from(vec![b]))
        .collect();
    
    if progress {
        let bar = ProgressBar::new(n as u64);
        for _ in 0..n {
            bar.inc(1);

            if circuits.len() == 1 {
                break;
            }

            let box_pair: (JunctionBox, JunctionBox) = distances
                .iter()
                .min_by_key(|entry| entry.1)
                .map(|(k, _)| *k)
                .unwrap();

            distances.remove(&box_pair);

            let circuits_for_boxes: Vec<Circuit> = circuits
                .iter()
                .filter(|c| c.contains(box_pair.0) || c.contains(box_pair.1))
                .map(|x| x.clone())
                .collect();

            if circuits_for_boxes.len() > 1 {
                if circuits_for_boxes[0] == circuits_for_boxes[1] {
                    continue
                }
            }

            for c in circuits_for_boxes.iter() {
                let idx = circuits.iter().position(|x| x == c);
                if let Some(i) = idx {
                    circuits.remove(i);
                }
            }

            let new_circuit: Circuit = circuits_for_boxes
                .into_iter()
                .sum();

            circuits.push(new_circuit);

        }

        bar.finish();
    } else {
        for _ in 0..n {

            if circuits.len() == 1 {
                break;
            }

            let box_pair: (JunctionBox, JunctionBox) = distances
                .iter()
                .min_by_key(|entry| entry.1)
                .map(|(k, _)| *k)
                .unwrap();

            distances.remove(&box_pair);

            let circuits_for_boxes: Vec<Circuit> = circuits
                .iter()
                .filter(|c| c.contains(box_pair.0) || c.contains(box_pair.1))
                .map(|x| x.clone())
                .collect();

            if circuits_for_boxes.len() > 1 {
                if circuits_for_boxes[0] == circuits_for_boxes[1] {
                    continue
                }
            }

            for c in circuits_for_boxes.iter() {
                let idx = circuits.iter().position(|x| x == c);
                if let Some(i) = idx {
                    circuits.remove(i);
                }
            }

            let new_circuit: Circuit = circuits_for_boxes
                .into_iter()
                .sum();

            circuits.push(new_circuit);

        }
    }

    circuits
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::day08::input::read;

    #[test]
    fn test_solve() {
        let input_str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let input = read(&input_str);

        let out = solve(&input, 10, false);

        assert_eq!(
            out,
            Some(40)
        );

    }
}
