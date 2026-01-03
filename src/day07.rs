use std::collections::{HashSet, VecDeque};

fn beam_splits(input: &str) -> i32 {
    let grid_height: i32 = input.lines().count() as i32;

    let mut beams: VecDeque<(i32, i32)> = VecDeque::new();
    let mut splitters: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                beams.push_back((x as i32, y as i32));
            } else if ch == '^' {
                splitters.insert((x as i32, y as i32));
            }
        }
    }

    let mut split_count = 0;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while let Some(beam) = beams.pop_front() {
        let next_pos = (beam.0, beam.1 + 1);
        if next_pos.1 >= grid_height {
            continue;
        } else if splitters.contains(&next_pos) {
            split_count += 1;
            insert_beam(&mut beams, &mut visited, (next_pos.0 - 1, next_pos.1));
            insert_beam(&mut beams, &mut visited, (next_pos.0 + 1, next_pos.1));
        } else {
            insert_beam(&mut beams, &mut visited, next_pos);
        }
    }

    split_count
}

fn insert_beam(beams: &mut VecDeque<(i32, i32)>, visited: &mut HashSet<(i32, i32)>, beam: (i32, i32)) {
    if visited.insert(beam) {
        beams.push_back(beam);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::day07;

    #[test]
    fn can_find_number_of_beam_splits_for_sample_input() {
        assert_eq!(day07::beam_splits(SAMPLE_INPUT), 21)
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string("inputs/day07.txt").unwrap();
        assert_eq!(day07::beam_splits(input.as_str()), 1642)
    }

    const SAMPLE_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
}