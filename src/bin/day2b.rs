use std::fs::File;
use std::io::{self, BufRead};

// 3 4 5
// 3 2 4 5
// 3 2 1
// 3 2 3 5
// 3 2 4 1

fn check_order(row_entries: &Vec<i32>) -> (bool, usize) {
    let mut order: i32 = 0;
    for i in 1..row_entries.len() {
        let diff: i32 = (row_entries[i - 1] - row_entries[i]).abs();
        if diff == 0 || diff > 3 {
            return (false, i);
        }

        if row_entries[i - 1] < row_entries[i] {
            if order < 0 {
                return (false, i);
            }
            order = 1
        }

        if row_entries[i - 1] > row_entries[i] {
            if order > 0 {
                return (false, i);
            }
            order = -1
        }
    }

    return (true, 0);
}

fn main() -> io::Result<()> {
    let file: File = File::open("./input/day2a.txt")?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut save_count: i32 = 0;
    for line in reader.lines() {
        let line_str: String = line?;
        let row_entries: Vec<i32> = line_str
            .split_whitespace()
            .map(|a: &str| a.parse::<i32>().unwrap())
            .collect();

        let (check_success, fail_index) = check_order(&row_entries);
        if check_success {
            save_count += 1;
            continue;
        }

        for i in 0..fail_index + 1 {
            let mut new_entries = row_entries.clone();
            new_entries.remove(i);
            if check_order(&new_entries).0 {
                save_count += 1;
                break;
            }
        }
    }

    println!("{}", save_count);
    Ok(())
}
