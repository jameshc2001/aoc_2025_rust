
fn find_total_joltage(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| { acc + find_joltage(line) })
}

fn find_joltage(input: &str) -> i32 {
    let left = input[0..input.len()-1].chars().max().unwrap();

    let index_of_left = input.chars().enumerate().find(|(_, element)| { *element == left }).unwrap().0;

    let right = input[index_of_left+1..input.len()].chars().max().unwrap();

    let mut result = String::with_capacity(2);
    result.push(left);
    result.push(right);
    result.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day03;

    #[test]
    fn can_find_total_joltage_for_sample() {
        assert_eq!(day03::find_total_joltage(SAMPLE_INPUT), 357)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day03.txt").unwrap();
        assert_eq!(day03::find_total_joltage(input.as_str()), 17311);
    }

    const SAMPLE_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

}