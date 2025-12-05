use crate::parsers::*;

#[aoc(day5, part1)]
pub fn part_a(contents: &str) -> u64 {
    let (mut ranges, ids) = parse_ranges_followed_by_lists(contents);
    let mut total_fresh = 0;
    ranges.sort_unstable_by_key(|x| x.0);
    let mut range_merged = Vec::with_capacity(ranges.len());
    range_merged.push(ranges[0]);
    for range in ranges.into_iter().skip(1) {
        if range.0 > range_merged.last().unwrap().1 + 1 {
            range_merged.push((range.0, range.1));
        } else {
            let to_modify = range_merged.last_mut().unwrap();
            to_modify.1 = to_modify.1.max(range.1);
        }
    }
    for id in ids {
        let partition_point = range_merged.binary_search_by_key(&id, |x| x.0);
        match partition_point {
            Ok(_) => {
                total_fresh += 1;
            }
            Err(p) => {
                if p > 0 && range_merged[p - 1].1 >= id {
                    total_fresh += 1;
                }
            }
        };
    }
    total_fresh
}

#[aoc(day5, part2)]
pub fn part_b(contents: &str) -> u64 {
    let (mut ranges, _) = parse_ranges_followed_by_lists(contents);
    ranges.sort_unstable_by_key(|x| x.0);
    let mut last_end = ranges[0].1;
    let mut total_fresh = ranges[0].1 - ranges[0].0 + 1;
    for range in ranges.iter().skip(1) {
        if range.0 > last_end + 1 {
            total_fresh += range.1 - range.0 + 1;
            last_end = range.1;
        } else if range.1 > last_end {
            total_fresh += range.1 - last_end;
            last_end = range.1;
        }
    }
    total_fresh
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
        assert_eq!(part_a(&contents), 3);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 14);
    }
}
