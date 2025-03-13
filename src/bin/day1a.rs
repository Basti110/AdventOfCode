use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let file = File::open("./input/day1a.txt")?;
    let reader = io::BufReader::new(file);

    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    for line in reader.lines() {
        let line_str = line?;
        let mut row_entries = line_str.split_whitespace();

        if let (Some(first), Some(second)) = (row_entries.next(), row_entries.next()) {
            first_column.push(first.parse::<i32>().unwrap());
            second_column.push(second.parse::<i32>().unwrap());
        }
    }
    first_column.sort();
    second_column.sort();
    // println!("{:?}", first_column);
    // println!("{:?}", second_column);

    let diff: i32 = first_column
        .iter()
        .zip(second_column.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("{}", diff);

    Ok(())
}
