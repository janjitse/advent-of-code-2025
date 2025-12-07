use crate::parsers::parse_array_of_chars;

#[aoc(day7, part1)]
pub fn part_a(contents: &str) -> i32 {
    let vec = parse_array_of_chars(contents);
    let mut nr_splits = 0;
    let directions = [1, usize::MAX];
    let mut todo: Vec<bool> = vec![false; vec[0].len()];

    for line in vec.iter() {
        let mut new_todo = vec![false; vec[0].len()];
        for (col_idx, pos) in line.iter().zip(todo.iter()).enumerate() {
            match pos {
                ('S', _) => {
                    new_todo[col_idx] = true;
                }
                ('^', true) => {
                    nr_splits += 1;
                    for dir in directions.iter() {
                        let new_beam = col_idx.wrapping_add(*dir);
                        if new_beam < line.len() {
                            new_todo[new_beam] = true;
                        }
                    }
                }
                (_, true) => {
                    new_todo[col_idx] = true;
                }
                _ => {}
            }
        }
        todo = new_todo;
    }

    nr_splits
}

#[aoc(day7, part2)]
pub fn part_b(contents: &str) -> u64 {
    let vec = parse_array_of_chars(contents);
    let mut nr_splits = 0;
    let directions = [1, usize::MAX];
    let mut todo: Vec<u64> = vec![0; vec[0].len()];

    for line in vec.iter() {
        let mut new_todo = vec![0; vec[0].len()];
        for (col_idx, (pos, nr_tachyons)) in line.iter().zip(todo.iter()).enumerate() {
            match pos {
                'S' => {
                    new_todo[col_idx] += 1 + nr_tachyons;
                    nr_splits += 1;
                }
                '^' => {
                    nr_splits += nr_tachyons;
                    for dir in directions.iter() {
                        let new_beam = col_idx.wrapping_add(*dir);
                        if new_beam < line.len() {
                            new_todo[new_beam] += nr_tachyons;
                        }
                    }
                }
                _ => {
                    new_todo[col_idx] += nr_tachyons;
                }
            }
        }
        todo = new_todo;
    }

    nr_splits
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{file, fs, path::Path};

    #[test]
    fn test_1() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_a(&contents), 21);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 40);
    }
}
