use std::collections::HashSet;

pub fn parse(contents: &str) -> Vec<(u64, u64)> {
    let mut vec = vec![];
    for ids in contents.trim().split(',') {
        let id_vec: Vec<u64> = ids.split('-').map(|x| x.parse().unwrap()).collect();
        vec.push((id_vec[0], id_vec[1]));
    }
    vec
}

#[aoc(day2, part1)]
pub fn part_a(contents: &str) -> u64 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    let max_id = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let mut all_invalid: HashSet<u64> = HashSet::new();
    let num_digits = max_id.to_string().chars().count() / 2;
    let max_pos_invalid: u64 = String::from_iter(vec!['9'; num_digits]).parse().unwrap();
    for i in 1..=max_pos_invalid {
        let invalid_id: u64 = (i.to_string() + &i.to_string()).parse().unwrap();
        all_invalid.insert(invalid_id);
    }
    for (id_start, id_end) in vec {
        for &invalid_id in all_invalid.iter() {
            if invalid_id <= id_end && invalid_id >= id_start {
                invalid_id_sum += invalid_id;
            }
        }
    }
    invalid_id_sum
}

#[aoc(day2, part1, binary_search)]
pub fn part_a_binary(contents: &str) -> u64 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    let max_id = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let mut all_invalid_vec = vec![];
    let num_digits = max_id.to_string().chars().count() / 2;
    let max_pos_invalid: u64 = String::from_iter(vec!['9'; num_digits]).parse().unwrap();
    for i in 1..=max_pos_invalid {
        let invalid_id: u64 = (i.to_string() + &i.to_string()).parse().unwrap();
        all_invalid_vec.push(invalid_id);
    }
    all_invalid_vec.sort_unstable();
    all_invalid_vec.dedup();
    for (id_start, id_end) in vec {
        let start_idx = all_invalid_vec.partition_point(|&x| x < id_start);
        let end_idx = all_invalid_vec.partition_point(|&x| x <= id_end);
        invalid_id_sum += all_invalid_vec.iter().take(end_idx).skip(start_idx).sum::<u64>();
    }
    invalid_id_sum
}

#[aoc(day2, part2)]
pub fn part_b(contents: &str) -> u64 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    let max_id = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let mut all_invalid: HashSet<u64> = HashSet::new();
    let max_num_digits = max_id.to_string().chars().count();
    for rep in 2..max_num_digits {
        let num_digits = max_id.to_string().chars().count() / rep;
        let max_pos_invalid: u64 = String::from_iter(vec!['9'; num_digits]).parse().unwrap();
        for i in 1..=max_pos_invalid {
            let invalid_string: Vec<String> = vec![i; rep].iter().map(|x| x.to_string()).collect();
            let invalid_id: u64 = invalid_string.join("").parse().unwrap();
            all_invalid.insert(invalid_id);
        }
    }

    for (id_start, id_end) in vec {
        for &invalid_id in all_invalid.iter() {
            if invalid_id <= id_end && invalid_id >= id_start {
                invalid_id_sum += invalid_id;
            }
        }
    }
    invalid_id_sum
}

#[aoc(day2, part2, binary_search)]
pub fn part_b_binary(contents: &str) -> u64 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    let max_id = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let mut all_invalid_vec = vec![];
    let max_num_digits = max_id.to_string().chars().count();
    for rep in 2..=max_num_digits {
        let num_digits = max_id.to_string().chars().count() / rep;
        let max_pos_invalid: u64 = String::from_iter(vec!['9'; num_digits]).parse().unwrap();
        for i in 1..=max_pos_invalid {
            let invalid_string: Vec<String> = vec![i; rep].iter().map(|x| x.to_string()).collect();
            let invalid_id: u64 = invalid_string.join("").parse().unwrap();
            all_invalid_vec.push(invalid_id);
        }
    }
    all_invalid_vec.sort_unstable();
    all_invalid_vec.dedup();
    for (id_start, id_end) in vec {
        let start_idx = all_invalid_vec.partition_point(|&x| x < id_start);
        let end_idx = all_invalid_vec.partition_point(|&x| x <= id_end);
        invalid_id_sum += all_invalid_vec.iter().take(end_idx).skip(start_idx).sum::<u64>();
    }
    invalid_id_sum
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
        assert_eq!(part_a(&contents), 1227775554);
    }

    #[test]
    fn test_1_binary() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_a_binary(&contents), 1227775554);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 4174379265);
    }
}
