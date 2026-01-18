use itertools::Itertools;

fn calculate_area(pair: &[(i64, i64)]) -> i64 {
    let a = pair[0];
    let b = pair[1];
    (a.0 - b.0 + 1).abs() * (a.1 - b.1 + 1).abs()
}

fn get_red_tiles(input: &str) -> Vec<(i64, i64)> {
    input.lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .collect::<Vec<_>>()
}

fn largest_area(input: &str) -> i64 { // O(n^2) but good enough, could optimize by finding min and max coords
    get_red_tiles(input)
        .iter()
        .copied()
        .combinations(2)
        .map(|pair| calculate_area(&pair))
        .max()
        .unwrap()
}

fn largest_area_constrained(input: &str) -> i64 {


    -1
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day09;

    #[test]
    fn can_find_largest_area() {
        assert_eq!(day09::largest_area(SAMPLE_INPUT), 50)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day09.txt").unwrap();
        assert_eq!(day09::largest_area(input.as_str()), 4748769124)
    }

    #[test]
    fn can_find_largest_area_constrained() {
        assert_eq!(day09::largest_area_constrained(SAMPLE_INPUT), 24)
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