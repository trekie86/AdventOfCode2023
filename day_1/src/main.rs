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
    let mut result: i32 = 0;
    for i in results {
        result += i
    }
    println!("{}", result);
}

fn part2() {

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
