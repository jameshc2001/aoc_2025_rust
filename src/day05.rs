use std::cmp::max;

fn fresh_ingredients(input: &str) -> i64 {
    let (ranges, ids) = parse_input(input);

    ids.iter().filter(|&id| {
        ranges.iter().any(|(left, right)| left <= id && id <= right)
    }).count() as i64
}

fn all_fresh_ingredients(input: &str) -> i64 {
    let (mut ranges, _) = parse_input(input);
    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    merged_ranges.push(*ranges.first().unwrap());

    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if last.0 <= range.0 && range.0 <= last.1 {
                last.1 = max(last.1, range.1);
            } else {
                merged_ranges.push(range);
            }
        }
    }

    merged_ranges.iter().map(|(left, right) | (right - left) + 1).sum()
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let cleaned_input = input.replace("\r", "");
    let parts = cleaned_input.split("\n\n").collect::<Vec<&str>>();

    let ranges = parts[0]
        .lines()
        .map(|line| {
            let range = line.split("-").collect::<Vec<&str>>();
            (range[0].parse::<i64>().unwrap(), range[1].parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();

    let ids = parts[1]
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day05;

    #[test]
    fn can_find_fresh_ingredients_for_sample() {
        assert_eq!(day05::fresh_ingredients(SAMPLE_INPUT), 3)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day05.txt").unwrap();
        assert_eq!(day05::fresh_ingredients(input.as_str()), 758)
    }

    #[test]
    fn can_find_all_fresh_ingredients_for_sample() {
        assert_eq!(day05::all_fresh_ingredients(SAMPLE_INPUT), 14)
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day05.txt").unwrap();
        assert_eq!(day05::all_fresh_ingredients(input.as_str()), 343143696885053)
    }

    const SAMPLE_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
}