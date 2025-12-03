pub fn parse(contents: &str) -> Vec<Vec<u32>> {
    let mut vec = vec![];
    for line in contents.lines() {
        let line_vec: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        vec.push(line_vec);
    }
    vec
}

#[aoc(day3, part1)]
pub fn part_a(contents: &str) -> u32 {
    let vec = parse(contents);
    let mut total_joltage = 0;
    for bank in vec {
        let mut bank_value = 0;
        let mut next_bank = bank.clone();
        for rem in (0..2_usize).rev() {
            let (val, idx) = bank_select(&next_bank, rem);
            bank_value = 10 * bank_value + val;
            next_bank = next_bank.into_iter().skip(idx + 1).collect();
        }
        total_joltage += bank_value;
    }
    total_joltage
}

pub fn bank_select(bank: &[u32], remaining: usize) -> (u32, usize) {
    let bank_len = bank.len();
    let mut max_value = bank.iter().max().unwrap();
    let mut arg_max: Vec<usize> = bank
        .iter()
        .enumerate()
        .filter(|(_, val)| *val == max_value)
        .map(|x| x.0)
        .collect();
    while arg_max[0] >= bank_len - remaining {
        max_value = bank.iter().filter(|val| *val < max_value).max().unwrap();
        arg_max = bank
            .iter()
            .enumerate()
            .filter(|(_, val)| *val == max_value)
            .map(|x| x.0)
            .collect();
    }
    (*max_value, arg_max[0])
}

#[aoc(day3, part2)]
pub fn part_b(contents: &str) -> u64 {
    let vec = parse(contents);
    let mut total_joltage = 0;
    for bank in vec {
        let mut bank_value: u64 = 0;
        let mut next_bank = bank.clone();
        for rem in (0..12_usize).rev() {
            let (val, idx) = bank_select(&next_bank, rem);
            bank_value = 10 * bank_value + val as u64;
            next_bank = next_bank.into_iter().skip(idx + 1).collect();
        }
        total_joltage += bank_value;
    }
    total_joltage
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
        assert_eq!(part_a(&contents), 357);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 3121910778619);
    }
}
