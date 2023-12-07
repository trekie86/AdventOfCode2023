use std::fs::read_to_string;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn split_game_values(game_str: &String) -> (i32, HashSet<i32>, HashSet<i32>) {
    let mut winning_numbers: HashSet<i32> = HashSet::new();
    let mut chosen_numbers: HashSet<i32> = HashSet::new();
    let re = Regex::new(r"(\d+)").unwrap();
    let caps = re.captures(&game_str).unwrap();
    let game_num = &caps[0].parse::<i32>().unwrap();
    let parts: Vec<&str> = game_str[game_str.find(':').unwrap()+1..].split("|").collect();

    for num in parts[0].trim().split(" ") {
        let result = num.trim().parse::<i32>();
        if result.is_ok() {
            winning_numbers.insert(result.unwrap());
        }
    }
    for num in parts[1].trim().split(" ") {
        let result = num.trim().parse::<i32>();
        if result.is_ok() {
            chosen_numbers.insert(result.unwrap());
        }
    }

    return (*game_num, winning_numbers, chosen_numbers);
}

fn part_1() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("input.txt");
    let mut total: i32 = 0;
    for game in input {
        let (_game_num, winning_numberes, chosen_numbers) = split_game_values(&game);
        let intersection = winning_numberes.intersection(&chosen_numbers);
        let intersection_count = &intersection.count();
        if intersection_count > &(0 as usize) {
            total += 2i32.pow((intersection_count - 1) as u32);
        }
    }
    println!("{}", total);
    let duration = start.elapsed();
    println!("Time elapsed in part_1() is {:?}", duration);
}

fn part_2() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("input.txt");
    let mut winning_games: HashMap<i32, i32> = HashMap::new();
    for game in &input {
        let (game_num, winning_numberes, chosen_numbers) = split_game_values(&game);
        let intersection = winning_numberes.intersection(&chosen_numbers);
        winning_games.insert(game_num, intersection.count() as i32);
    }
    let mut results: HashMap<i32, i32> = HashMap::new();
    for game_num in 1..=input.len() {
        results.insert(game_num as i32, 1 as i32);
    }
    for i in 1..=input.len() {
        let wins = winning_games.get(&(i as i32)).unwrap();
        for j in 1..=*wins {
            results.insert(i as i32 + j as i32, results[&(i as i32 +j as i32)] + results[&(i as i32)]);
        }
    }
    let total: i32 = results.values().sum();
    println!("{}", total);
    let duration = start.elapsed();
    println!("Time elapsed in part_2() is {:?}", duration);
}

fn main() {
    println!("Results for part_1: ");
    part_1();
    println!("Results for part_2: ");
    part_2();
}
