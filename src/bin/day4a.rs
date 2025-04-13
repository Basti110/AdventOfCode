use std::char::from_u32_unchecked;
use std::fs::File;
use std::io::{self, BufRead};

fn check_in_direction(
    christmas_matrix: &Vec<Vec<char>>,
    match_string: &Vec<char>,
    index: (usize, usize),
    max_index: (usize, usize),
    direction: (i32, i32),
) -> bool {
    let mut i: i32 = index.0 as i32;
    let mut j: i32 = index.1 as i32;

    for character in match_string.iter() {
        if i < 0 || i > max_index.0 as i32 || j < 0 || j > max_index.1 as i32 {
            return false;
        }
        if christmas_matrix[i as usize][j as usize] != *character {
            return false;
        }
        i += direction.0;
        j += direction.1;
    }
    true
}

fn check_search_vectors(
    christmas_matrix: &Vec<Vec<char>>,
    match_string: &Vec<char>,
    index: (usize, usize),
    max_index: (usize, usize),
) -> usize {
    let directions: Vec<(i32, i32)> = vec![
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut found_count: usize = 0;
    for direction in directions {
        let check: bool =
            check_in_direction(christmas_matrix, match_string, index, max_index, direction);
        if check {
            found_count += 1;
        }
    }
    found_count
}

fn main() -> io::Result<()> {
    let file: File = File::open("./input/day4a.txt")?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let match_string: Vec<char> = vec!['X', 'M', 'A', 'S'];

    let mut christmas_matrix: Vec<Vec<char>> = Vec::new();
    let mut array_size: usize = 0;

    for line in reader.lines() {
        let line_chars: Vec<char> = line.unwrap().chars().collect();
        let length: usize = line_chars.len();
        if array_size == 0 {
            array_size = length;
        }
        if array_size != length {
            println!("Error: Array size is not consistent");
            return Ok(());
        }
        christmas_matrix.push(line_chars);
    }
    let mut found_count: usize = 0;
    let line_size: usize = christmas_matrix.len();
    for i in 0..line_size {
        for j in 0..array_size {
            if christmas_matrix[i][j] == match_string[0] {
                found_count += check_search_vectors(
                    &christmas_matrix,
                    &match_string,
                    (i, j),
                    (line_size - 1, array_size - 1),
                );
            }
        }
    }
    print!("{}", found_count);
    // println!("{:?}", christmas_matrix);
    Ok(())
}
