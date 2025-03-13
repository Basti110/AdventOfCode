use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Same input as in 1a
    let file: File = File::open("./input/day1a.txt")?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line_str = line?;
        let mut row_entries = line_str.split_whitespace();

        if let (Some(first), Some(second)) = (row_entries.next(), row_entries.next()) {
            first_column.push(first.parse::<i32>().unwrap());
            second_column.push(second.parse::<i32>().unwrap());
        }
    }

    let mut second_column_value_count: HashMap<i32, i32> = HashMap::new();
    for &value in second_column.iter() {
        *second_column_value_count.entry(value).or_insert(0) += 1;
    }

    // println!("{:?}", second_column_value_count);

    // println!("{:?}", first_column);
    // println!("{:?}", second_column);

    let score: i32 = first_column
        .iter()
        .map(|a: &i32| (a * second_column_value_count.get(a).unwrap_or(&0)).abs())
        .sum();

    println!("{}", score);

    Ok(())
}
