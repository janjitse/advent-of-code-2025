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
        numbers.fold(self.initial(), |inter, x| Op::op(self, inter, x))
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
        for ((op, p), update) in ops.iter().zip(line.into_iter()).zip(intermed.iter_mut()) {
            let num_p = p.parse::<u64>().unwrap();
            *update = op.op(*update, num_p);
        }
    }
    intermed.iter().sum()
}

#[aoc(day6, part2)]
pub fn part_b(contents: &str) -> u64 {
    let vec = parse_array_of_chars(contents);
    let ops = vec
        .last()
        .unwrap()
        .iter()
        .filter(|x| **x == '+' || **x == '*')
        .map(|x| Op::from_char(*x))
        .rev()
        .collect::<Vec<Op>>();
    let mut new_vec = vec![];

    for width_idx in (0..vec[0].len()).rev() {
        let mut new_line = vec![];

        for row_idx in 0..vec.len() - 1 {
            new_line.push(vec[row_idx][width_idx]);
        }
        let line_value = new_line.into_iter().collect::<String>();
        new_vec.push(String::from(line_value.trim()));
    }
    let mut ops_idx = 0;
    let mut total = 0;
    let mut intermed_value = ops[ops_idx].initial();
    for s in new_vec {
        match s.is_empty() {
            true => {
                ops_idx += 1;
                total += intermed_value;
                intermed_value = ops[ops_idx].initial();
            }
            false => {
                intermed_value = ops[ops_idx].op(intermed_value, s.parse::<u64>().unwrap());
            }
        }
    }
    total += intermed_value;
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
