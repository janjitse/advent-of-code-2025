use itertools::min;

 pub fn parse(contents: &str) -> Vec<(Vec<bool>, Vec<Vec<u32>>, Vec<i64>)> {
    let mut output = vec![];
    for line in contents.lines() {
        let s = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let mut bitmap = vec![];
        for c in s[0].chars().skip(1).take(s[0].len()-2) {
            if c == '.' {bitmap.push(false)} else {bitmap.push(true)}
        }
        let mut toggles =vec![];
        for toggle in s.iter().skip(1).take(s.len() - 2) {
            let toggle_digit = toggle.chars().filter(|c| c.is_digit(10)).map(|x| x.to_digit(10).unwrap()).collect::<Vec<u32>>();
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

fn powerset<T>(s: &[T]) -> Vec<Vec<T>> where T: Clone {
    (0..2usize.pow(s.len() as u32)).map(|i| {
         s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
                             .map(|(_, element)| element.clone())
                             .collect()
     }).collect()
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
        // println!("{:?}", switches);
        for t in powerset(&switches) {
            // println!("{:?}", t);
            let t_len = t.len();
            let val = t.into_iter().fold(target, |x, y| x^y);
            // println!("{:?}, {:?}", target, val);
            if val == 0 {
                if t_len < min_length {
                    min_length = t_len;
                }
            }
            
        }
        output += min_length as u64;

    }
    output
}

// #[aoc(day10, part2)]
#[allow(dead_code)]
pub fn part_b(contents: &str) -> u64 {
    let lines = parse(contents);
    let mut output = 0;
    for (idx, (_, toggles, joltage)) in lines.into_iter().enumerate() {
        let mut switches = vec![];
        for t in toggles.iter() {
            let mut new_vector = vec![0; joltage.len()];
            for v in t {
                new_vector[*v as usize] = 1;
            }
            switches.push(new_vector);
        }
        switches.sort_unstable_by_key(|x| -x.iter().fold(0, |x,y| - x - y));
        let min_length = recurse(switches, joltage).unwrap();
        println!("Done line {:?}, min presses {:?}", idx, min_length);
        output += min_length as u64;

    }
    output
}


#[aoc(day10, part2, faster)]
pub fn part_b_faster(contents: &str) -> i64 {
    let lines = parse(contents);
    let mut output = 0;
    for (_idx, (_, toggles, mut joltage)) in lines.into_iter().enumerate() {
        let mut switches = vec![];
        for t in toggles.iter() {
            let mut new_vector = vec![0; joltage.len()];
            for v in t {
                new_vector[*v as usize] = 1;
            }
            switches.push(new_vector);
        }
        println!("{:?}, {:?}", switches.len(), joltage.len());
        let (mut row_reduced, mut t_matrix) = smith_normal_form(switches);
        // let min_length = recurse(matrix_trans(&row_reduced), joltage).unwrap();
        // output += min_length;
        // let matrix = ndarray::Array2::from(switches);
        // let min_length = recurse(switches, joltage).unwrap();
        // let matrix = dmatrix![switches];
        // println!("{:?}", matrix);
        // println!("Done line {:?}, min presses {:?}", idx, min_length);
        // output += min_length as u64;

    }
    output
}


fn vector_subtract(vec1: &Vec<i64>, vec2: &Vec<i64>, times: i64) -> Vec<i64> {
    vec1.iter().zip(vec2.iter()).map(|(x, y)| *x - times * *y).collect()
}

fn matrix_trans(matrix: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut switches_transpose = vec![vec![0; matrix.len()]; matrix[0].len()];
    for idx_i in 0..matrix.len() {
        for idx_j in 0..matrix[0].len() {
            switches_transpose[idx_j][idx_i] = matrix[idx_i][idx_j];
        }
    }
    switches_transpose
}

fn identity_matrix(size: usize) -> Vec<Vec<i64>> {
    let mut output = vec![vec![0; size]; size];
    for idx in 0..size {
        output[idx][idx] = 1;
    }
    output
}

fn row_reduce(matrix: &Vec<Vec<i64>>, pivot: usize, pivot_row_idx: usize, par_matrix: &Vec<Vec<i64>>) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    let matrix_mut = matrix.clone();
    let par_matrix_mut = par_matrix.clone();
    let mut pivot_row = matrix_mut[pivot_row_idx].clone();
    let mut pivot_row_par = par_matrix_mut[pivot].clone();
    if pivot_row[pivot] < 0 {
        pivot_row = pivot_row.into_iter().map(|x| -1 * x).collect();
        pivot_row_par = pivot_row_par.into_iter().map(|x| -1 * x).collect();
    }
    println!("Pivot row: {:?}", pivot_row);
    let mut new_matrix = vec![];
    let mut new_par_matrix = vec![];
    for (idx, (row, par_row)) in matrix_mut.into_iter().zip(par_matrix_mut.into_iter()).enumerate() {
        if idx < pivot_row_idx {
            new_matrix.push(row);
            new_par_matrix.push(par_row);
        } else if idx == pivot_row_idx {
            new_matrix.push(pivot_row.clone());
            new_par_matrix.push(pivot_row_par.clone());
        } else {
            let div = row[pivot] / pivot_row[pivot];
            let new_row = vector_subtract(&row, &pivot_row,div);
            let par_new_row = vector_subtract(&par_row, &pivot_row_par,div);
            new_matrix.push(new_row);
            new_par_matrix.push(par_new_row);
        }
    }

    return (new_matrix, new_par_matrix)
}

fn matrix_vec_mul(matrix: &Vec<Vec<i64>>, vector: &Vec<i64>) -> Vec<i64> {
    let mut output = vec![0; matrix.len()];
    for (idx, l ) in matrix.iter().enumerate() {
        output[idx] = l.iter().zip(vector).fold(0,|v, (x,y)|  v + *x * *y);
    }
    output
}

fn smith_normal_form(switches: Vec<Vec<i64>>) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    // Choose pivot: smallest column index with non-zero entry
    // let mut switches_transpose = matrix_trans(&switches);
    let mut switches_transpose = switches.clone();
    let mut prev_pivot_row = 0;
    let mut prev_pivot_col = 0;
    let mut t_matrix = identity_matrix(switches_transpose.len());
    println!("starting {:?}", switches_transpose);
    while prev_pivot_col < switches_transpose[0].len() && prev_pivot_row < switches_transpose.len() {
        let mut min_col = switches_transpose[0].len();
        let mut cur_row = 0;
        for (row_idx, row) in switches_transpose.iter().skip(prev_pivot_row).enumerate() {
            let min_col_row = row.iter().position(|x| *x != 0);
            match min_col_row {
                Some(n) => {
                    if n < min_col {
                        min_col = n;
                        cur_row = row_idx + prev_pivot_row;
                    }
                },
                None => {continue}
            }
        }
        if min_col == switches_transpose[0].len() {
            break;
        }
        println!("pivot found: row{:?}, col {:?}", cur_row, min_col);
        switches_transpose.swap(cur_row, prev_pivot_row);
        println!("switched {:?}", switches_transpose);
        t_matrix.swap(cur_row, prev_pivot_row);
        (switches_transpose, t_matrix) = row_reduce(&switches_transpose, min_col, prev_pivot_row, &t_matrix);
        prev_pivot_col = min_col;
        prev_pivot_row = prev_pivot_row+1;
    }
    println!("reduced: {:?}", &switches_transpose);
    println!("reduced transposes: {:?}", matrix_trans(&switches_transpose));
    println!("T matrix: {:?}", t_matrix);
    // Improve pivot, use Bezout?
    // Eliminate entries
    return (matrix_trans(&switches_transpose), matrix_trans(&t_matrix))
}

