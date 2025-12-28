
fn find_invalid_ids(input: &str) -> i64 {
    input
        .replace('\n', "")
        .split(',')
        .fold(0, |acc, line| {
            let parts: Vec<i64> = line.split("-")
                .map(|s| s.parse::<i64>().unwrap())
                .collect();

            let (a, b) = (parts[0], parts[1]);

            (a..=b).filter(|id| { is_invalid_id(*id) }).sum::<i64>() + acc
        })
}

fn is_invalid_id(id: i64) -> bool {
    let as_string = id.to_string();
    if as_string.len() % 2 != 0 { return false; }
    let chars: Vec<char> = as_string.chars().collect();
    let first_half: String = chars[0..chars.len() / 2].iter().collect();
    let second_half: String = chars[chars.len() / 2..].iter().collect();
    first_half == second_half
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day02;

    #[test]
    fn can_find_invalid_ids_for_sample() {
        assert_eq!(day02::find_invalid_ids(SAMPLE_INPUT), 1227775554)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day02.txt").unwrap();
        assert_eq!(day02::find_invalid_ids(input.as_str()), 23560874270);
    }

    const SAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

}