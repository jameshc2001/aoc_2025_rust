
fn fresh_ingredients(input: &str) -> i64 {
    let (ranges, ids) = parse_input(input);

    ids.iter().filter(|&id| {
        ranges.iter().any(|(left, right)| left <= id && id <= right)
    }).count() as i64
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