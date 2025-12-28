fn find_password(input: &str) -> i32 {
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

fn find_password_complex(input: &str) -> i32 {
    input
        .lines()
        .map(str::trim)
        .fold((50, 0), |(position, zeros), line| {
            let amount = line[1..].parse::<i32>().unwrap_or_else(|e| {
                panic!("Failed to parse amount in line '{}': {}", line, e);
            });

            let new_position = if line.starts_with('R') {
                position + amount
            } else {
                position - amount
            };

            let boundary_crossings = if new_position < 0 {
                (new_position / 100).abs()
            } else {
                new_position / 100
            };

            let zero_crossing = if (new_position < 0 && position > 0) || new_position == 0 { 1 } else { 0 };

            (wrap_position(new_position, 100), zeros + boundary_crossings + zero_crossing)
        })
        .1
}

fn wrap_position(position: i32, modulo: i32) -> i32 {
    ((position % modulo) + modulo) % modulo
}

#[cfg(test)]
mod tests {
    use crate::day01;
    use std::fs;

    #[test]
    fn sample_password() {
        assert_eq!(day01::find_password(SAMPLE_INPUT), 3);
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day01.txt").unwrap();
        assert_eq!(day01::find_password(input.as_str()), 1029);
    }

    #[test]
    fn sample_complex_password() {
        assert_eq!(day01::find_password_complex(SAMPLE_INPUT), 6);
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day01.txt").unwrap();
        assert_eq!(day01::find_password_complex(input.as_str()), 5892);
    }

    #[test]
    fn how_does_division_work() {
        assert_eq!(100 / 100, 1);
        assert_eq!(50 / 100, 0);
        assert_eq!(101 / 100, 1);
        assert_eq!(199 / 100, 1);
        assert_eq!(-100 / 100, -1);
        assert_eq!(-199 / 100, -1);
        assert_eq!(-50 / 100, 0);
    }

    const SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
}
