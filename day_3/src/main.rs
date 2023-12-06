use itertools::Itertools;
use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug)]
struct EnginePart {
    row_idx: i32,
    start_col: i32,
    end_col: i32,
    val: i32,
}

fn define_engine_part(row_idx: i32, start_col: i32, end_col: i32, val: i32) -> EnginePart {
    EnginePart {
        row_idx,
        start_col,
        end_col,
        val,
    }
}

fn build_grid(input: &Vec<String>) -> Vec<Vec<String>> {
    let height = input.len();
    let width = input[0].len();
    let mut grid = vec![vec!["".to_string(); width]; height];
    let mut row_idx = 0;
    for row in input {
        for (i, cell) in row.chars().enumerate() {
            if cell != '.' {
                grid[row_idx][i] = cell.to_string();
            }
        }
        row_idx += 1;
    }
    grid
}

fn build_parts_list(schematic: &Vec<Vec<String>>) -> Vec<EnginePart> {
    let mut parts_list: Vec<EnginePart> = Vec::new();
    for (i, row) in schematic.iter().enumerate() {
        let mut start_pos: i32 = -1;
        let mut end_pos: i32 = -1;
        for (j, cell) in row.iter().enumerate() {
            if cell.chars().next().map(char::is_numeric).unwrap_or(false) {
                //This is a number, so we need to set or adjust position info
                if start_pos == -1 {
                    start_pos = j as i32;
                    end_pos = j as i32;
                } else {
                    end_pos = j as i32;
                }

                //Edge case, handle end of row
                if j as i32 == (row.len() -1) as i32 {
                    let val = schematic[i][start_pos as usize..end_pos as usize + 1]
                        .iter()
                        .join("")
                        .parse::<i32>()
                        .unwrap();
                    parts_list.push(define_engine_part(i as i32, start_pos, end_pos, val));
                }
            } else {
                if start_pos != -1 {
                    let val = schematic[i][start_pos as usize..end_pos as usize + 1]
                        .iter()
                        .join("")
                        .parse::<i32>()
                        .unwrap();
                    parts_list.push(define_engine_part(i as i32, start_pos, end_pos, val));
                    start_pos = -1;
                    end_pos = -1;
                }
            }
        }
    }
    parts_list
}

fn count_part(part: &EnginePart, schematic: &Vec<Vec<String>>) -> bool {
    let row_idx = part.row_idx as usize;
    let start_row = if row_idx == 0 { 0 } else { row_idx - 1 };
    let end_row = if row_idx == (schematic.len() - 1) {
        row_idx
    } else {
        row_idx + 1
    };
    let start_col = if part.start_col == 0 {
        0
    } else {
        (part.start_col - 1) as usize
    };
    let end_col = if part.end_col as usize == (schematic[row_idx].len() - 1) {
        part.end_col as usize
    } else {
        (part.end_col + 1) as usize
    };
    for i in start_row..end_row + 1 {
        for j in start_col..end_col + 1 {
            let _compare_val = &schematic[i][j];
            if !schematic[i][j].is_empty()
                && !schematic[i][j]
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
            {
                return true;
            }
        }
    }
    return false;
}

fn get_stars(schematic: &Vec<Vec<String>>) -> Vec<(i32, i32)> {
    let mut stars: Vec<(i32, i32)> = Vec::new();
    for (i, rows) in schematic.iter().enumerate() {
        for (j, cell) in rows.iter().enumerate() {
            if cell == "*" {
                stars.push((i as i32, j as i32));
            }
        }
    }

    return stars;
}

fn get_neighbor_gears(star: (i32, i32), parts: &Vec<EnginePart>) -> HashSet<&EnginePart> {
    let mut matching_parts: HashSet<&EnginePart> = HashSet::new();
    let (row_idx, col_idx) = star;
    for part in parts {
        //First confirm if the range is even close
        if row_idx - 1 <= part.row_idx && part.row_idx <= row_idx + 1 {
            //Check perminter for the star to see if the part intersects
            for i in row_idx - 1..=row_idx + 1 {
                for j in col_idx - 1..=col_idx + 1 {
                    if i == part.row_idx && (part.start_col <= j && j <= part.end_col) {
                        matching_parts.insert(part);
                    }
                }
            }
        }
    }
    return matching_parts;
}

fn part_1() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("input.txt");
    let schematic = build_grid(&input);
    let parts = build_parts_list(&schematic);
    let mut total: i32 = 0;
    for part in parts {
        if count_part(&part, &schematic) == true {
            total += &part.val;
        }
    }
    println!("{}", total);
    let duration = start.elapsed();

    println!("Time elapsed in part_1() is: {:?}", duration);
}

fn part_2() {
    let start = Instant::now();
    let input: Vec<String> = read_lines("input.txt");
    let schematic = build_grid(&input);
    let parts = build_parts_list(&schematic);
    let stars: Vec<(i32, i32)> = get_stars(&schematic);
    let mut total: i32 = 0;
    for star in stars {
        let matches = get_neighbor_gears(star, &parts);
        if matches.len() >= 2 {
            let prod: i32 = matches.iter().map(|m| m.val).product();
            total += prod;
        }
    }
    println!("{}", total);
    let duration = start.elapsed();

    println!("Time elapsed in part_2() is: {:?}", duration);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
fn main() {
    println!("Result of part 1: ");
    part_1();
    println!("Result of part 2: ");
    part_2();
}
