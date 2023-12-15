use crate::Instr::{Place, Remove};
use aoc2023::{read_lines, run_timed};

fn aoc_hash(s: &str) -> u16 {
    s.bytes().fold(0, |acc, b| (acc + b as u16) * 17 % 256)
}

#[derive(Eq, PartialEq, Debug)]
struct Lens {
    label: String,
    hash: u16,
}

impl Lens {
    fn from(s: &str) -> Lens {
        Lens {
            label: String::from(s),
            hash: aoc_hash(s),
        }
    }
}

#[derive(Debug)]
enum Instr {
    Remove(Lens),
    Place(Lens, u32),
}

impl Instr {
    fn from(s: &str) -> Instr {
        if s.ends_with('-') {
            Remove(Lens::from(&s[..s.len() - 1]))
        } else {
            let (label, f) = s.split_once('=').unwrap();
            Place(Lens::from(label), f.parse().unwrap())
        }
    }
}

#[derive(Debug)]
struct Boxes {
    store: Vec<Vec<(Lens, u32)>>,
}

impl Boxes {
    fn new() -> Boxes {
        let mut store = Vec::with_capacity(256);
        for _ in 0..256 {
            store.push(Vec::with_capacity(32));
        }
        Boxes { store }
    }

    fn run_instr(&mut self, instr: Instr) {
        match instr {
            Remove(lens) => {
                let container = self.store.get_mut(lens.hash as usize).unwrap();

                if let Some(idx) = container.iter().position(|(l, _)| *l == lens) {
                    container.remove(idx);
                }
            }
            Place(lens, new_focal) => {
                let container = self.store.get_mut(lens.hash as usize).unwrap();

                if let Some((_, ref mut focal)) = container.iter_mut().find(|(l, _)| *l == lens) {
                    *focal = new_focal
                } else {
                    container.push((lens, new_focal))
                }
            }
        }
    }

    fn score(&self) -> u32 {
        self.store
            .iter()
            .enumerate()
            .map(|(box_id, container)| {
                container
                    .iter()
                    .enumerate()
                    .fold(0u32, |acc, (idx, (_, focal))| {
                        acc + ((box_id as u32 + 1) * (idx as u32 + 1) * *focal)
                    })
            })
            .sum()
    }
}

fn part2(raw_instrs: &Vec<String>) -> u32 {
    let instrs = raw_instrs.iter().map(|s| Instr::from(s));
    let mut boxes = Boxes::new();

    for i in instrs {
        boxes.run_instr(i);
    }

    boxes.score()
}

fn main() {
    let raw_instrs: Vec<String> = read_lines("./inputs/day15")
        .next()
        .unwrap()
        .split(',')
        .map(String::from)
        .collect();

    println!(
        "Part 1: {}",
        run_timed(|| {
            raw_instrs
                .iter()
                .map(|s| aoc_hash(s))
                .fold(0u32, |acc, h| acc + h as u32)
        })
    );

    println!("Part 2: {}", run_timed(|| { part2(&raw_instrs) }));
}
