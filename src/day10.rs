pub fn parse(contents: &str) -> Vec<(Vec<bool>, Vec<Vec<u32>>, Vec<i64>)> {
    let mut output = vec![];
    for line in contents.lines() {
        let s = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut bitmap = vec![];
        for c in s[0].chars().skip(1).take(s[0].len() - 2) {
            if c == '.' {
                bitmap.push(false)
            } else {
                bitmap.push(true)
            }
        }
        let mut toggles = vec![];
        for toggle in s.iter().skip(1).take(s.len() - 2) {
            let toggle_digit = toggle
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            toggles.push(toggle_digit);
        }
        let mut joltage = vec![];
        let mut last = s.last().unwrap().chars();
        last.next();
        last.next_back();
        for d in last.as_str().split(',') {
            joltage.push(d.parse::<i64>().unwrap());
        }
        output.push((bitmap, toggles, joltage));
    }
    output
}

fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        })
        .collect()
}

#[aoc(day10, part1)]
pub fn part_a(contents: &str) -> u64 {
    let lines = parse(contents);
    let mut output = 0;
    for (bitmap, toggles, _) in lines {
        let mut target = 0;
        for (idx, val) in bitmap.iter().enumerate() {
            if *val {
                target += 1 << idx as u32
            }
        }
        let mut switches = vec![];
        for t in toggles.iter() {
            let mut switch_value = 0;
            for v in t {
                switch_value += 1 << v;
            }
            switches.push(switch_value);
        }
        let mut min_length = usize::MAX;
        for t in powerset(&switches) {
            let t_len = t.len();
            let val = t.into_iter().fold(target, |x, y| x ^ y);
            if val == 0 && t_len < min_length {
                min_length = t_len;
            }
        }
        output += min_length as u64;
    }
    output
}

#[aoc(day10, part2, faster)]
pub fn part_b_faster(contents: &str) -> i64 {
    let lines = parse(contents);
    let mut output = 0;
    for (idx, (_, toggles, joltage)) in lines.into_iter().enumerate() {
        println!("Doing case {:?}", idx);
        let max_presses = *joltage.iter().max().unwrap();
        let mut switches = vec![];
        for t in toggles.iter() {
            let mut new_vector = vec![0; joltage.len()];
            for v in t {
                new_vector[*v as usize] = 1;
            }
            switches.push(new_vector);
        }
        // println!("{:?}, {:?}", switches.len(), joltage.len());
        let (mut row_reduced, mut t_matrix) = smith_normal_form(switches, joltage);
        let mut solutions: Vec<Option<i64>> = vec![None; row_reduced[0].len()];

        let mut single_rows = row_reduced
            .iter()
            .enumerate()
            .filter(|(_, x)| x.iter().filter(|y| **y != 0).count() == 1)
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();
        while let Some(next) = single_rows.pop() {
            let col = row_reduced[next].iter().position(|x| *x != 0).unwrap();
            solutions[col] = Some(t_matrix[next][0] / row_reduced[next][col]);
            (row_reduced, t_matrix) = row_reduce_updown(&row_reduced, col, next, &t_matrix);
            single_rows = row_reduced
                .iter()
                .enumerate()
                .filter(|(_, x)| x.iter().filter(|y| **y != 0).count() == 1)
                .map(|(idx, _)| idx)
                .collect::<Vec<usize>>();
        }
        let empty_cols = matrix_trans(&row_reduced)
            .iter()
            .enumerate()
            .filter(|(_, x)| x.iter().filter(|y| **y != 0).count() == 0)
            .map(|(idx, _)| idx)
            .collect::<Vec<usize>>();
        for empty in empty_cols {
            match solutions[empty] {
                Some(_) => {}
                None => solutions[empty] = Some(0),
            };
        }
        println!("{:?}, {:?}, {:?}", solutions, row_reduced, t_matrix);
        let target = t_matrix.iter().map(|x| x[0]).collect();
        let intermed = recurse_b(row_reduced, target, solutions, max_presses).unwrap();
        println!("{:?}", intermed);
        output += intermed;
    }
    output
}

