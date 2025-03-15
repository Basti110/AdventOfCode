use std::fs::File;
use std::io::{self, BufRead};

fn check_order(row_entries: &Vec<i32>) -> bool {
    let mut order: i32 = 0;
    for i in 1..row_entries.len() {
        let diff = (row_entries[i - 1] - row_entries[i]).abs();
        if diff == 0 || diff > 3 {
            return false;
        }

        if row_entries[i - 1] < row_entries[i] {
            if order < 0 {
                return false;
            }
            order = 1
        }

        if row_entries[i - 1] > row_entries[i] {
            if order > 0 {
                return false;
            }
            order = -1
        }
    }

    return true;
}

fn main() -> io::Result<()> {
    let file: File = File::open("./input/day2a.txt")?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut save_count: i32 = 0;
    for line in reader.lines() {
        let line_str: String = line?;
        let mut row_entries: Vec<i32> = line_str
            .split_whitespace()
            .map(|a: &str| a.parse::<i32>().unwrap())
            .collect();

        if check_order(&mut row_entries) {
            save_count += 1;
        }
    }

    println!("{}", save_count);

    Ok(())
}
