use crate::day08::input::Input;
use crate::day08::circuit::Circuit;
use crate::day08::junction_box::JunctionBox;
use crate::day08::distance::{Distance, get_distances};

pub fn solve(input: &Input) -> Option<usize> {
    let boxes: Vec<JunctionBox> = input
        .into_iter()
        .map(|s| JunctionBox::from(s))
        .collect();

    let box_distances = get_distances(&boxes);

    let mut circuits: Vec<Circuit> = boxes
        .into_iter()
        .map(|b| Circuit::from(vec![b]))
        .collect();

    // This is kinda slow, but it works...
    while circuits.len() > 2 {
        let distances = get_distances(&circuits);


        let circuit_pair: (Circuit, Circuit) = distances
            .iter()
            .min_by_key(|entry| entry.1)
            .map(|(k, _)| k.clone())
            .unwrap();

        circuits.retain(|x| *x != circuit_pair.0 && *x != circuit_pair.1);

        let new_circuit = circuit_pair.0 + circuit_pair.1;

        circuits.push(new_circuit);
    }

    let final_distance = circuits[0].sq_distance(&circuits[1]);

    let junctions = box_distances
        .into_iter()
        .find_map(|(k, v)| if v == final_distance { Some(k) } else { None })
        .unwrap();

    let out = junctions.0.0 * junctions.1.0;

    Some(out)
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

        let out = solve(&input);

        assert_eq!(
            out,
            Some(25272)
        );
    }
}
