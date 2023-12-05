use std::collections::HashMap;
use aoc2023::read_lines;

#[derive(Debug)]
struct Mapping {
    dest_start: u64,
    src_start: u64,
    lenght: u64,
}

impl Mapping {
    fn from(line: String) -> Option<Mapping> {
        let parts: Vec<u64> = line
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        if parts.len() == 3 {
            Some(Mapping {
                dest_start: parts.get(0).unwrap().clone(),
                src_start: parts.get(1).unwrap().clone(),
                lenght: parts.get(2).unwrap().clone(),
            })
        } else {
            None
        }
    }
}

fn parse_file() -> (Vec<u64>, Vec<(String,Vec<Mapping>)>) {
    let mut lines= read_lines("./inputs/day05").unwrap().map(|l| l.unwrap());

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .strip_prefix("Seeds: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut mappings = Vec::new();
    let mut current_mapping = Vec::new();
    let mut current_name = "";



    (seeds, mappings)
}

fn main() {
    let (seeds, mappings) = parse_file();
    println!("{:?}", seeds);
    println!("{:?}", mappings)
}
