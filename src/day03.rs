
fn find_total_joltage_simple(input: &str) -> i64 {
    find_total_joltage(input, 2)
}

fn find_total_joltage_complex(input: &str) -> i64 {
    find_total_joltage(input, 12)
}

fn find_total_joltage(input: &str, batteries: usize) -> i64 {
    input.lines().fold(0, |acc, line| { acc + find_joltage(line, batteries).parse::<i64>().unwrap() })
}

fn find_joltage(input: &str, batteries: usize) -> String {
    if batteries == 1 { return input.chars().max().unwrap().to_string(); }

    let left = input[0..=(input.len() - batteries)].chars().max().unwrap();

    let index_of_left = input.chars().enumerate().find(|(_, element)| { *element == left }).unwrap().0;

    let right = find_joltage(&input[index_of_left + 1..input.len()], batteries - 1);

    left.to_string() + &right
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day03;

    #[test]
    fn can_find_total_joltage_simple_for_sample() {
        assert_eq!(day03::find_total_joltage_simple(SAMPLE_INPUT), 357)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day03.txt").unwrap();
        assert_eq!(day03::find_total_joltage_simple(input.as_str()), 17311);
    }

    #[test]
    fn can_find_total_joltage_complex_for_sample() {
        assert_eq!(day03::find_total_joltage_complex(SAMPLE_INPUT), 3121910778619)
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day03.txt").unwrap();
        assert_eq!(day03::find_total_joltage_complex(input.as_str()), 171419245422055);
    }

    const SAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

}