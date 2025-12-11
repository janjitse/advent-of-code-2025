use std::collections::{HashMap, HashSet};

use crate::parsers::*;

#[aoc(day9, part1)]
pub fn part_a(contents: &str) -> i64 {
    let vec = parse_comma_separated_ints2(contents);
    let mut max_rectangle = 0;
    for (idx1, (y1, x1)) in vec.iter().enumerate() {
        for (y2, x2) in vec.iter().skip(idx1 + 1) {
            let rectangle_size = ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1);
            if rectangle_size > max_rectangle {
                max_rectangle = rectangle_size;
            }
        }
    }
    // println!("{:?}", vec);
    max_rectangle
}

fn create_compressed(coordinates: Vec<i64>) -> (Vec<i64>, HashMap<i64, usize>) {
    let mut compress_addition = vec![];
    for v in coordinates.iter() {
        compress_addition.push(v + 1);
    }
    compress_addition.push(0);
    compress_addition.push(i64::MAX);
    compress_addition.extend_from_slice(&coordinates);
    compress_addition.sort_unstable();
    compress_addition.dedup();
    let mut inverse_coordinates = HashMap::new();
    for (idx, v) in compress_addition.iter().enumerate() {
        inverse_coordinates.insert(*v, idx as usize);
    }
    (compress_addition, inverse_coordinates)
}

#[aoc(day9, part2)]
pub fn part_b(contents: &str) -> i64 {
    let vec = parse_comma_separated_ints2(contents);
    let (compressed_coordinates_x, inverse_compressed_x) =
        create_compressed(vec.iter().map(|x| x.0).collect::<Vec<i64>>());
    let (compressed_coordinates_y, inverse_compressed_y) =
        create_compressed(vec.iter().map(|x| x.1).collect::<Vec<i64>>());
    let mut green_lines = HashSet::new();
    for (idx1, (x1, y1)) in vec.iter().enumerate() {
        for (x2, y2) in vec.iter().cycle().skip(idx1 + 1).take(1) {
            if y1 == y2 {
                let y1_c = inverse_compressed_y[y1];
                let x1_c = inverse_compressed_x[x1];
                let x2_c = inverse_compressed_x[x2];
                for r in x1_c.min(x2_c)..=x1_c.max(x2_c) {
                    green_lines.insert((r, y1_c));
                }
            } else if x1 == x2 {
                let y1_c = inverse_compressed_y[y1];
                let y2_c = inverse_compressed_y[y2];
                let x1_c = inverse_compressed_x[x1];
                for r in y1_c.min(y2_c)..=y1_c.max(y2_c) {
                    green_lines.insert((x1_c, r));
                }
            }
        }
    }
    let mut todo = vec![];
    let mut filled = vec![vec![0; compressed_coordinates_x.len()]; compressed_coordinates_y.len()];
    let directions = [(0, 1), (1, 0), (usize::MAX, 0), (0, usize::MAX)];
    todo.push((0_usize, 0_usize));
    while let Some(next) = todo.pop() {
        for dir in directions.iter() {
            let new_next = (next.0.wrapping_add(dir.0), next.1.wrapping_add(dir.1));
            if new_next.0 < filled[0].len()
                && new_next.1 < filled.len()
                && filled[new_next.1][new_next.0] == 0
                && !green_lines.contains(&new_next)
            {
                filled[new_next.1][new_next.0] = 1;
                todo.push(new_next);
            }
        }
    }
    let mut prefix_sum =
        vec![vec![0; compressed_coordinates_x.len()]; compressed_coordinates_y.len()];
    for idx_y in 0..compressed_coordinates_y.len() {
        for idx_x in 0..compressed_coordinates_x.len() {
            let up_left = *prefix_sum
                .get(idx_y.wrapping_sub(1))
                .unwrap_or(&vec![0])
                .get(idx_x.wrapping_sub(1))
                .unwrap_or(&0);
            let up = *prefix_sum
                .get(idx_y.wrapping_sub(1))
                .unwrap_or(&vec![0])
                .get(idx_x)
                .unwrap_or(&0);
            let left = *prefix_sum
                .get(idx_y)
                .unwrap_or(&vec![0])
                .get(idx_x.wrapping_sub(1))
                .unwrap_or(&0);
            prefix_sum[idx_y][idx_x] = filled[idx_y][idx_x] + up + left - up_left;
        }
    }

    let mut max_rectangle = 0;
    for (idx1, (x1, y1)) in vec.iter().enumerate() {
        for (x2, y2) in vec.iter().skip(idx1 + 1) {
            let rectangle_size = ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1);
            if rectangle_size > max_rectangle {
                let y1_c = inverse_compressed_y[y1];
                let y2_c = inverse_compressed_y[y2];
                let x1_c = inverse_compressed_x[x1];
                let x2_c = inverse_compressed_x[x2];
                let corner_left_up = (x1_c.min(x2_c), y1_c.min(y2_c));
                let corner_left_down = (x1_c.min(x2_c), y1_c.max(y2_c));
                let corner_right_down = (x1_c.max(x2_c), y1_c.max(y2_c));
                let corner_right_up = (x1_c.max(x2_c), y1_c.min(y2_c));
                let value_left_up = prefix_sum[corner_left_up.1][corner_left_up.0];
                let value_left_down = prefix_sum[corner_left_down.1][corner_left_down.0];
                let value_right_up = prefix_sum[corner_right_up.1][corner_right_up.0];
                let value_right_down = prefix_sum[corner_right_down.1][corner_right_down.0];
                if value_right_down - value_left_down - value_right_up + value_left_up == 0 {
                    max_rectangle = rectangle_size;
                }
            }
        }
    }
    max_rectangle
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
        assert_eq!(part_a(&contents), 50);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 24);
    }
}
