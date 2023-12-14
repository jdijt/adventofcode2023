use crate::Elem::{Cube, Empty, Round};
use aoc2023::{read_lines, run_timed};
use std::mem::swap;
use std::process::id;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum Elem {
    Round,
    Cube,
    Empty,
}

impl Elem {
    fn from(c: char) -> Elem {
        match c {
            '#' => Cube,
            'O' => Round,
            '.' => Empty,
            _ => panic!("Invalid field element: {}", c),
        }
    }
}

type Field = Vec<Vec<Elem>>;

fn shift_north(fld: &mut Field) {
    let mut changed = true;
    while changed {
        //Reset flag:
        changed = false;
        for y in 0..fld.len() - 1 {
            if let [ref mut row_n, ref mut row_s] = fld[y..y + 2] {
                for p in row_n.iter_mut().zip(row_s.iter_mut()) {
                    if let (n @ Empty, s @ Round) = p {
                        swap(n, s);
                        changed = true
                    }
                }
            }
        }
    }
}

fn shift_west(fld: &mut Field) {
    let mut changed = true;
    while changed {
        changed = false;
        for x in 0..fld.first().unwrap().len() - 1 {
            for row in fld.iter_mut() {
                if let [ref mut w @ Elem::Empty, ref mut e @ Elem::Round] = row[x..x + 2] {
                    swap(w, e);
                    changed = true
                }
            }
        }
    }
}

fn shift_south(fld: &mut Field) {
    let mut changed = true;
    while changed {
        //Reset flag:
        changed = false;
        for y in (0..fld.len() - 1).rev() {
            if let [ref mut row_n, ref mut row_s] = fld[y..y + 2] {
                for p in row_n.iter_mut().zip(row_s.iter_mut()) {
                    if let (n @ Round, s @ Elem::Empty) = p {
                        swap(n, s);
                        changed = true
                    }
                }
            }
        }
    }
}

fn shift_east(fld: &mut Field) {
    let mut changed = true;
    while changed {
        changed = false;
        for x in (0..fld.first().unwrap().len() - 1).rev() {
            for row in fld.iter_mut() {
                if let [ref mut w @ Elem::Round, ref mut e @ Elem::Empty] = row[x..x + 2] {
                    swap(w, e);
                    changed = true
                }
            }
        }
    }
}

fn field_score(field: &Field) -> u64 {
    let len = field.len();
    field
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .map(|e| if *e == Round { (len - y) as u64 } else { 0 })
                .sum::<u64>()
        })
        .sum()
}

fn part1(field: &Field) -> u64 {
    let mut work_field = field.clone();

    shift_north(&mut work_field);

    field_score(&work_field)
}

fn part2(field: &Field) -> u64 {
    let mut work_field = field.clone();
    let mut counter = 0;
    let mut past_fields: Vec<Field> = vec![work_field.clone()];
    let mut first_match_idx: usize = 0;

    while counter < 1_000_000_000 {
        shift_north(&mut work_field);
        shift_west(&mut work_field);
        shift_south(&mut work_field);
        shift_east(&mut work_field);

        if let Some(idx) = past_fields.iter().position(|f| f == &work_field) {
            first_match_idx = idx;
            break;
        }
        past_fields.push(work_field.clone());

        counter += 1;
    }

    let loop_len = past_fields.len() - first_match_idx;
    let rem = (1_000_000_000 - first_match_idx) % loop_len;

    field_score(&past_fields.get(first_match_idx + rem).unwrap())
}

fn main() {
    let field: Field = read_lines("./inputs/day14")
        .map(|l| l.chars().map(Elem::from).collect())
        .collect();

    println!("Part 1: {}", run_timed(|| part1(&field)));
    println!("Part 2: {}", run_timed(|| part2(&field)));
}
