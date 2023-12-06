use aoc2023::{read_lines, run_timed};
use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    applies_to: Range<i64>,
    offset: i64,
}

impl Mapping {
    fn from(line: String) -> Option<Mapping> {
        let parts: Vec<i64> = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if let [dest_start, src_start, len] = parts.as_slice() {
            Some(Mapping {
                applies_to: *src_start..(src_start + len),
                offset: dest_start - src_start,
            })
        } else {
            None
        }
    }

    fn apply_to(&self, value: i64) -> i64 {
        if self.applies_to.contains(&value) {
            value + self.offset
        } else {
            value
        }
    }

    fn mutate(&self, value: &mut i64) {
        if self.applies_to.contains(value) {
            *value += self.offset
        }
    }
}

fn parse_file() -> (Vec<i64>, Vec<(String, Vec<Mapping>)>) {
    let mut lines = read_lines("./inputs/day05").unwrap().map(|l| l.unwrap());

    let seeds: Vec<i64> = lines
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mappings: Vec<(String, Vec<Mapping>)> = lines.fold(Vec::new(), |mut acc, line| {
        if line.starts_with(|c: char| c.is_digit(10)) {
            let (_, ms) = acc.last_mut().unwrap();
            ms.push(Mapping::from(line).unwrap());
        } else if line.starts_with(|c: char| c.is_alphabetic()) {
            acc.push((String::from(line.strip_suffix(":").unwrap()), Vec::new()));
        }
        acc
    });

    (seeds, mappings)
}

fn part1(seeds: &Vec<i64>, mappings: &Vec<(String, Vec<Mapping>)>) -> i64 {
    let mapped_seeds = mappings.iter().fold(seeds.clone(), |acc, (_, ms)| {
        acc.into_iter()
            .map(|v| {
                ms.iter()
                    .find(|m| m.applies_to.contains(&v))
                    .map_or(v, |m| m.apply_to(v))
            })
            .collect()
    });

    *mapped_seeds.iter().min().unwrap()
}

fn part2(seeds: &Vec<i64>, mappings: &Vec<(String, Vec<Mapping>)>) -> i64 {
    let mut expanded_seeds: Vec<i64> = seeds
        .clone()
        .chunks_exact(2)
        .flat_map(|chunk| {
            let from = *chunk.first().unwrap();
            let to = from + chunk.last().unwrap();
            from..to
        })
        .collect();

    for (_, ms) in mappings {
        for v in expanded_seeds.iter_mut() {
            if let Some(m) = ms.iter().find(|m| m.applies_to.contains(&v)) {
                m.mutate(v);
            }
        }
    }

    *expanded_seeds.iter().min().unwrap()
}

fn main() {
    let (seeds, mappings) = parse_file();
    println!("Part 1: {}", run_timed(|| part1(&seeds, &mappings)));
    println!("Part 2: {}", run_timed(|| part2(&seeds, &mappings)));
}
