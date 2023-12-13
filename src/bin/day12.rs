use crate::SpringState::{Damaged, Operational, Unknown};
use aoc2023::{read_lines, run_timed};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum SpringState {
    Unknown,
    Damaged,
    Operational,
}

impl SpringState {
    fn from(c: char) -> SpringState {
        match c {
            '#' => Damaged,
            '.' => Operational,
            '?' => Unknown,
            _ => panic!("Invalid spring state: {:?}", c),
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct SpringData {
    history: Vec<SpringState>,
    damage_groups: Vec<usize>,
}

impl SpringData {
    fn from(l: String) -> SpringData {
        if let Some((hist_section, dmg_groups_section)) = l.split_once(" ") {
            let history = hist_section.chars().map(SpringState::from).collect();
            let damage_groups = dmg_groups_section
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect();
            SpringData {
                history,
                damage_groups,
            }
        } else {
            panic!("Invalid line {:?}", l)
        }
    }

    fn expand(&self) -> SpringData {
        let mut expanded_history = self.history.clone();
        for _ in 0..4 {
            expanded_history.push(Unknown);
            self.history.iter().for_each(|e| expanded_history.push(*e))
        }
        SpringData {
            history: expanded_history,
            damage_groups: self.damage_groups.repeat(5),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Res {
    Invalid,
    Valid(u64),
}

impl Res {
    fn value(&self) -> u64 {
        match self {
            Res::Invalid => 0,
            Res::Valid(n) => *n,
        }
    }

    fn combine(&self, other: &Res) -> Res {
        match (self, other) {
            (Res::Invalid, Res::Invalid) => Res::Invalid,
            (Res::Invalid, v @ Res::Valid(_)) => *v,
            (v @ Res::Valid(_), Res::Invalid) => *v,
            (Res::Valid(a), Res::Valid(b)) => Res::Valid(a + b),
        }
    }
}

type Memory = HashMap<(usize, usize), Res>;

fn put_group(group: usize, line: &[SpringState], groups: &[usize], memory: &mut Memory) -> Res {
    //Check if group fits in the line.
    if group > line.len() {
        Res::Invalid
    // Check we do not conflict with known data in the line
    } else if line[..group].iter().any(|e| *e == Operational) {
        Res::Invalid
    //Check if we fit exactly (end of line group)
    } else if group == line.len() {
        count_options(&line[group..], groups, memory)
    //Check if an "operational" after the damaged group conflicts (we're not end of line here)
    } else if line[group] == Damaged {
        Res::Invalid
    //Place group & "Operational" after.
    } else {
        count_options(&line[group + 1..], groups, memory)
    }
}

fn count_options(line: &[SpringState], groups: &[usize], memory: &mut Memory) -> Res {
    // Memoize by length:
    let key = (line.len(), groups.len());
    if let Some(res) = memory.get(&key) {
        *res
    } else {
        let res = match line.split_first() {
            // Cannot place a Damaged group here -> next.
            Some((Operational, l_rem)) => count_options(l_rem, groups, memory),
            // Should place the next Damaged group here.
            Some((Damaged, _)) => {
                if let Some((group, g_rem)) = groups.split_first() {
                    put_group(*group, line, g_rem, memory)
                } else {
                    //We must be able to place a group here but cannot -> invalid.
                    Res::Invalid
                }
            }
            //Here we can choose to place or not
            Some((Unknown, l_rem)) => {
                if let Some((group, g_rem)) = groups.split_first() {
                    let not_put = count_options(l_rem, groups, memory);
                    let put = put_group(*group, line, g_rem, memory);

                    put.combine(&not_put)
                } else {
                    //No groups available, this "unknown" must be "operational".
                    count_options(l_rem, groups, memory)
                }
            }
            None => {
                //We're through the line but a group is left -> invalid
                if groups.len() > 0 {
                    Res::Invalid
                } else {
                    Res::Valid(1)
                }
            }
        };
        memory.insert(key, res);
        res
    }
}

fn main() {
    let input: Vec<SpringData> = read_lines("./inputs/day12").map(SpringData::from).collect();

    println!(
        "Part 1: {}",
        run_timed(|| {
            input
                .iter()
                .map(|l| {
                    count_options(&l.history[..], &l.damage_groups[..], &mut Memory::new()).value()
                })
                .sum::<u64>()
        })
    );
    println!(
        "Part 1: {}",
        run_timed(|| {
            input
                .iter()
                .map(|l| {
                    let expanded = l.expand();
                    count_options(
                        &expanded.history[..],
                        &expanded.damage_groups[..],
                        &mut Memory::new(),
                    )
                    .value()
                })
                .sum::<u64>()
        })
    );
}
