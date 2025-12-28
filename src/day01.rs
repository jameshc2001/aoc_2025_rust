fn find_password(input: &str) -> i32 {
    fn wrap_position(position: i32, modulo: i32) -> i32 {
        ((position % modulo) + modulo) % modulo
    }

    input
        .lines()
        .map(str::trim)
        .fold((50, 0), |(mut position, zeros), line| {
            let amount = line[1..].parse::<i32>().unwrap_or_else(|e| {
                panic!("Failed to parse amount in line '{}': {}", line, e);
            });

            position = if line.starts_with('R') {
                position + amount
            } else {
                position - amount
            };

            position = wrap_position(position, 100);

            let zeros = if position == 0 { zeros + 1 } else { zeros };

            (position, zeros)
        })
        .1
}

#[cfg(test)]
mod tests {
    use crate::day01;
    use std::fs;

    #[test]
    fn simple_password() {
        let sample_input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(day01::find_password(sample_input), 3);
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day01.txt").unwrap();
        assert_eq!(day01::find_password(input.as_str()), 1029);
    }
}
