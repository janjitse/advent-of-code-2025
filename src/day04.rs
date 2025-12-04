use crate::parsers::*;


#[aoc(day4, part1)]
pub fn part_a(contents: &str) -> i32 {
    let mut accessible = 0;
    let vec = parse_array_of_chars(contents);
    let directions: Vec<(usize, usize)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (usize::MAX, 0),
        (usize::MAX, 1),
        (usize::MAX, usize::MAX),
        (1, usize::MAX),
        (0, usize::MAX),
    ];
    for (row_idx, line) in vec.iter().enumerate() {
        for (col_idx, val) in line.iter().enumerate() {
            if *val == '@' {
                let mut sur_papers = 0;
                for dir in directions.iter() {
                    let pos_x = col_idx.wrapping_add(dir.1);
                    let pos_y = row_idx.wrapping_add(dir.0);
                    if pos_x >= line.len() || pos_y >= vec.len() {
                        continue;
                    }
                    let test_c = vec[pos_y][pos_x];
                    if test_c == '@' {
                        sur_papers +=1;
                    }

                }
                if sur_papers < 4 {
                    accessible += 1;
                }
            }
            
            

        }       
    }
    accessible
    
}

#[aoc(day4, part2)]
pub fn part_b(contents: &str) -> i32 {
    let mut accessible = 0;
    let mut vec = parse_array_of_chars(contents);
    let directions: Vec<(usize, usize)> = vec![
        (0, 1),
        (1, 0),
        (1, 1),
        (usize::MAX, 0),
        (usize::MAX, 1),
        (usize::MAX, usize::MAX),
        (1, usize::MAX),
        (0, usize::MAX),
    ];
    let mut changes = true;
    while changes {
        changes = false;
        let mut new_vec = vec.clone();
        for (row_idx, line) in vec.iter().enumerate() {
            for (col_idx, val) in line.iter().enumerate() {
                if *val == '@' {
                    let mut sur_papers = 0;
                    for dir in directions.iter() {
                        let pos_x = col_idx.wrapping_add(dir.1);
                        let pos_y = row_idx.wrapping_add(dir.0);
                        if pos_x >= line.len() || pos_y >= vec.len() {
                            continue;
                        }
                        let test_c = vec[pos_y][pos_x];
                        if test_c == '@' {
                            sur_papers +=1;
                        }

                    }
                    if sur_papers < 4 {
                        accessible += 1;
                        new_vec[row_idx][col_idx] = '.';
                        changes = true;
                    }
                }

                
                

            }
        }
        vec = new_vec;
    }

    // vec.into_iter().sum()
    accessible
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
        assert_eq!(part_a(&contents), 13);
    }

    #[test]
    fn test_2() {
        let s = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
        let file_path = format!("input/2025/{}_small.txt", s);
        let contents = fs::read_to_string(file_path).expect("file not found");
        assert_eq!(part_b(&contents), 43);
    }
}