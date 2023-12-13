use aoc2023::{read_lines, run_timed};
use std::collections::HashMap;

fn find_first<'a>(s: &String, search_for: &Vec<&'a str>) -> &'a str {
    let (_, res) = search_for
        .iter()
        .flat_map(|opt| s.find(opt).map(|idx| (idx, opt)))
        .min_by_key(|t| t.0)
        .unwrap();

    return res;
}

fn find_last<'a>(s: &String, search_for: &Vec<&'a str>) -> &'a str {
    let (_, res) = search_for
        .iter()
        .flat_map(|opt| s.rfind(opt).map(|idx| (idx, opt)))
        .max_by_key(|t| t.0)
        .unwrap();

    return res;
}

fn part1(lines: &Vec<String>) -> i32 {
    let search_for = Vec::from(["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

    lines
        .iter()
        .map(|line| {
            let val = format!(
                "{}{}",
                find_first(&line, &search_for),
                find_last(&line, &search_for)
            );
            return val.parse::<i32>().unwrap();
        })
        .sum()
}

fn part2(lines: &Vec<String>) -> i32 {
    let numerics = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let search_for: Vec<&str> = numerics.keys().copied().collect();

    lines
        .iter()
        .map(|line| {
            let first = numerics.get(find_first(&line, &search_for)).unwrap();
            let last = numerics.get(find_last(&line, &search_for)).unwrap();
            return format!("{}{}", first, last).parse::<i32>().unwrap();
        })
        .sum()
}

fn main() {
    let lines = read_lines("./inputs/day01").collect::<Vec<String>>();

    println!("Part 1: {}", run_timed(|| part1(&lines)));
    println!("Part 2: {}", run_timed(|| part2(&lines)));
}
