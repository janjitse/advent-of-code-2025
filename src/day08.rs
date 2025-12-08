use bit_set::BitSet;

use crate::parsers::*;

#[aoc(day8, part1)]
pub fn part_a(contents: &str) -> i64 {
    let vec = parse_comma_separated_ints(contents);
    let nr_pairs = match vec.len() {
        20 => 10,
        _ => 1000,
    };
    let mut distances = vec![];
    for (idx_1, vec_1) in vec.iter().enumerate() {
        for (idx_2, vec_2) in vec.iter().skip(idx_1 + 1).enumerate() {
            let distance = (vec_1[0] - vec_2[0]) * (vec_1[0] - vec_2[0])
                + (vec_1[1] - vec_2[1]) * (vec_1[1] - vec_2[1])
                + (vec_1[2] - vec_2[2]) * (vec_1[2] - vec_2[2]);
            distances.push((distance, idx_1, idx_2 + idx_1 + 1));
        }
    }
    distances.select_nth_unstable_by_key(nr_pairs, |x| x.0);
    let mut heaps: Vec<BitSet> = vec![];
    for (_, idx_1, idx_2) in distances.into_iter().take(nr_pairs) {
        let mut merged_heaps = BitSet::new();
        for (heap_idx, h) in heaps.iter_mut().enumerate() {
            if h.contains(idx_1) || h.contains(idx_2) {
                merged_heaps.insert(heap_idx);
            }
        }
        let mut new_heaps = vec![];
        let mut merged_heap: BitSet = BitSet::from_iter(vec![idx_1, idx_2]);
        for (idx, h) in heaps.into_iter().enumerate() {
            if merged_heaps.contains(idx) {
                merged_heap.union_with(&h);
                continue;
            }
            new_heaps.push(h);
        }
        new_heaps.push(merged_heap);
        heaps = new_heaps;
    }
    heaps.sort_unstable_by_key(|x| -(x.len() as i64));
    let mut total = 1;
    for h in heaps.iter().take(3) {
        total *= h.len() as i64
    }

    total
}

#[aoc(day8, part2)]
pub fn part_b(contents: &str) -> i64 {
    let vec = parse_comma_separated_ints(contents);
    let mut distances = vec![];
    for (idx_1, vec_1) in vec.iter().enumerate() {
        for (idx_2, vec_2) in vec.iter().skip(idx_1 + 1).enumerate() {
            let distance = (vec_1[0] - vec_2[0]) * (vec_1[0] - vec_2[0])
                + (vec_1[1] - vec_2[1]) * (vec_1[1] - vec_2[1])
                + (vec_1[2] - vec_2[2]) * (vec_1[2] - vec_2[2]);
            distances.push((distance, idx_1, idx_2 + idx_1 + 1));
        }
    }
    distances.sort_unstable_by_key(|x| x.0);
    let mut heaps: Vec<BitSet> = vec![];
    for (_, idx_1, idx_2) in distances {
        let mut merged_heaps = BitSet::new();
        for (heap_idx, h) in heaps.iter_mut().enumerate() {
            if h.contains(idx_1) || h.contains(idx_2) {
                merged_heaps.insert(heap_idx);
            }
        }
        let mut new_heaps = vec![];
        let mut merged_heap: BitSet = BitSet::from_iter(vec![idx_1, idx_2]);
        for (idx, h) in heaps.into_iter().enumerate() {
            if merged_heaps.contains(idx) {
                merged_heap.union_with(&h);
                // merged_heap = FxHashSet::from_iter(merged_heap.union(h).copied());
                continue;
            }
            new_heaps.push(h);
        }
        new_heaps.push(merged_heap);
        heaps = new_heaps;

        if heaps[0].len() == vec.len() {
            return vec[idx_2][0] * vec[idx_1][0];
        }
    }
    println!("{:?}", heaps);
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
