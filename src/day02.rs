use std::cmp::max;
use std::collections::HashSet;

pub fn parse(contents: &str) -> Vec<(u64, u64)> {
    let mut vec = vec![];
    for ids in contents.trim().split(',') {
        let id_vec: Vec<u64> = ids.split('-').map(|x| x.parse().unwrap()).collect();
        vec.push((id_vec[0], id_vec[1]));
    }
    vec
}

fn num_digits(n: u64, b: u32) -> usize {
    (0..).take_while(|i| (b as u64).pow(*i) <= n).count()
}

// #[aoc(day2, part1)]
#[allow(dead_code)]
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

// #[aoc(day2, part1, binary_search)]
#[allow(dead_code)]
pub fn part_a_binary(contents: &str) -> u64 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    let max_id = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let mut all_invalid_vec = vec![];
    let max_num_digits = num_digits(max_id, 10) / 2;
    let max_pos_invalid: u64 = String::from_iter(vec!['9'; max_num_digits])
        .parse()
        .unwrap();
    for i in 1..=max_pos_invalid {
        let invalid_id = i + i * 10u64.pow(num_digits(i, 10) as u32);
        if invalid_id > max_id {
            break;
        }
        all_invalid_vec.push(invalid_id);
    }
    all_invalid_vec.sort_unstable();
    for (id_start, id_end) in vec {
        let start_idx = all_invalid_vec.partition_point(|&x| x < id_start);
        let end_idx = all_invalid_vec.partition_point(|&x| x <= id_end);
        invalid_id_sum += all_invalid_vec
            .iter()
            .take(end_idx)
            .skip(start_idx)
            .sum::<u64>();
    }
    invalid_id_sum
}

#[aoc(day2, part1, smarter)]
// #[allow(dead_code)]
pub fn part_a_smarter(contents: &str) -> u128 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    for (id_start, id_end) in vec {
        let digits_start = (num_digits(id_start, 10) as f64 / 2.0).ceil() as usize;
        let digits_end = max(num_digits(id_end, 10) / 2, 1);
        for digits in digits_start..=digits_end {
            let factor = 10u64.pow(digits as u32) + 1;

            let mut invalid_check_start = 10u64.pow(digits as u32 - 1).max(id_start / factor);
            while invalid_check_start * factor < id_start {
                invalid_check_start += 1;
            }

            let end_invalid_ids = (10u64.pow(digits as u32) - 1).min(id_end / factor);
            if end_invalid_ids < invalid_check_start {
                continue;
            }
            let n = end_invalid_ids - invalid_check_start;
            invalid_id_sum += factor as u128
                * ((n + 1) as u128 * invalid_check_start as u128
                    + ((n as u128 * (n as u128 + 1)) / 2));
        }
    }
    invalid_id_sum
}

// #[aoc(day2, part2)]
#[allow(dead_code)]
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

// #[aoc(day2, part2, binary_search)]
#[allow(dead_code)]
pub fn part_b_binary(contents: &str) -> u128 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    let max_id = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let mut all_invalid_vec = vec![];
    let max_num_digits = num_digits(max_id, 10);
    for rep in 2..=max_num_digits {
        let mut i = 1;
        loop {
            let digits_i = num_digits(i, 10) as u32;
            let invalid_id = (0..rep).fold(0, |acc, r| acc + i * 10u64.pow(digits_i * r as u32));
            if invalid_id > max_id {
                break;
            }
            all_invalid_vec.push(invalid_id);
            i += 1
        }
    }
    all_invalid_vec.sort_unstable();
    all_invalid_vec.dedup();
    for (id_start, id_end) in vec {
        let start_idx = all_invalid_vec.partition_point(|&x| x < id_start);
        let end_idx = all_invalid_vec.partition_point(|&x| x <= id_end);
        invalid_id_sum += all_invalid_vec
            .iter()
            .take(end_idx)
            .skip(start_idx)
            .map(|&x| x as u128)
            .sum::<u128>();
    }
    invalid_id_sum
}

fn determine_sign(mut num_digits: usize) -> i32 {
    let small_primes = [2, 3, 5, 7, 11]; // Should be enough up to 121 digit numbers
    let mut nr_prime_div = 0;
    for p in small_primes {
        let mut nr_divisors = 0;
        while num_digits.is_multiple_of(p) {
            num_digits /= p;
            nr_divisors += 1;
        }
        match nr_divisors {
            2.. => return 0,
            1 => {
                nr_prime_div += 1;
            }
            _ => continue,
        }
    }
    if num_digits > 1 {
        nr_prime_div += 1;
    }
    match nr_prime_div % 2 {
        0 => -1,
        _ => 1,
    }
}

#[aoc(day2, part2, smarter)]
pub fn part_b_smarter(contents: &str) -> i128 {
    let vec = parse(contents);
    let mut invalid_id_sum = 0;
    let max_id = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let max_num_digits = num_digits(max_id, 10);
    for rep in 2..=max_num_digits {
        let sign = determine_sign(rep);
        if sign == 0 {
            continue;
        }
        for (id_start, id_end) in vec.iter() {
            if num_digits(*id_end, 10) < rep {
                continue;
            }
            let digits_start = (num_digits(*id_start, 10) as f64 / rep as f64).ceil() as usize;
            let digits_end = num_digits(*id_end, 10) / rep;

            for digits in digits_start..=digits_end {
                let mut factor = 0;

                for r in 0..rep {
                    factor += 10u64.pow(r as u32 * digits as u32);
                }

                let mut invalid_check_start = 10u64.pow(digits as u32 - 1).max(*id_start / factor);

                if invalid_check_start * factor < *id_start {
                    invalid_check_start += 1;
                }

                let end_invalid_ids = (10u64.pow(digits as u32) - 1).min(id_end / factor);

                if end_invalid_ids < invalid_check_start {
                    continue;
                }
                let n = end_invalid_ids - invalid_check_start;
                invalid_id_sum += sign as i128
                    * factor as i128
                    * ((n + 1) as i128 * invalid_check_start as i128
                        + ((n as i128 * (n as i128 + 1)) / 2));
            }
        }
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
    fn test_1_smarter() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_a_smarter(&contents), 1227775554);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b_smarter(&contents), 4174379265);
    }
}
