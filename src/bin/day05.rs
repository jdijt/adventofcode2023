use aoc2023::{read_lines, run_timed};
use std::ops::Range;
use std::thread;
use std::thread::JoinHandle;

#[derive(Debug, Clone)]
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

fn parse_file() -> (Vec<i64>, Vec<Vec<Mapping>>) {
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

    let mappings: Vec<Vec<Mapping>> = lines.fold(Vec::new(), |mut acc, line| {
        if line.starts_with(|c: char| c.is_digit(10)) {
            let  ms = acc.last_mut().unwrap();
            ms.push(Mapping::from(line).unwrap());
        } else if line.starts_with(|c: char| c.is_alphabetic()) {
            acc.push(Vec::new());
        }
        acc
    });

    (seeds, mappings)
}

fn part1(seeds: &Vec<i64>, mappings: &Vec<Vec<Mapping>>) -> i64 {
    let mapped_seeds = mappings.iter().fold(seeds.clone(), |acc, ms| {
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

fn part2(seeds: &Vec<i64>, mappings: &Vec<Vec<Mapping>>) -> i64 {
    let expanded_seeds: Vec<Range<i64>> = seeds
        .chunks_exact(2)
        .map(|chunk| {
            let from = *chunk.first().unwrap();
            let to = from + chunk.last().unwrap();
            from..to
        })
        .collect();


    let threads: Vec<JoinHandle<i64>> = expanded_seeds.into_iter().map(|range| {
        let mappings_clone: Vec<Vec<Mapping>> = mappings.clone();
        thread::spawn(move || {
            let mut seeds = range.collect::<Vec<i64>>();
            for ms in mappings_clone {
                for v in seeds.iter_mut() {
                    if let Some(m) = ms.iter().find(|m| m.applies_to.contains(&v)) {
                        m.mutate(v);
                    }
                }
            }
            *seeds.iter().min().unwrap()
        })
    }).collect();

    threads.into_iter().map(|j| j.join().unwrap()).min().unwrap()
}

fn main() {
    let (seeds, mappings) = parse_file();
    println!("Part 1: {}", run_timed(|| part1(&seeds, &mappings)));
    println!("Part 2: {}", run_timed(|| part2(&seeds, &mappings)));
}
