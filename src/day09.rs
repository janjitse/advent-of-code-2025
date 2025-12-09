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

#[aoc(day9, part2)]
pub fn part_b(contents: &str) -> i64 {
    let vec = parse_comma_separated_ints2(contents);
    let max_x = vec.iter().max_by_key(|x| x.0).unwrap().0;
    let max_y = vec.iter().max_by_key(|x| x.1).unwrap().1;
    let mut min_max_y = vec![(i64::MAX, 0); max_x.abs() as usize + 1];
    let mut min_max_x = vec![(i64::MAX, 0); max_y.abs() as usize + 1];
    for (idx1, (x1, y1)) in vec.iter().enumerate() {
        for (x2, y2) in vec.iter().skip(idx1 + 1) {
            if y1 == y2 {
                for r in *x1.min(x2)..=*x1.max(x2) {
                    min_max_y[r as usize].0 = *y1.min(&min_max_y[r as usize].0);
                    min_max_y[r as usize].1 = *y1.max(&min_max_y[r as usize].1);
                }
            } else if x1 == x2 {
                for r in *y1.min(y2)..=*y1.max(y2) {
                    min_max_x[r as usize].0 = *x1.min(&min_max_x[r as usize].0);
                    min_max_x[r as usize].1 = *x1.max(&min_max_x[r as usize].1);
                }
            }
        }
    }
    let mut max_rectangle = 0;
    for (idx1, (x1, y1)) in vec.iter().enumerate() {
        for (x2, y2) in vec.iter().skip(idx1 + 1) {
            let rectangle_size = ((y2 - y1).abs() + 1) * ((x2 - x1).abs() + 1);
            if rectangle_size > max_rectangle {
                let corner_left_up = (*x1.min(x2), *y1.min(y2));
                let corner_left_down = (*x1.min(x2), *y1.max(y2));
                let _corner_right_down = (*x1.max(x2), *y1.max(y2));
                let corner_right_up = (*x1.max(x2), *y1.min(y2));
                let mut possible = true;
                for x in corner_left_up.0..=corner_right_up.0 {
                    if corner_left_up.1 > min_max_y[x as usize].1
                        || corner_left_down.1 < min_max_y[x as usize].0
                    {
                        possible = false;
                        break;
                    }
                }
                if !possible {
                    continue;
                }
                for y in corner_left_up.1..=corner_left_down.1 {
                    if corner_right_up.0 > min_max_x[y as usize].1
                        || corner_left_up.0 < min_max_x[y as usize].0
                    {
                        possible = false;
                        break;
                    }
                }
                if possible {
                    max_rectangle = rectangle_size;
                }
            }
        }
    }
    // println!("{:?}", vec);
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
