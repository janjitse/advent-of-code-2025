use regex::Regex;
use std::time::SystemTime;

#[allow(dead_code)]
pub fn parse_twin_vectors(contents: &str) -> (Vec<i32>, Vec<i32>) {
    let vec: Vec<i32> = contents
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let vec1 = vec.iter().step_by(2).cloned().collect();
    let vec2 = vec.iter().skip(1).step_by(2).cloned().collect();
    (vec1, vec2)
}

#[allow(dead_code)]
pub fn parse_whitespace_separated_ints(contents: &str) -> Vec<Vec<i32>> {
    contents
        .lines()
        .map(|x| {
            x.split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
pub fn parse_array_of_chars(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[allow(dead_code)]
pub fn parse_bar_separated_followed_by_commaseparated(
    input: &str,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split('|')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    let output2 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    (output1, output2)
}

#[allow(dead_code)]
pub fn parse_array_of_chars2(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines();
    lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

#[allow(dead_code)]
pub fn parse_number_colon_followed_by_commaseparated(input: &str) -> Vec<(u64, Vec<u64>)> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let l: Vec<&str> = line.split(':').collect();
            let left: u64 = l[0].parse().unwrap();
            let right: Vec<u64> = l[1]
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (left, right)
        })
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[allow(dead_code)]
pub fn parse_line_of_digits(input: &str) -> Vec<usize> {
    let time_start = SystemTime::now();
    let output1 = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[allow(dead_code)]
pub fn parse_array_of_digits(input: &str) -> Vec<Vec<u32>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[allow(dead_code)]
pub fn parse_list_of_whitespace_sep_digits(input: &str) -> Vec<u64> {
    let time_start = SystemTime::now();
    let output1 = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[allow(dead_code)]
pub fn parse_regex_tuples(input: &str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let time_start = SystemTime::now();
    let lines = input.lines();
    let mut positions = vec![];
    let mut velocities = vec![];
    // p=12,45 v=86,29
    let regex = Regex::new(r"p=(?<px>-?\d+),(?<py>-?\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    for l in lines {
        let cap = regex.captures(l).unwrap();
        let p = (
            cap["px"].parse::<i32>().unwrap(),
            cap["py"].parse::<i32>().unwrap(),
        );
        let v = (
            cap["vx"].parse::<i32>().unwrap(),
            cap["vy"].parse::<i32>().unwrap(),
        );
        positions.push(p);
        velocities.push(v);
    }
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    (positions, velocities)
}

#[allow(dead_code)]
pub fn parse_array_of_chars_followed_by_vec_of_chars(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let time_start = SystemTime::now();

    let mut lines = input.lines();
    let map = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let mut movement = vec![];

    for extra in lines {
        let mut p = extra.chars().collect::<Vec<char>>();
        movement.append(&mut p);
    }

    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    (map, movement)
}

#[allow(dead_code)]
pub fn parse_strings_separated_by_char(input: &str, sep: char) -> Vec<Vec<String>> {
    let time_start = SystemTime::now();
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split(sep).map(|x| x.to_string()).collect())
        .collect();
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output1
}

#[allow(dead_code)]
pub fn parse_chunks_of_arrays_of_chars(input: &str, chunk_size: usize) -> Vec<Vec<Vec<char>>> {
    let time_start = SystemTime::now();
    let lines = input.lines().collect::<Vec<_>>();
    let mut output = vec![];
    for l in lines.chunks(chunk_size) {
        let output1 = l
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect();
        output.push(output1);
    }
    println!("Parsing: {:?}", time_start.elapsed().unwrap());
    output
}

#[allow(dead_code)]
pub fn parse_ranges_followed_by_lists(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut lines = input.lines();
    let output1 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let s = line.split('-').collect::<Vec<&str>>();
            (s[0].parse().unwrap(), s[1].parse().unwrap())
        })
        .collect();
    let output2 = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|x| x.parse().unwrap())
        .collect();
    (output1, output2)
}
