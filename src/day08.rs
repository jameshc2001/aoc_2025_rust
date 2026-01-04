use itertools::Itertools;
use std::collections::HashMap;

fn connect_junction_boxes(input: &str, limit: usize) -> i64 {

    let junction_boxes = input
        .lines()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|p| p.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            (parts[0], parts[1], parts[2])
        })
        .collect::<Vec<(i64, i64, i64)>>();

    let unique_pairs_by_distance: Vec<_> = junction_boxes
        .iter()
        .combinations(2)
        .sorted_by_key(|pair| {
            let a = pair[0];
            let b = pair[1];
            let x = a.0 - b.0;
            let y = a.1 - b.1;
            let z = a.2 - b.2;
            x * x + y * y + z * z
        })
        .map(|pair| (pair[0], pair[1]))
        .take(limit) //assumes that if we try to connect boxes already in a circuit, it counts as a 'connection'
        .collect();

    let mut box_to_circuit: HashMap<&(i64, i64, i64), usize> = HashMap::new();
    let mut circuit_to_size: HashMap<usize, i64> = HashMap::new();
    let mut next_circuit_id: usize = 0;

    for (a, b) in unique_pairs_by_distance {
        let a_circuit = box_to_circuit.get(&a).copied();
        let b_circuit = box_to_circuit.get(&b).copied();

        match (a_circuit, b_circuit) {
            (Some(a_c), Some(b_c)) if a_c == b_c => {
                //already in the same circuit, do nothing
                continue;
            }
            (Some(a_c), Some(b_c)) => {
                //both in different circuits, merge them
                let b_c_size = circuit_to_size.remove(&b_c).unwrap();
                circuit_to_size.insert(a_c, circuit_to_size.get(&a_c).unwrap() + b_c_size);
                for (_, circuit) in box_to_circuit.iter_mut() {
                    if *circuit == b_c {
                        *circuit = a_c;
                    }
                }
            }
            (Some(a_c), None) => {
                //a already in circuit, add b to it
                box_to_circuit.insert(b, a_c);
                circuit_to_size.insert(a_c, circuit_to_size.get(&a_c).unwrap() + 1);
            }
            (None, Some(b_c)) => {
                //b already in circuit, add a to it
                box_to_circuit.insert(a, b_c);
                circuit_to_size.insert(b_c, circuit_to_size.get(&b_c).unwrap() + 1);
            }
            (None, None) => {
                //neither a or b in a circuit, create one
                box_to_circuit.insert(a, next_circuit_id);
                box_to_circuit.insert(b, next_circuit_id);
                circuit_to_size.insert(next_circuit_id, 2);
                next_circuit_id += 1;
            }
        }
    }

    circuit_to_size
        .values()
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc: i64, n| acc * *n)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day08;

    #[test]
    fn can_connect_junction_boxes_for_sample_input() {
        assert_eq!(day08::connect_junction_boxes(SAMPLE_INPUT, 10), 40)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day08.txt").unwrap();
        assert_eq!(day08::connect_junction_boxes(input.as_str(), 1000), 47040)
    }

    const SAMPLE_INPUT: &str = "162,817,812
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
}