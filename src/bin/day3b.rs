use std::fs::read_to_string;

fn parse_parent(word: &str) -> Option<Vec<i32>> {
    let word = word.trim_start();
    if !word.starts_with('(') {
        return None;
    }

    let mut values: Vec<i32> = Vec::new();
    let mut current: String = String::new();
    let mut chars = word.chars().skip(1);

    while let Some(c) = chars.next() {
        match c {
            ')' => {
                if !current.is_empty() {
                    values.push(current.parse::<i32>().ok()?);
                }
                return Some(values);
            }
            ',' => {
                if !current.is_empty() {
                    values.push(current.parse::<i32>().ok()?);
                    current.clear();
                } else {
                    return None;
                }
            }
            d if d.is_ascii_digit() => {
                current.push(d);
            }
            _ => return None, // Then its a invalid Character
        }
    }

    None
}

fn main() {
    let input: String = read_to_string("./input/day3a.txt").expect("Failed to read file");

    let expressions = ["do", "don't", "mul"];
    let mut total_value = 0;
    let mut enabled = true;
    let input_chars: Vec<char> = input.chars().collect();

    let mut i = 0;
    while i < input_chars.len() {
        for expr in &expressions {
            let expr_len = expr.len();
            if i + expr_len > input_chars.len() {
                continue;
            }

            let slice: String = input_chars[i..i + expr_len].iter().collect();
            if slice != *expr {
                continue;
            }

            let after_expr: String = input_chars[i + expr_len..].iter().collect();
            if let Some(values) = parse_parent(&after_expr) {
                // println!("{} {:?}", expr, values);

                // remember this nice match tuple match
                match (*expr, values.len()) {
                    ("do", 0) => enabled = true,
                    ("don't", 0) => enabled = false,
                    ("mul", 2) if enabled => {
                        total_value += values[0] * values[1];
                    }
                    _ => {
                        println!("INVALID: {}, {:?}", *expr, values);
                    }
                }
                println!("Enabled: {}, value: {}", enabled, total_value);
            }

            i += expr_len;
            break;
        }
        i += 1;
    }

    println!("Total value: {}", total_value);
}
