use std::cmp::max;

fn accessible_rolls(input: &str) -> i32 {
    let grid = parse_input_to_grid(input);

    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(x, cell)| **cell == '@' && is_accessible(*x as i32, y as i32, &grid))
                .count()
        })
        .sum::<usize>() as i32
}

fn parse_input_to_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|line| grid.push(line.chars().collect()));
    grid
}

fn is_accessible(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
    let mut adj_count = 0;
    for x_adj in max(0, x-1)..=(x+1) {
        for y_adj in max(0, y-1)..=(y+1) {
            if x_adj == x && y_adj == y { continue; }

            if &'@' == grid
                .get(y_adj as usize)
                .and_then(|row| row.get(x_adj as usize))
                .unwrap_or(&'.')
            {
                adj_count += 1;
            }
        }
    }
    adj_count < 4
}


#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day04;

    #[test]
    fn can_find_accessible_rolls_for_sample() {
        assert_eq!(day04::accessible_rolls(SAMPLE_INPUT), 13)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day04.txt").unwrap();
        assert_eq!(day04::accessible_rolls(input.as_str()), 1457);
    }

    const SAMPLE_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

}