fn vector_subtract(vec1: &[i64], vec2: &[i64], times: i64) -> Vec<i64> {
    vec1.iter()
        .zip(vec2.iter())
        .map(|(x, y)| *x - times * *y)
        .collect()
}

fn matrix_trans(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let mut switches_transpose = vec![vec![0; matrix.len()]; matrix[0].len()];
    for idx_i in 0..matrix.len() {
        for idx_j in 0..matrix[0].len() {
            switches_transpose[idx_j][idx_i] = matrix[idx_i][idx_j];
        }
    }
    switches_transpose
}

fn gcd(a: &[i64]) -> i64 {
    if a.len() == 2 {
        return egcd(a[0], a[1]).2;
    }
    let gcd_rest = gcd(&a.iter().skip(1).copied().collect::<Vec<i64>>());
    gcd(&[a[0], gcd_rest])
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r.div_euclid(r);
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_s, old_t, old_r) // (x, y, gcd, such that x * a[0] + y * a[0] == gcd)
}

fn bezout(numbers: &[i64], mut target: i64, max: i64) -> Vec<Vec<i64>> {
    if numbers.is_empty() {
        return vec![vec![]];
    }
    // println!("{:?}, {:?}", numbers, target);
    if numbers.len() == 1 {
        if target / numbers[0] >= 0 && target / numbers[0] <= max {
            return vec![vec![target / numbers[0]]];
        } else {
            return vec![vec![]];
        }
    }
    let common = gcd(numbers);
    let reduced_numbers = numbers.iter().map(|x| x / common).collect::<Vec<i64>>();
    if target % common != 0 {
        return vec![vec![]];
    }
    target /= common;
    // println!("{:?}", reduced_numbers);
    if reduced_numbers.len() == 2 {
        let (x, y, gcd_v) = egcd(reduced_numbers[0], reduced_numbers[1]);
        let mut pairs = vec![];
        for k in -max - x.abs().max(y.abs())..=max + x.abs().max(y.abs()) {
            pairs.push(vec![
                target * x - k * reduced_numbers[1] / gcd_v,
                target * y + k * reduced_numbers[0] / gcd_v,
            ]);
        }
        // println!("{:?}", pairs);
        pairs.retain(|x| x.iter().all(|y| *y <= max && *y >= 0));
        // println!("{:?}", pairs);
        return pairs;
    }
    let s = *reduced_numbers.last().unwrap();
    let mut output = vec![];
    // println!("{:?}, {:?}, {:?}", s, reduced_numbers, target);
    for rem in 0..=max {
        let new_target = target - rem * s;
        let new_pairs = bezout(
            &reduced_numbers
                .iter()
                .copied()
                .take(reduced_numbers.len() - 1)
                .collect::<Vec<i64>>(),
            new_target,
            max,
        );
        for mut n_p in new_pairs {
            if n_p.is_empty() {
                continue;
            }
            n_p.push(rem);
            output.push(n_p);
        }
    }
    output
}

