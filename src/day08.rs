use bit_set::BitSet;

use crate::parsers::*;

#[aoc(day8, part1)]
pub fn part_a(contents: &str) -> i64 {
    let vec = parse_comma_separated_ints(contents);
    let nr_pairs = match vec.len() {
        20 => 10,
        _ => 1000,
    };
    let mut distances = Vec::with_capacity((vec.len() * (vec.len() - 1)) / 2);
    for (idx_1, vec_1) in vec.iter().enumerate() {
        for (idx_2, vec_2) in vec.iter().skip(idx_1 + 1).enumerate() {
            let distance = (vec_1.0 - vec_2.0).pow(2)
                + (vec_1.1 - vec_2.1).pow(2)
                + (vec_1.2 - vec_2.2).pow(2);
            distances.push((distance, idx_1, idx_2 + idx_1 + 1));
        }
    }
    distances.select_nth_unstable_by_key(nr_pairs, |x| x.0);
    let mut heaps: Vec<BitSet> = vec![];
    for (_, idx_1, idx_2) in distances.into_iter().take(nr_pairs) {
        let mut merged_heaps = Vec::with_capacity(2);
        for (heap_idx, h) in heaps.iter().enumerate() {
            if h.contains(idx_1) || h.contains(idx_2) {
                merged_heaps.push(heap_idx);
            }
        }
        let merged_heap: BitSet = BitSet::from_iter(vec![idx_1, idx_2]);
        match merged_heaps.len() {
            2 => {
                let (before, after) = heaps.split_at_mut(merged_heaps[0] + 1);
                before[merged_heaps[0]].union_with(&after[merged_heaps[1] - merged_heaps[0] - 1]);
                heaps.remove(merged_heaps[1]);
            }
            1 => {
                heaps[merged_heaps[0]].union_with(&merged_heap);
            }
            _ => heaps.push(merged_heap),
        };
    }
    heaps.select_nth_unstable_by_key(3, |x| -(x.len() as i64));
    let mut total = 1;
    for h in heaps.iter().take(3) {
        total *= h.len() as i64
    }
    total
}

#[aoc(day8, part2)]
pub fn part_b(contents: &str) -> i64 {
    let vec = parse_comma_separated_ints(contents);
    let mut distances = Vec::with_capacity((vec.len() * (vec.len() - 1)) / 2);
    for (idx_1, vec_1) in vec.iter().enumerate() {
        for (idx_2, vec_2) in vec.iter().skip(idx_1 + 1).enumerate() {
            let distance = (vec_1.0 - vec_2.0).pow(2)
                + (vec_1.1 - vec_2.1).pow(2)
                + (vec_1.2 - vec_2.2).pow(2);
            distances.push((distance, idx_1, idx_2 + idx_1 + 1));
        }
    }
    distances.sort_unstable_by_key(|x| x.0);
    let mut heaps: Vec<BitSet> = Vec::with_capacity(vec.len());
    for (_, idx_1, idx_2) in distances {
        let mut merged_heaps = Vec::with_capacity(2);
        for (heap_idx, h) in heaps.iter().enumerate() {
            if h.contains(idx_1) || h.contains(idx_2) {
                merged_heaps.push(heap_idx);
            }
        }
        let merged_heap: BitSet = BitSet::from_iter(vec![idx_1, idx_2]);
        match merged_heaps.len() {
            2 => {
                let (before, after) = heaps.split_at_mut(merged_heaps[0] + 1);
                before[merged_heaps[0]].union_with(&after[merged_heaps[1] - merged_heaps[0] - 1]);
                heaps.remove(merged_heaps[1]);
            }
            1 => {
                heaps[merged_heaps[0]].union_with(&merged_heap);
            }
            _ => heaps.push(merged_heap),
        };
        if heaps[0].len() == vec.len() {
            return vec[idx_2].0 * vec[idx_1].0;
        }
    }
    unreachable!()
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
        assert_eq!(part_a(&contents), 40);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 25272);
    }
}
