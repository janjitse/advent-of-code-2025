use rustc_hash::{FxHashMap, FxHashSet};

use crate::parsers::*;

#[aoc(day11, part1)]
pub fn part_a(contents: &str) -> u64 {
    let vec = parse_string_colon_followed_by_whitespace_separated(contents);
    let mut connections: FxHashMap<String, Vec<&str>> = FxHashMap::default();
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
    let mut connections: FxHashMap<String, Vec<&str>> = FxHashMap::default();
    let mut inputs: FxHashMap<String, FxHashSet<&str>> = FxHashMap::default();

    for (start, out) in vec {
        for o in out.iter() {
            inputs.entry(o.to_string()).or_default().insert(start);
        }
        connections.insert(start.to_string(), out);
    }
    let sources = inputs
        .iter()
        .filter(|(_, set)| set.is_empty())
        .map(|(x, _s)| x)
        .collect::<Vec<&String>>();
    let mut colouring: FxHashMap<String, (u64, FxHashSet<&str>)> = FxHashMap::default();

    let mut todo = vec![];
    for i in connections.get("svr").unwrap() {
        todo.push((*i, "svr", 1u64));
    }
    for s in sources {
        if s != "srv" {
            for i in connections.get(s).unwrap() {
                todo.push((*i, s, 0u64));
            }
        }
    }
    let mut svr_fft_paths = 0;
    while let Some((next, prev, count)) = todo.pop() {
        let item = colouring
            .entry(next.to_string())
            .or_insert((0, FxHashSet::default()));
        item.0 += count;
        item.1.insert(prev);
        if item.1 == *inputs.get(next).unwrap() {
            for i in connections.get(next).unwrap() {
                if *i == "fft" {
                    svr_fft_paths += item.0;
                } else {
                    todo.push((*i, next, item.0))
                }
            }
        }
    }
    for (_, (u, _)) in colouring.iter_mut() {
        *u = 0;
    }

    for i in connections.get("fft").unwrap() {
        todo.push((*i, "fft", 1u64));
    }

    let mut fft_dac_paths = 0;
    while let Some((next, prev, count)) = todo.pop() {
        let item = colouring
            .entry(next.to_string())
            .or_insert((0, FxHashSet::default()));
        item.0 += count;
        item.1.insert(prev);
        // inputs.get_mut(next).unwrap().remove(prev);
        if item.1 == *inputs.get(next).unwrap() {
            for i in connections.get(next).unwrap() {
                if *i == "dac" {
                    fft_dac_paths += item.0;
                } else {
                    todo.push((*i, next, item.0))
                }
            }
        }
    }

    for (_, (u, _)) in colouring.iter_mut() {
        *u = 0;
    }

    for i in connections.get("dac").unwrap() {
        todo.push((*i, "dac", 1u64));
    }

    let mut dac_out_paths = 0;
    while let Some((next, prev, count)) = todo.pop() {
        let item = colouring
            .entry(next.to_string())
            .or_insert((0, FxHashSet::default()));
        item.0 += count;
        item.1.insert(prev);
        if item.1 == *inputs.get(next).unwrap() {
            for i in connections.get(next).unwrap() {
                if *i == "out" {
                    dac_out_paths += item.0;
                } else {
                    todo.push((*i, next, item.0))
                }
            }
        }
    }

    println!(
        "{:}, {:?}, {:?}",
        svr_fft_paths, fft_dac_paths, dac_out_paths
    );
    svr_fft_paths * fft_dac_paths * dac_out_paths
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
