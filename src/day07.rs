use std::collections::{HashMap, HashSet, VecDeque};

fn beam_splits(input: &str) -> i64 {
    let (grid_height, mut beams, splitters) = parse_input(input);

    let mut split_count = 0;
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

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

fn insert_beam(beams: &mut VecDeque<(i64, i64)>, visited: &mut HashSet<(i64, i64)>, beam: (i64, i64)) {
    if visited.insert(beam) {
        beams.push_back(beam);
    }
}

fn timelines(input: &str) -> i64 {
    let (grid_height, mut beams, splitters) = parse_input(input);
    let mut memory: HashMap<(i64, i64), i64> = HashMap::new();

    fn timelines_for_particle(
        particle: (i64, i64),
        grid_height: i64,
        splitters: &HashSet<(i64, i64)>,
        memory: &mut HashMap<(i64, i64), i64>,
    ) -> i64 {
        if let Some(&timeline) = memory.get(&particle) {
            return timeline;
        }

        let next_pos = (particle.0, particle.1 + 1);
        let result = if splitters.contains(&next_pos) {
            let left = (next_pos.0 - 1, next_pos.1);
            let right = (next_pos.0 + 1, next_pos.1);
            timelines_for_particle(left, grid_height, splitters, memory) + timelines_for_particle(right, grid_height, splitters, memory)
        } else if next_pos.1 > grid_height {
            1
        } else {
            timelines_for_particle(next_pos, grid_height, splitters, memory)
        };

        memory.insert(particle, result);
        result
    }

    let start = beams.pop_front().unwrap();
    timelines_for_particle(start, grid_height, &splitters, &mut memory)
}

fn parse_input(input: &str) -> (i64, VecDeque<(i64, i64)>, HashSet<(i64, i64)>) {
    let grid_height: i64 = input.lines().count() as i64;

    let mut beams: VecDeque<(i64, i64)> = VecDeque::new();
    let mut splitters: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'S' {
                beams.push_back((x as i64, y as i64));
            } else if ch == '^' {
                splitters.insert((x as i64, y as i64));
            }
        }
    }

    (grid_height, beams, splitters)
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

    #[test]
    fn can_find_number_of_timelines_for_sample_input() {
        assert_eq!(day07::timelines(SAMPLE_INPUT), 40)
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string("inputs/day07.txt").unwrap();
        assert_eq!(day07::timelines(input.as_str()), 47274292756692)
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