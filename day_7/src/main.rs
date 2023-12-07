use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<String>,
    bet: u64,
}

impl Hand {
    fn rank(&self) -> u64 {
        let mut letters: HashMap<String, u8> = HashMap::new();
        for c in &self.cards {
            *letters.entry(c.to_string()).or_insert(0) += 1;
        }

        //Five of a kind
        if letters.len() == 1 as usize {
            return 6;
            //Four of a kind or full house
        } else if letters.len() == 2 {
            if letters.values().any(|&x| x == 4) {
                return 5;
            } else {
                return 4;
            }
            //Three of a kind or 2 pair
        } else if letters.len() == 3 {
            if letters.values().any(|&x| x == 3) {
                return 3;
            } else {
                return 2;
            }
        } else if letters.len() == 4 {
            return 1;
        } else {
            //High card
            return 0;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if &self.rank() == &other.rank() {
            for i in 0..self.cards.len() {
                if !(card_order_value(&self.cards[i]) == card_order_value(&other.cards[i])) {
                    return card_order_value(&self.cards[i])
                        .cmp(&card_order_value(&other.cards[i]));
                }
            }
            return Ordering::Less;
        } else {
            return self.rank().cmp(&other.rank());
        }
    }
}

//I don't like this duplication but I don't know how to change the comparator
#[derive(Debug, PartialEq, Eq)]
struct JokerHand {
    cards: Vec<String>,
    bet: u64,
}

impl JokerHand {
    fn rank(&self) -> u64 {
        //five of a kind = 6
        //four of a kind = 5
        //full house = 4
        //three of a kind = 3
        //two pair = 2
        //one pair = 1
        //High card = 0
        let contains_joker = self.cards.contains(&"J".to_string());
        if contains_joker {
            let mut new_cards = self.cards.clone();
            new_cards.retain(|x| *x != "J");
            let mut letters: HashMap<String, u8> = HashMap::new();
            for c in &new_cards {
                *letters.entry(c.to_string()).or_insert(0) += 1;
            }
            let joker_count = 5 - new_cards.len();
            if joker_count >= 4 || letters.len() == 1 {
                return 6;
            } else if joker_count == 3 {
                //Four of a kind
                return 5;
            } else if joker_count == 2 {
                match letters.len() {
                    2 => return 5,
                    _ => return 3,
                }
            } else {
                //One joker
                if letters.values().any(|&x| x == 3) {
                    return 5;
                } else if letters.len() == 2 {
                    return 4;  
                } else if letters.len() == 3 {
                    return 3;
                } else {
                    return 1;
                }
            }
        } else {
            let mut letters: HashMap<String, u8> = HashMap::new();
            for c in &self.cards {
                *letters.entry(c.to_string()).or_insert(0) += 1;
            }

            //Five of a kind
            if letters.len() == 1 as usize {
                return 6;
                //Four of a kind or full house
            } else if letters.len() == 2 {
                if letters.values().any(|&x| x == 4) {
                    return 5;
                } else {
                    return 4;
                }
                //Three of a kind or 2 pair
            } else if letters.len() == 3 {
                if letters.values().any(|&x| x == 3) {
                    return 3;
                } else {
                    return 2;
                }
            } else if letters.len() == 4 {
                return 1;
            } else {
                //High card
                return 0;
            }
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if &self.rank() == &other.rank() {
            for i in 0..self.cards.len() {
                if !(card_order_value_with_joker(&self.cards[i])
                    == card_order_value_with_joker(&other.cards[i]))
                {
                    return card_order_value_with_joker(&self.cards[i])
                        .cmp(&card_order_value_with_joker(&other.cards[i]));
                }
            }
            return Ordering::Less;
        } else {
            return self.rank().cmp(&other.rank());
        }
    }
}

fn card_order_value(card: &String) -> u8 {
    let val: &str = &card;
    match val {
        "A" => return 14,
        "K" => return 13,
        "Q" => return 12,
        "J" => return 11,
        "T" => return 10,
        _ => return card.parse::<u8>().unwrap(),
    }
}

fn card_order_value_with_joker(card: &String) -> u8 {
    let val: &str = &card;
    match val {
        "A" => return 14,
        "K" => return 13,
        "Q" => return 12,
        "J" => return 1,
        "T" => return 10,
        _ => return card.parse::<u8>().unwrap(),
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn part_1() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("input.txt");
    let mut hands: Vec<Hand> = Vec::new();
    for line in input {
        let mut parts = line.trim().split(" ");
        let cards: Vec<String> = parts
            .next()
            .unwrap()
            .chars()
            .into_iter()
            .map(|c| c.to_string())
            .collect();
        let bet: u64 = parts.next().unwrap().parse().unwrap();
        hands.push(Hand { cards, bet });
    }
    hands.sort();
    let mut total = 0;
    for (i, hands) in hands.iter().enumerate() {
        total += &hands.bet * (i as u64 + 1);
    }
    println!("{}", total);
    let duration = start.elapsed();
    println!("Time elapsed in part_1() is: {:?}", duration);
}

fn part_2() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("input.txt");
    let mut hands: Vec<JokerHand> = Vec::new();
    for line in input {
        let mut parts = line.trim().split(" ");
        let cards: Vec<String> = parts
            .next()
            .unwrap()
            .chars()
            .into_iter()
            .map(|c| c.to_string())
            .collect();
        let bet: u64 = parts.next().unwrap().parse().unwrap();
        hands.push(JokerHand { cards, bet });
    }
    hands.sort();
    let mut total = 0;
    for (i, hands) in hands.iter().enumerate() {
        total += &hands.bet * (i as u64 + 1);
    }
    println!("{}", total);
    let duration = start.elapsed();
    println!("Time elapsed in part_2() is: {:?}", duration);
}

fn main() {
    println!("Result for part_1() is ");
    part_1();
    println!("Result for part_2() is ");
    part_2();
}
