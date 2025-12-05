use crate::parsers::*;

#[aoc(day5, part1)]
pub fn part_a(contents: &str) -> i32 {
    let (ranges, mut ids) = parse_ranges_followed_by_lists(contents);
    let mut total_fresh = 0;
    ids.sort_unstable();
    for id in ids {
        let mut fresh = false;
        for range in ranges.iter() {
            if range[0] <= id && range[1] >= id {
                fresh = true;
                break;
            }
        }
        if fresh {
            total_fresh += 1;
        }
    }
    total_fresh
}

#[aoc(day5, part2)]
pub fn part_b(contents: &str) -> u128 {
    let (mut ranges, _) = parse_ranges_followed_by_lists(contents);
    ranges.sort_by_key(|x| x[0]);
    let mut range_merged = vec![(ranges[0][0], ranges[0][1])];
    for range in ranges.iter().skip(1) {
        if range[0] > range_merged.last().unwrap().1 {
            range_merged.push((range[0], range[1]));
        } else if range[1] > range_merged.last().unwrap().1 {
            let to_modify = range_merged.last_mut().unwrap();
            to_modify.1 = range[1];
        }
    }
    let mut total_fresh = 0;
    for (start, end) in range_merged {
        total_fresh += (end - start + 1) as u128;
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
