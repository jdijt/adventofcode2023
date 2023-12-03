use std::collections::HashMap;

use crate::advent_util::read_lines;

struct Schematic {
    raw: Vec<Vec<char>>,
    x_limit: usize,
    y_limit: usize,
}

impl Schematic {
    fn from(raw: Vec<Vec<char>>) -> Schematic {
        if raw.len() == 0 || raw.first().unwrap().len() == 0 {
            panic!("Cannot accept raw schematic with 0 lenght!")
        }

        let y_limit: usize = raw.len() - 1;
        let x_limit: usize = raw.first().unwrap().len() - 1;

        return Schematic {
            raw,
            y_limit,
            x_limit,
        };
    }

    fn gen_to_check(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut to_check: Vec<(usize, usize)> = Vec::with_capacity(8);
        if x > 0 {
            to_check.push((x - 1, y));
            if y < self.y_limit {
                to_check.push((x - 1, y + 1));
            }
            if y > 0 {
                to_check.push((x - 1, y - 1));
            }
        }
        if x < self.x_limit {
            to_check.push((x + 1, y));
            if y < self.y_limit {
                to_check.push((x + 1, y + 1));
            }
            if y > 0 {
                to_check.push((x + 1, y - 1));
            }
        }
        if y < self.y_limit {
            to_check.push((x, y + 1));
        }
        if y > 0 {
            to_check.push((x, y - 1));
        }

        return to_check;
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        return self.raw.get(y).and_then(|xs| xs.get(x));
    }

    fn iter(&self) -> Box<dyn Iterator<Item = ((usize, usize), &char)> + '_> {
        let iter =
            self.raw.iter().enumerate().flat_map(move |(y, xs)| {
                xs.iter().enumerate().map(move |(x, value)| ((x, y), value))
            });

        return Box::new(iter);
    }
}

fn part1(schematic: &Schematic) -> i64 {
    let mut number: Vec<char> = Vec::new();
    let mut number_adjacent = false;
    let mut sum: i64 = 0;

    for ((x, y), value) in schematic.iter() {
        match value {
            '0'..='9' => {
                number.push(*value);
                let to_check = schematic.gen_to_check(x, y);
                for (cx, cy) in to_check {
                    let char = schematic.get(cx, cy).unwrap();
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
    return sum;
}

fn part2(schematic: &Schematic) -> u64 {
    let mut gear_candidates: HashMap<(usize, usize), Vec<u64>> = HashMap::new();

    let mut number: Vec<char> = Vec::new();
    let mut number_gear: Option<(usize, usize)> = None;

    for ((x, y), value) in schematic.iter() {
        match value {
            '0'..='9' => {
                number.push(*value);
                let to_check = schematic.gen_to_check(x, y);
                for (cx, cy) in to_check {
                    let char = schematic.get(cx, cy).unwrap();
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
    let schematic = Schematic::from(
        read_lines("./inputs/day03")
            .unwrap()
            .map(|lr| lr.unwrap().chars().collect::<Vec<char>>())
            .collect(),
    );

    println!("Part 1: {}", part1(&schematic));
    println!("Part 2: {}", part2(&schematic));
}
