use std::collections::HashMap;

use crate::advent_util::read_lines;

fn gen_to_check(x: usize, y: usize, x_limit: usize, y_limit: usize) -> Vec<(usize, usize)> {
    let mut to_check: Vec<(usize, usize)> = Vec::with_capacity(8);
    if x > 0 {
        to_check.push((x - 1, y));
        if y < y_limit {
            to_check.push((x - 1, y + 1));
        }
        if y > 0 {
            to_check.push((x - 1, y - 1));
        }
    }
    if x < x_limit {
        to_check.push((x + 1, y));
        if y < y_limit {
            to_check.push((x + 1, y + 1));
        }
        if y > 0 {
            to_check.push((x + 1, y - 1));
        }
    }
    if y < y_limit {
        to_check.push((x, y + 1));
    }
    if y > 0 {
        to_check.push((x, y - 1));
    }

    return to_check;
}

fn part1(schematic: &Vec<Vec<char>>) -> i64 {
    let mut number: Vec<char> = Vec::new();
    let mut number_adjacent = false;
    let mut sum: i64 = 0;

    let y_limit = schematic.len() - 1;
    let x_limit = schematic.get(0).unwrap().len() - 1;

    for (y, xs) in schematic.iter().enumerate() {
        for (x, value) in xs.iter().enumerate() {
            match value {
                '0'..='9' => {
                    number.push(*value);
                    let to_check = gen_to_check(x, y, x_limit, y_limit);
                    for (cx, cy) in to_check {
                        let char = schematic.get(cy).unwrap().get(cx).unwrap();
                        if !(char.is_digit(10) || *char == '.') {
                            number_adjacent = true;
                        }
                    }
                }
                _ => {
                    if !number.is_empty() {
                        if number_adjacent {
                            let parsed_num: i64 = String::from_iter(number).parse().unwrap();
                            sum += parsed_num;
                        }
                        number = Vec::new();
                        number_adjacent = false;
                    }
                }
            }
        }
    }
    return sum;
}

fn part2(schematic: &Vec<Vec<char>>) -> u64 {
    let mut gear_candidates: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    let mut number: Vec<char> = Vec::new();
    let mut number_gear: Option<(usize, usize)> = None;

    let y_limit = schematic.len() - 1;
    let x_limit = schematic.get(0).unwrap().len() - 1;

    for (y, xs) in schematic.iter().enumerate() {
        for (x, value) in xs.iter().enumerate() {
            match value {
                '0'..='9' => {
                    number.push(*value);
                    let to_check = gen_to_check(x, y, x_limit, y_limit);
                    for (cx, cy) in to_check {
                        let char = schematic.get(cy).unwrap().get(cx).unwrap();
                        if *char == '*' {
                            number_gear = Some((cx, cy));
                        }
                    }
                }
                _ => {
                    if !number.is_empty() {
                        if let Some(gear) = number_gear {
                            let parsed_num: u64 = String::from_iter(number).parse().unwrap();
                            if gear_candidates.contains_key(&gear) {
                                gear_candidates.get_mut(&gear).unwrap().push(parsed_num);
                            } else {
                                gear_candidates.insert(gear, vec![parsed_num]);
                            }
                        }
                        number = Vec::new();
                        number_gear = None;
                    }
                }
            }
        }
    }

    return gear_candidates
        .values()
        .map(|vs| {
            if vs.len() == 2 {
                return vs.iter().product();
            } else {
                return 0;
            }
        })
        .sum();
}

pub fn run() {
    let schematic: Vec<Vec<char>> = read_lines("./inputs/day03")
        .unwrap()
        .map(|lr| lr.unwrap().chars().collect::<Vec<char>>())
        .collect();

    println!("Part 1: {}", part1(&schematic));
    println!("Part 2: {}", part2(&schematic));
}
