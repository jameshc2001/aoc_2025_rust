use std::collections::HashSet;

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

fn removable_rolls(input: &str) -> i32 {
    let mut grid = parse_input_to_grid(input);

    let mut coords_to_check: HashSet<(i32, i32)> = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                coords_to_check.insert((x as i32, y as i32));
            }
        }
    }

    let mut result = 0;
    while coords_to_check.len() > 0 {
        let mut next_coords_to_check: HashSet<(i32, i32)> = HashSet::new();
        for (x, y) in coords_to_check {
            if &'@' == safe_get(x, y, &grid) && is_accessible(x, y, &grid) {
                grid[y as usize][x as usize] = 'x';
                result += 1;

                for x_adj in x - 1..=(x + 1) {
                    for y_adj in y - 1..=(y + 1) {
                        if x_adj == x && y_adj == y { continue; }

                        if &'@' == safe_get(x_adj, y_adj, &grid) {
                            next_coords_to_check.insert((x_adj, y_adj));
                        }
                    }
                }
            }
        }
        coords_to_check = next_coords_to_check;
    }

    result
}

fn parse_input_to_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();
    input.lines().for_each(|line| grid.push(line.chars().collect()));
    grid
}

fn is_accessible(x: i32, y: i32, grid: &Vec<Vec<char>>) -> bool {
    let mut adj_count = 0;
    for x_adj in x-1..=(x+1) {
        for y_adj in y-1..=(y+1) {
            if x_adj == x && y_adj == y { continue; }

            if &'@' == safe_get(x_adj, y_adj, grid) {
                adj_count += 1;
            }
        }
    }
    adj_count < 4
}

fn safe_get(x: i32, y: i32, grid: &Vec<Vec<char>>) -> &char {
    if x < 0 || y < 0 { return &'.'; }
    grid.get(y as usize).and_then(|row| row.get(x as usize)).unwrap_or(&'.')
}

#[cfg(test)]
mod tests {
    use crate::day04;
    use std::fs;

    #[test]
    fn can_find_accessible_rolls_for_sample() {
        assert_eq!(day04::accessible_rolls(SAMPLE_INPUT), 13)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day04.txt").unwrap();
        assert_eq!(day04::accessible_rolls(input.as_str()), 1457);
    }

    #[test]
    fn can_find_removable_rolls_for_sample() {
        assert_eq!(day04::removable_rolls(SAMPLE_INPUT), 43)
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day04.txt").unwrap();
        assert_eq!(day04::removable_rolls(input.as_str()), 8310);
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