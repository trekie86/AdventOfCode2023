use regex::Regex;
use std::fs::read_to_string;

fn part1() {
    let input: Vec<String> = read_lines("input.txt");
    let mut results: Vec<i32> = Vec::new();
    for row in input {
        let mut row_vals: Vec<char> = Vec::new();
        for c in row.chars() {
            if c.to_digit(10).is_some() {
                row_vals.push(c);
            }
        }
        let mut row_string: String = String::new();
        row_string.push(*row_vals.get(0).unwrap());
        row_string.push(*row_vals.get(row_vals.len() - 1).unwrap());
        results.push(row_string.parse::<i32>().unwrap());
    }
    let result: i32 = results.iter().sum();
    println!("{}", result);
}

fn part2() {
    let input: Vec<String> = read_lines("input.txt");
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let mut results: Vec<i32> = Vec::new();
    for row in &input {
        let mut row_vals: Vec<String> = Vec::new();
        // dbg!(&row);
        for n in 0..row.len() {
            let matched_val = re.find(&row[n..]);
            if matched_val.is_some() {
                let val = matched_val.unwrap().as_str();
                if val.parse::<i32>().is_ok() {
                    row_vals.push(val.to_string())
                } else {
                    match val {
                        "one" => row_vals.push("1".to_string()),
                        "two" => row_vals.push("2".to_string()),
                        "three" => row_vals.push("3".to_string()),
                        "four" => row_vals.push("4".to_string()),
                        "five" => row_vals.push("5".to_string()),
                        "six" => row_vals.push("6".to_string()),
                        "seven" => row_vals.push("7".to_string()),
                        "eight" => row_vals.push("8".to_string()),
                        "nine" => row_vals.push("9".to_string()),
                        _ => println!("Weird input detected in {}", val),
                    }
                }
            }
        }
        // dbg!(&row_vals);
        let mut row_string: String = String::from(row_vals.get(0).unwrap());
        row_string.push_str(row_vals.get(row_vals.len() - 1).unwrap());
        // dbg!(&row_string);
        let row_val: i32 = row_string.parse::<i32>().unwrap();
        // dbg!(&row_val);
        results.push(row_val);
    }
    // dbg!(&results);
    let result: i32 = results.iter().sum();
    // let mut result: i64 = 0;
    // for i in results {
    //     result += i64::from(i)
    // }
    println!("{}", result);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    println!("Part 1 Result:");
    part1();
    println!("Part 2 Result:");
    part2();
}
