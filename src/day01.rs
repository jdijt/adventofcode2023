use std::collections::HashMap;

use super::advent_util::read_lines;

fn find_first<'a>(s: &String, search_for: &'a Vec<&str>) -> &'a &'a str {
    return search_for
        .iter()
        .flat_map(|opt| s.find(opt).map(|idx| (idx, opt)))
        .min_by_key(|t| t.0)
        .unwrap()
        .1;
}

fn find_last<'a>(s: &String, search_for: &'a Vec<&str>) -> &'a &'a str {
    return search_for
        .iter()
        .flat_map(|opt| s.rfind(opt).map(|idx| (idx, opt)))
        .max_by_key(|t| t.0)
        .unwrap()
        .1;
}

fn part1() -> i32 {
    let mut sum = 0;
    let search_for = Vec::from(["1", "2", "3", "4", "5", "6", "7", "8", "9"]);

    if let Ok(lines) = read_lines("./inputs/day1") {
        for line in lines {
            if let Ok(l) = line {
                let num = format!(
                    "{}{}",
                    find_first(&l, &search_for),
                    find_last(&l, &search_for)
                )
                .parse::<i32>()
                .unwrap();

                sum += num
            }
        }
    }

    return sum;
}

fn part2() -> i32 {
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
    let mut sum = 0;

    if let Ok(lines) = read_lines("./inputs/day1") {
        for line in lines {
            if let Ok(l) = line {
                let first = numerics.get(find_first(&l, &search_for)).unwrap();
                let last = numerics.get(find_last(&l, &search_for)).unwrap();
                let num = format!("{}{}", first,last).parse::<i32>().unwrap();
                sum += num;
            }
        }
    }
    return sum;
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}
