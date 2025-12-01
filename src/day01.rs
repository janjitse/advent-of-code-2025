#[aoc(day1, part1)]
pub fn part_a(contents: &str) -> i32 {
    let vec = parse(contents);
    let mut initial = 50;
    let mut nr_times = 0;
    for p in vec {
        initial += p;
        initial %= 100;
        if initial == 0 {
            nr_times += 1;
        }
    }
    nr_times
}

#[aoc(day1, part2)]
pub fn part_b(contents: &str) -> i32 {
    let vec = parse(contents);
    let mut initial = 50;
    let mut nr_times = 0;

    for p in vec {
        initial += p;
        nr_times += (initial / 100).abs();
        if initial == 0 {
            nr_times += 1;
        }
        if initial < 0 && (initial - p) != 0 {
            nr_times += 1;
        }
        initial = initial.rem_euclid(100);
    }
    nr_times
}

pub fn parse(contents: &str) -> Vec<i32> {
    let mut vec = vec![];
    for l in contents.lines() {
        let sign = match l.chars().next() {
            Some('L') => -1,
            Some('R') => 1,
            _ => 0,
        };
        let value: i32 = l[1..].trim().parse().unwrap();
        vec.push(sign * value)
    }

    vec
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
        assert_eq!(part_a(&contents), 3);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 6);
    }
}