fn row_reduce(
    matrix: &[Vec<i64>],
    pivot: usize,
    pivot_row_idx: usize,
    par_matrix: &[Vec<i64>],
) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let mut pivot_row = matrix[pivot_row_idx].clone();
    let mut pivot_row_par = par_matrix[pivot_row_idx].clone();
    if pivot_row[pivot] < 0 {
        pivot_row = pivot_row.into_iter().map(|x| -x).collect();
        pivot_row_par = pivot_row_par.into_iter().map(|x| -x).collect();
    }

    // println!("Pivot row: {:?}", pivot_row);
    let mut new_matrix = Vec::from_iter(matrix.iter().take(pivot_row_idx).cloned());
    let mut new_par_matrix = Vec::from_iter(par_matrix.iter().take(pivot_row_idx).cloned());
    for (idx, (row, par_row)) in matrix
        .iter()
        .zip(par_matrix.iter())
        .enumerate()
        .skip(pivot_row_idx)
    {
        if idx == pivot_row_idx {
            new_matrix.push(pivot_row.clone());
            new_par_matrix.push(pivot_row_par.clone());
        } else {
            let div = row[pivot] / pivot_row[pivot];
            let mut new_row = vector_subtract(row, &pivot_row, div);
            let mut par_new_row = vector_subtract(par_row, &pivot_row_par, div);

            if let Some(pos) = new_row.iter().position(|x| *x != 0) {
                if new_row[pos] != 1 {
                    let div = new_row[pos];
                    if new_row.iter().all(|x| *x % div == 0)
                        && par_new_row.iter().all(|x| *x % div == 0)
                    {
                        new_row = new_row.into_iter().map(|x| x / div).collect();
                        par_new_row = par_new_row.into_iter().map(|x| x / div).collect();
                    }
                }
                if new_row[pos] < 0 {
                    new_row = new_row.into_iter().map(|x| -x).collect();
                    par_new_row = par_new_row.into_iter().map(|x| -x).collect();
                }
            };
            new_matrix.push(new_row);
            new_par_matrix.push(par_new_row);
        }
    }

    (new_matrix, new_par_matrix)
}

fn row_reduce_updown(
    matrix: &[Vec<i64>],
    pivot: usize,
    pivot_row_idx: usize,
    par_matrix: &[Vec<i64>],
) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let pivot_row = matrix[pivot_row_idx].clone();
    let pivot_row_par = par_matrix[pivot_row_idx].clone();

    let mut new_matrix = vec![];
    let mut new_par_matrix = vec![];
    for (row, par_row) in matrix.iter().zip(par_matrix.iter()) {
        let div = row[pivot] / pivot_row[pivot];
        let mut new_row = vector_subtract(row, &pivot_row, div);
        let mut par_new_row = vector_subtract(par_row, &pivot_row_par, div);

        if let Some(pos) = new_row.iter().position(|x| *x != 0) {
            if new_row[pos] != 1 {
                let div = new_row[pos];
                if new_row.iter().all(|x| *x % div == 0)
                    && par_new_row.iter().all(|x| *x % div == 0)
                {
                    new_row = new_row.into_iter().map(|x| x / div).collect();
                    par_new_row = par_new_row.into_iter().map(|x| x / div).collect();
                }
            }
            if new_row[pos] < 0 {
                new_row = new_row.into_iter().map(|x| -x).collect();
                par_new_row = par_new_row.into_iter().map(|x| -x).collect();
            }
        };
        new_matrix.push(new_row);
        new_par_matrix.push(par_new_row);
    }

    (new_matrix, new_par_matrix)
}

fn smith_normal_form(switches: Vec<Vec<i64>>, joltage: Vec<i64>) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    // Choose pivot: smallest column index with non-zero entry
    let mut switches_transpose = matrix_trans(&switches);
    let mut prev_pivot_row = 0;
    let mut prev_pivot_col = 0;
    let mut t_matrix = vec![];
    for x in joltage.iter() {
        t_matrix.push(vec![*x]);
    }
    println!("starting {:?}, {:?}", switches_transpose, joltage);
    while prev_pivot_col < switches_transpose[0].len() && prev_pivot_row < switches_transpose.len()
    {
        let mut min_col = switches_transpose[0].len();
        let mut cur_row = 0;
        for (row_idx, row) in switches_transpose.iter().skip(prev_pivot_row).enumerate() {
            let min_col_row = row.iter().position(|x| *x == 1 || *x == -1);
            match min_col_row {
                Some(n) => {
                    if n < min_col {
                        min_col = n;
                        cur_row = row_idx + prev_pivot_row;
                    }
                }
                None => continue,
            }
        }
        if min_col == switches_transpose[0].len() {
            break;
        }
        switches_transpose.swap(cur_row, prev_pivot_row);
        // println!("switched {:?}", switches_transpose);
        t_matrix.swap(cur_row, prev_pivot_row);
        (switches_transpose, t_matrix) =
            row_reduce(&switches_transpose, min_col, prev_pivot_row, &t_matrix);
        prev_pivot_col = min_col;
        prev_pivot_row += 1;
    }
    // println!("reduced: {:?}", &switches_transpose);
    // println!("reduced transposes: {:?}", matrix_trans(&switches_transpose));
    // println!("T matrix: {:?}", t_matrix);
    (switches_transpose, t_matrix)
}

