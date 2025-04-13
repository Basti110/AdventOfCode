use regex::Regex;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("./input/day3a.txt").unwrap();
    let re: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let results: i32 = re
        .captures_iter(&input)
        .filter_map(|cap| {
            let x = cap.get(1)?.as_str().parse::<i32>().ok()?;
            let y = cap.get(2)?.as_str().parse::<i32>().ok()?;
            Some(x * y)
        })
        .sum();

    println!("{:?}", results); // Outpu<t: [(1, 3), (4, 5)]
}
