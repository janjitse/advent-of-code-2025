use std::collections::{HashMap, HashSet};

use crate::parsers::*;

#[aoc(day11, part1)]
pub fn part_a(contents: &str) -> u64 {
    let vec = parse_string_colon_followed_by_whitespace_separated(contents);
    let mut connections: HashMap<String, Vec<&str>> = HashMap::new();
    for (start, out) in vec {
        connections.insert(start.to_string(), out);
    }
    // println!("{:?}", connections);
    let mut todo = vec![];
    let mut out_paths = 0;
    for i in connections.get("you").unwrap() {
        todo.push(vec![*i]);
    }
    while let Some(next) = todo.pop() {
        let last_item = next.last().unwrap().to_string();
        for i in connections.get(&last_item).unwrap() {
            if *i == "fft" || *i == "dac" {
                println!("Found: {:?}", *i);
            }
            if *i == "out" {
                out_paths += 1;
            } else {
                let new_route = next.iter().copied().chain(vec![*i]).collect::<Vec<&str>>();

                todo.push(new_route);
            }
        }
    }
    out_paths
}

#[aoc(day11, part2)]
pub fn part_b(contents: &str) -> u64 {
    let vec = parse_string_colon_followed_by_whitespace_separated(contents);
    let mut connections: HashMap<String, Vec<&str>> = HashMap::new();
    let mut inputs: HashMap<String, HashSet<&str>> = HashMap::new();

    for (start, out) in vec {
        for o in out.iter() {
            inputs.entry(o.to_string()).or_default().insert(start);
        }
        connections.insert(start.to_string(), out);
    }
    let mut todo_out = vec![];
    let mut after_dac = HashSet::new();
    let mut dac_out_paths: HashSet<Vec<&str>> = HashSet::new();
    for i in connections.get("dac").unwrap() {
        todo_out.push(vec![*i]);
    }
    while let Some(next) = todo_out.pop() {
        let last_item = next.last().unwrap().to_string();
        for i in connections.get(&last_item).unwrap() {
            let new_route = next.iter().copied().chain(vec![*i]).collect::<Vec<&str>>();
            if *i == "fft" {
                println!("Warning! fft found after dac!");
            }
            if *i == "out" {
                for node in new_route.iter() {
                    after_dac.insert(*node);
                }
                dac_out_paths.insert(new_route);
            } else {
                todo_out.push(new_route);
            }
        }
    }
    println!("From dac to out {:?}", dac_out_paths.len());

    let mut colouring_fft: HashMap<String, (u64, bool, HashSet<&str>)> = HashMap::new();
    let inputs_fft = inputs.clone();
    let mut todo_fft = vec![];
    let mut svr_fft_paths: u64 = 0;
    for i in connections.get("svr").unwrap() {
        todo_fft.push((*i, "svr", 1u64));
    }
    while let Some((next, prev, count)) = todo_fft.pop() {
        let item = colouring_fft
            .entry(next.to_string())
            .or_insert((0, false, HashSet::new()));
        item.0 += count;
        item.2.insert(prev);
        if item.2 == *inputs_fft.get(next).unwrap() {
            item.1 = true;
            for i in connections.get(next).unwrap() {
                if *i == "fft" {
                    svr_fft_paths += item.0;
                } else if after_dac.contains(i) {
                    continue;
                } else {
                    let s = inputs.get_mut(*i).unwrap();
                    s.remove(next);
                    todo_fft.push((*i, next, item.0))
                }
            }
        }
    }

    println!("From svr to fft {:?}", svr_fft_paths);

    let mut colouring_dac: HashMap<String, (u64, bool, HashSet<&str>)> = HashMap::new();
    let mut todo_dac = vec![];
    let mut fft_dac_paths: u64 = 0;
    for i in connections.get("fft").unwrap() {
        todo_dac.push((*i, "fft", 1u64));
    }
    while let Some((next, prev, count)) = todo_dac.pop() {
        let item = colouring_dac
            .entry(next.to_string())
            .or_insert((0, false, HashSet::new()));
        item.0 += count;
        item.2.insert(prev);
        if item.2 == *inputs.get(next).unwrap() {
            item.1 = true;
            for i in connections.get(next).unwrap() {
                if *i == "dac" {
                    fft_dac_paths += item.0;
                } else if after_dac.contains(i) {
                    continue;
                } else {
                    todo_dac.push((*i, next, item.0))
                }
            }
        }
    }

    println!(
        "{:}, {:?}, {:?}",
        svr_fft_paths,
        fft_dac_paths,
        dac_out_paths.len()
    );
    svr_fft_paths * fft_dac_paths * dac_out_paths.len() as u64
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
        assert_eq!(part_a(&contents), 5);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_smallb.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 2);
    }
}