fn recurse(mut switches: Vec<Vec<i64>>, mut remaining: Vec<i64>) -> Option<i64>{
    if remaining.iter().all(|x| *x == 0) {
        return Some(0)
    }
    if remaining.iter().any(|x| *x< 0) {
        return None
    }
    if switches.is_empty() {
        return None
    }
    let remaining_non_zero: Vec<usize> = remaining.iter().enumerate().filter(|(_idx, x)| **x > 0).map(|(idx, _)| idx).collect();
    let first_remaining = remaining_non_zero[0];
    let mut vec_to_take = 0;
    for (idx, s) in switches.iter().enumerate() {
        if s[first_remaining] > 0 {
            vec_to_take = idx;
            break;
        }
    }
    // if switches[vec_to_take] {}
    let test_vec = switches.remove(vec_to_take);

    let mut min_presses = i64::MAX;
    let to_remove = remaining[first_remaining];
    for presses in 0..=to_remove {
        let cur_pres = recurse(switches.clone(), remaining.clone());
        match cur_pres {
            Some(number) => {min_presses = min_presses.min(presses + number as i64);},
            None => {},
        }
        for (idx, val) in test_vec.iter().enumerate() {
            remaining[idx] -= *val as i64;
        }
        if remaining.iter().any(|x| *x< 0) {
            break
        }
        

    }
    if min_presses == i64::MAX {
        return None
    }
    return Some(min_presses)
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
}