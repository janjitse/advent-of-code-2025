use crate::parsers::*;

enum Op {
    Mul,
    Add,
}

impl Op {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Op::Add,
            '*' => Op::Mul,
            _ => panic!(),
        }
    }
    fn initial(&self) -> u64 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }
    fn op(&self, x: u64, y: u64) -> u64 {
        match self {
            Self::Add => x + y,
            Self::Mul => x * y,
        }
    }

    fn consume<I>(&self, numbers: I) -> u64
    where
        I: Iterator<Item = u64>,
    {
        numbers.fold(self.initial(), |inter, x| self.op(inter, x))
    }
}

#[aoc(day6, part1)]
pub fn part_a(contents: &str) -> u64 {
    let vec = parse_array_of_strings(contents);
    let ops = vec
        .last()
        .unwrap()
        .iter()
        .map(|x| Op::from_char(x.chars().next().unwrap()))
        .collect::<Vec<Op>>();
    let mut intermed: Vec<u64> = ops.iter().map(|x| x.initial()).collect();
    for line in vec.iter().take(vec.len() - 1) {
        for ((op, p), update) in ops.iter().zip(line.iter()).zip(intermed.iter_mut()) {
            let num_p = p.parse::<u64>().unwrap();
            *update = op.op(*update, num_p);
        }
    }
    intermed.iter().sum()
}

#[aoc(day6, part2)]
pub fn part_b(contents: &str) -> u64 {
    let vec = parse_array_of_chars(contents);
    let mut ops = vec
        .last()
        .unwrap()
        .iter()
        .filter(|x| **x == '+' || **x == '*')
        .map(|x| Op::from_char(*x));
    let mut new_vec = vec![];

    for width_idx in 0..vec[0].len() {
        let new_line = vec
            .iter()
            .take(vec.len() - 1)
            .map(|s| s[width_idx])
            .filter(|s| !s.is_whitespace())
            .collect::<String>();
        new_vec.push(new_line);
    }
    let mut total = 0;
    let mut transposed = new_vec.iter().peekable();
    while transposed.peek().is_some() {
        let op = ops.next().unwrap();
        total += op.consume(transposed.by_ref().map_while(|x| x.parse::<u64>().ok()));
    }
    total
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
        assert_eq!(part_a(&contents), 4277556);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 3263827);
    }
}
