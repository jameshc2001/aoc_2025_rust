
fn grand_total(input: &str) -> i64 {
    let (operations, numbers) = parse_input(input);

    let mut result = 0;
    for i in 0..numbers[0].len() {
        let mut current_result = if operations[i] == '*' { 1 } else { 0 };
        for j in 0..numbers.len() {
            if operations[i] == '*' {
                current_result = current_result * numbers[j][i];
            } else {
                current_result = current_result + numbers[j][i];
            }
        }
        result += current_result;
    }

    result
}

fn parse_input(input: &str) -> (Vec<char>, Vec<Vec<i64>>) {
    let lines = input.lines().collect::<Vec<&str>>();

    let numbers = lines[0..lines.len() - 1].iter()
        .map(|line| line.split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
        )
        .collect::<Vec<Vec<i64>>>();

    let operations = lines[lines.len() - 1].split_whitespace()
        .filter_map(|token| token.chars().next())
        .collect::<Vec<char>>();

    (operations, numbers)
}

fn correct_grand_total(input: &str) -> i64 {
    let lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let max_length = lines.iter().map(|line| line.len()).max().unwrap();

    let transposed = (0..max_length)
        .rev()
        .map(|col| {
            (0..lines.len())
                .map(|row| lines[row].get(col).unwrap_or(&' '))
                .collect::<String>()
                .trim()
                .to_string()
        })
        .collect::<Vec<String>>();

    let mut total_result = 0;
    let mut buffer: Vec<i64> = Vec::new();

    for line in transposed {
        if let Ok(num) = line.parse::<i64>() {
            buffer.push(num);
        } else if line.contains("*") || line.contains("+") {
            buffer.push(line[0..line.len() - 1].trim().parse::<i64>().unwrap());

            if line.contains("*") {
                total_result += buffer.iter().fold(1, |acc, num| acc * num);
            } else {
                total_result += buffer.iter().fold(0, |acc, num| acc + num);
            }

            buffer.clear();
        }
    }

    total_result
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day06;

    #[test]
    fn can_find_grand_total_for_sample() {
        assert_eq!(day06::grand_total(SAMPLE_INPUT), 4277556)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day06.txt").unwrap();
        assert_eq!(day06::grand_total(input.as_str()), 6503327062445)
    }

    #[test]
    fn can_find_correct_grand_total_for_sample() {
        assert_eq!(day06::correct_grand_total(SAMPLE_INPUT), 3263827)
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day06.txt").unwrap();
        assert_eq!(day06::correct_grand_total(input.as_str()), 9640641878593)
    }

    const SAMPLE_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
}