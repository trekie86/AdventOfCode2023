use regex::Regex;
use std::fs::read_to_string;
use std::time::Instant;

struct Transition {
    name: String,
    rules: Vec<Rule>,
}

struct Rule {
    dest_start: u64,
    source_start: u64,
    source_end: u64,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn process_rules(input: u64, rules: &Vec<Rule>) -> u64 {
    let mut result = input;
    for rule in rules {
        if input >= rule.source_start && input <= rule.source_end {
            result = rule.dest_start + input - rule.source_start;
            return result;
        }
    }
    return result;
}

fn process(input: &Vec<String>, re: Regex, seeds: Vec<u64>) {
    let mut current: Vec<u64> = seeds;
    let mut transitions: Vec<Transition> = Vec::new();
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut inner: Vec<String> = Vec::new();
    for line in input {
        if !line.trim().is_empty() {
            inner.push(String::from(line.trim()));
        } else {
            chunks.push(inner);
            inner = Vec::new();
        }
    }
    //Get the last one
    chunks.push(inner);
    //Build the actual rules
    for chunk in chunks {
        let name = &chunk[0];
        let mut rules: Vec<Rule> = Vec::new();
        for line in &chunk[1..] {
            let vals: Vec<u64> = re.find_iter(line).map(|x| x.as_str().parse::<u64>().unwrap()).collect();
            let delta = vals[2];
            rules.push(Rule {dest_start: vals[0], source_start: vals[1], source_end: vals[1]+(delta-1)});
        }
        transitions.push(Transition {name: name.to_string(), rules: rules});
    }
    let mut next: Vec<u64> = Vec::new();
    for transition in transitions {
        println!("Transition: {}", transition.name);
        for i in current {
            next.push(process_rules(i, &transition.rules));
        }
        current = next;
        next = Vec::new();
    }
    println!("{}", current.iter().min().unwrap());
}

fn part_1() {
    let start = Instant::now();
    let input = read_lines("input.txt");
    let re = Regex::new(r"(\d+)").unwrap();
    let seeds: Vec<u64> = re
        .find_iter(&input[0])
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect();
    process(&input, re, seeds);
    let duration = start.elapsed();
    println!("Time elapsed in part_1() is {:?}", duration);
}

fn part_2() {
    let start = Instant::now();
    let input = read_lines("input.txt");
    let re = Regex::new(r"(\d+)").unwrap();
    let interum: Vec<u64> = re
        .find_iter(&input[0])
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect();
    let mut seeds: Vec<u64> = Vec::new();
    for i in (0..interum.len()).into_iter().step_by(2) {
        for j in 0..interum[i+1] {
            seeds.push(interum[i] as u64 + j);
        }
    }
    process(&input, re, seeds);
    let duration = start.elapsed();
    println!("Time elapsed in part_2() is {:?}", duration);
}

fn main() {
    println!("Results for part_1: ");
    part_1();
    println!("Results for part_2: ");
    part_2();
}
