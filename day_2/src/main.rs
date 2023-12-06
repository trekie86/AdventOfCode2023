use std::fs::read_to_string;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn part_1() {
    let start = Instant::now();
    let mut rules: HashMap<&str, i32> = HashMap::new();
    rules.insert("red", 12);
    rules.insert("blue", 14);
    rules.insert("green", 13);

    let mut plausible_games: Vec<i32> = Vec::new();
    let games: Vec<String> = read_lines("input.txt");
    for raw_game in games {
        let game_str = raw_game.split(':').next().unwrap();
        let game_num: i32 = game_str.split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        // dbg!(game_num);
        let raw_rounds = raw_game.split(':').nth(1).unwrap();
        let rounds: Vec<&str> = raw_rounds.split(";").collect();
        // dbg!(&rounds);
        let mut valid = true;
        for round in rounds {
            for cube in round.trim().split(",") {
                // dbg!(&cube);
                let val = cube.trim().split(" ").next().unwrap().parse::<i32>().unwrap();
                let color = cube.trim().split(" ").nth(1).unwrap();
                if rules[color] < val {
                    valid = false;
                }
            }
        }
        if valid {
            plausible_games.push(game_num);
        }

    }

    let result: i32 = plausible_games.iter().sum();
    println!("{}", result);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);

}

fn part_2() {
    let start = Instant::now();
    let mut min_power: Vec<i32> = Vec::new();
    let games: Vec<String> = read_lines("input.txt");
    for raw_game in games {
        // dbg!(game_num);
        let raw_rounds = raw_game.split(':').nth(1).unwrap();
        let rounds: Vec<&str> = raw_rounds.split(";").collect();
        let mut min_needs: HashMap<&str, i32> = HashMap::new();
        for round in rounds {
            for cube in round.trim().split(",") {
                let val = cube.trim().split(" ").next().unwrap().parse::<i32>().unwrap();
                let color = cube.trim().split(" ").nth(1).unwrap();
                if !min_needs.contains_key(color) {
                    min_needs.insert(color, val);
                } else if min_needs[color] < val {
                    min_needs.insert(color, val);
                }
            }
        }
        min_power.push(min_needs.values().product());
    }
    let result: i32 = min_power.iter().sum();
    println!("{}", result);
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    println!("Part 1 result:");
    part_1();
    println!("Part 2 result:");
    part_2();
}
