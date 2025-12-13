pub fn parse(input: &str) -> (Vec<(usize, Vec<Vec<char>>)>, Vec<(Vec<i64>, Vec<i64>)>) {
    let mut lines = input.lines();
    let mut output = vec![];

    for _ in 0..=5 {
        let map = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let idx = map[0]
            .iter()
            .take(map[0].len() - 1)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let box_shape = map.iter().skip(1).cloned().collect();
        output.push((idx, box_shape))
    }
    let mut boxes = vec![];
    for l in lines {
        let output1 = l.split(":").collect::<Vec<&str>>();
        let box_dimensions = output1[0]
            .split("x")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let box_contents = output1[1]
            .split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        boxes.push((box_dimensions, box_contents))
    }
    (output, boxes)
}

#[aoc(day12, part1)]
pub fn part_a(contents: &str) -> i64 {
    let (_presents, boxes): (Vec<(usize, Vec<Vec<char>>)>, Vec<(Vec<i64>, Vec<i64>)>) =
        parse(contents);
    let mut output = 0;
    for b in boxes {
        if b.0[0]/3 * b.0[1]/3 >=  b.1.iter().sum::<i64>() {
            output += 1;
        }
        if b.0[0] * b.0[1] >= 7 * b.1.iter().sum::<i64>() &&  b.0[0] * b.0[1] < 9 * b.1.iter().sum::<i64>(){
            // Do something complicated...?
             println!("{:?}, {:?}", b.0, b.1);
        }
    }
    output
}

#[aoc(day12, part2)]
pub fn part_b(_contents: &str) -> i64 {
    0
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
        assert_eq!(part_a(&contents), 0);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 0);
    }
}