fn recurse_b(
    switches: Vec<Vec<i64>>,
    remaining: Vec<i64>,
    solutions: Vec<Option<i64>>,
    max: i64,
) -> Option<i64> {
    // println!("{:?}", remaining);
    // println!("{:?}", solutions);
    if remaining.iter().all(|x| *x == 0) {
        return Some(solutions.iter().fold(0, |v, x| v + x.unwrap_or(0)));
    }

    let mut min_presses: Option<i64> = None;
    let remaining_non_zero: Vec<usize> = remaining
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| (*x != 0).then_some(idx))
        .collect();
    let last_remaining = *remaining_non_zero.last().unwrap();
    let bezout_selection_idx = switches[last_remaining]
        .iter()
        .enumerate()
        .filter_map(|(idx, x)| (*x != 0).then_some(idx))
        .collect::<Vec<usize>>();
    let bezout_coeff = switches[last_remaining]
        .iter()
        .enumerate()
        .filter_map(|(idx, &v)| bezout_selection_idx.contains(&idx).then_some(v))
        .collect::<Vec<i64>>();
    let bezout_pairs = bezout(&bezout_coeff, remaining[last_remaining], max);
    if bezout_pairs.is_empty() || bezout_pairs[0].is_empty() {
        return None;
    }
    // println!("Bezout: {:?}, {:?}, {:?}", bezout_selection_idx, bezout_coeff, bezout_pairs);
    for b in bezout_pairs {
        let mut new_remaining = remaining.clone();
        let mut new_switches = switches.clone();
        for (row_idx, row) in switches.iter().enumerate() {
            for (col_idx, v) in row.iter().enumerate() {
                if let Some(idx) = bezout_selection_idx.iter().position(|x| *x == col_idx) {
                    new_remaining[row_idx] -= b[idx] * *v;
                    new_switches[row_idx][col_idx] = 0;
                }
            }
        }
        let mut new_solutions = solutions.clone();
        for (idx, b_val) in bezout_selection_idx.iter().zip(b.iter()) {
            new_solutions[*idx] = Some(*b_val);
        }
        // println!("Trying {:?}, {:?}, {:?}, {:?}", b, new_switches, new_remaining, new_solutions);
        let cur_min_presses = recurse_b(new_switches, new_remaining, new_solutions, max);
        match (min_presses, cur_min_presses) {
            (Some(nr1), Some(nr)) => min_presses = Some(nr1.min(nr)),
            (None, Some(nr)) => min_presses = Some(nr),
            _ => {}
        }
    }

    min_presses
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
        assert_eq!(part_a(&contents), 7);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b_faster(&contents), 33);
    }

    #[test]
    fn test_bezout() {
        let mut pairs = bezout(&vec![5, 3], 14, 15);
        pairs.sort_by_key(|x| x[0]);
        assert_eq!(pairs, vec![vec![1, 3]]);
    }

    #[test]
    fn test_bezout2() {
        let mut pairs = bezout(&vec![5, 3, 1], 14, 15);
        pairs.sort_by_key(|x| x[0]);
        assert_eq!(
            pairs,
            vec![
                vec![0, 4, 2],
                vec![0, 3, 5],
                vec![0, 2, 8],
                vec![0, 1, 11],
                vec![0, 0, 14],
                vec![1, 3, 0],
                vec![1, 2, 3],
                vec![1, 1, 6],
                vec![1, 0, 9],
                vec![2, 1, 1],
                vec![2, 0, 4]
            ]
        );
    }
}
