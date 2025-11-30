#[aoc(day1, part1)]
pub fn part_a(contents: &str) -> i32 {
    let vec = parse(contents);
    vec.into_iter().sum()
}

#[aoc(day1, part2)]
pub fn part_b(contents: &str) -> i32 {
    let vec = parse(contents);
    vec.into_iter().sum()
}

pub fn parse(contents: &str) -> Vec<i32> {
    let vec: Vec<i32> = contents
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{file, fs, path::Path};

    #[test]
    fn test_1() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2018/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_a(&contents), 240);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2018/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 4455);
    }
}