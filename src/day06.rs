use crate::parsers::*;

#[aoc(day6, part1)]
pub fn part_a(contents: &str) -> u64 {
    let vec = parse_array_of_strings(contents);
    let ops = vec
        .last()
        .unwrap()
        .iter()
        .map(|x| x.chars().next().unwrap())
        .collect::<Vec<char>>();
    let mut intermed = vec![0; ops.len()];
    for (idx, p) in intermed.iter_mut().enumerate() {
        *p = match ops[idx] {
            '*' => 1,
            '+' => 0,
            _ => panic!(),
        };
    }
    // println!()
    // println!("{:?}", ops);
    // println!("{:?}", intermed);
    for line in vec.iter().take(vec.len() - 1) {
        for (idx, p) in line.iter().enumerate() {
            let num_p = p.parse::<u64>().unwrap();
            intermed[idx] = match ops[idx] {
                '*' => num_p * intermed[idx],
                '+' => num_p + intermed[idx],
                _ => panic!(),
            };
        }
    }
    // let vec = parse(contents);
    // vec.into_iter().sum()
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
        .copied()
        .rev()
        .collect::<Vec<char>>();
    let mut new_vec = vec![];

    for width_idx in (0..vec[0].len()).rev() {
        let mut new_line = vec![];

        for row_idx in 0..vec.len() - 1 {
            new_line.push(vec[row_idx][width_idx]);
        }
        let line_value = new_line.into_iter().collect::<String>();
        new_vec.push(String::from(line_value.trim()));
    }
    // println!("{:?}", new_vec);
    let mut ops_idx = 0;
    let mut total = 0;
    let mut intermed_value = match ops[ops_idx] {
        '*' => 1,
        '+' => 0,
        _ => panic!(),
    };
    for s in new_vec {
        match s.len() {
            0 => {
                ops_idx += 1;
                total += intermed_value;
                intermed_value = match ops[ops_idx] {
                    '*' => 1,
                    '+' => 0,
                    _ => panic!(),
                };
            }
            _ => {
                match ops[ops_idx] {
                    '*' => intermed_value *= s.parse::<u64>().unwrap(),
                    '+' => intermed_value += s.parse::<u64>().unwrap(),
                    _ => panic!(),
                };
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
