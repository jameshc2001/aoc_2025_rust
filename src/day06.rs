const MULTIPLY: char = '*';
const ADD: char = '+';

fn grand_total(input: &str) -> i64 {
    let (operations, numbers) = parse_input(input);

    (0..numbers[0].len())
        .map(|i| {
            let op = operations[i];
            let (init, combine): (i64, fn(i64, i64) -> i64) = if op == MULTIPLY {
                (1, |a, b| a * b)
            } else {
                (0, |a, b| a + b)
            };
            numbers.iter().map(|row| row[i]).fold(init, combine)
        })
        .sum()
}

fn parse_input(input: &str) -> (Vec<char>, Vec<Vec<i64>>) {
    let lines: Vec<_> = input.lines().collect();

    let (last_line, number_lines) = lines.split_last()
        .expect("Input must have at least one line");

    let numbers: Vec<Vec<i64>> = number_lines.iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().expect("Invalid number"))
                .collect()
        })
        .collect();

    let operations: Vec<char> = last_line
        .split_whitespace()
        .filter_map(|token| token.chars().next())
        .collect();

    (operations, numbers)
}

fn correct_grand_total(input: &str) -> i64 {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let max_length = lines.iter()
        .map(|line| line.len())
        .max()
        .unwrap_or(0);

    let transposed: Vec<String> = (0..max_length)
        .rev()
        .map(|col| {
            lines.iter()
                .filter_map(|row| row.get(col))
                .collect::<String>()
                .trim()
                .to_string()
        })
        .collect();

    let mut total = 0i64;
    let mut buffer = Vec::new();

    for line in transposed {
        if let Ok(num) = line.parse::<i64>() {
            buffer.push(num);
        } else if let Some(op_pos) = line.rfind(|c| c == MULTIPLY || c == ADD) {
            // Extract the number before the operator
            if let Ok(num) = line[..op_pos].trim().parse::<i64>() {
                buffer.push(num);

                // Apply the operation
                let op = line.chars().nth(op_pos).unwrap();
                let result = match op {
                    MULTIPLY => buffer.iter().product::<i64>(),
                    ADD => buffer.iter().sum::<i64>(),
                    _ => unreachable!(),
                };
                total += result;
                buffer.clear();
            }
        }
    }

    total
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