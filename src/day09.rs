use itertools::Itertools;

fn calculate_area(pair: &[(i64, i64)]) -> i64 {
    let a = pair[0];
    let b = pair[1];
    (a.0 - b.0 + 1).abs() * (a.1 - b.1 + 1).abs()
}

fn largest_area(input: &str) -> i64 {
    let tiles = input.lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<_>>();

    tiles.iter()
        .copied()
        .combinations(2)
        .map(|pair| calculate_area(&pair))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day09;

    #[test]
    fn can_connect_junction_boxes_for_sample_input() {
        assert_eq!(day09::largest_area(SAMPLE_INPUT), 50)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day09.txt").unwrap();
        assert_eq!(day09::largest_area(input.as_str()), 4748769124)
    }


    const SAMPLE_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
}