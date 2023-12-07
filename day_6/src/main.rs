use regex::Regex;
use std::time::Instant;
use std::fs::read_to_string;


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn part_1() {
    let start = Instant::now();
    let input = read_lines("input.txt");
    let re = Regex::new(r"(\d+)").unwrap();
    let times: Vec<u32> = re.find_iter(&input[0]).map(|x| x.as_str().parse::<u32>().unwrap()).collect();
    let distances: Vec<u32> = re.find_iter(&input[1]).map(|x| x.as_str().parse::<u32>().unwrap()).collect();
    let mut winning_options: Vec<i32> = Vec::new();
    for i in 0..times.len() {
        let mut win_count: i32 = 0;
        for j in 0..=times[i] {
            if j * (times[i] - j) > distances[i] {
                win_count += 1;
            }
        }
        winning_options.push(win_count);
    }
    let mut total: i32 = 1;
    for i in &winning_options {
        total *= i;
    }
    println!("{:?}", &total);
    let duration = start.elapsed();
    println!("Time elapsed in part_1() is {:?}", duration);
}

fn part_2() {
    let start = Instant::now();
    let input = read_lines("input.txt");
    let re = Regex::new(r"(\d+)").unwrap();
    let times_raw: String = input[0].replace(" ", "");
    let distances_raw: String = input[1].replace(" ", "");
    let time: u64 = re.captures(&times_raw).unwrap()[0].parse().unwrap();
    let distance: u64 = re.captures(&distances_raw).unwrap()[0].parse().unwrap();
    let mut count: u64 = 0;
    for i in 0..=time {
        if i * (time - i) > distance {
            count += 1;
        }
    }
    println!("{}", count);
    let duration = start.elapsed();
    println!("Time elapsed in part_2() is {:?}", duration);
}

fn main() {
    println!("Result for part_1 ");
    part_1();
    println!("Result for part_2 ");
    part_2();
}
