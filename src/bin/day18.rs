use aoc2023::{read_lines, run_timed};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn from(c: &str) -> Dir {
        match c {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => panic!("Invalid dir: {}", c),
        }
    }
    fn from_p2(c: &str) -> Dir {
        match c {
            "0" => Dir::Right,
            "1" => Dir::Down,
            "2" => Dir::Left,
            "3" => Dir::Up,
            _ => panic!("Invalid dir: {}", c),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    steps: u32,
    dir: Dir,
}

impl Instruction {
    fn from_line(line: &str) -> Instruction {
        if let [d, st, _co] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            let dir = Dir::from(d);
            let steps = st.parse().unwrap();
            Instruction { dir, steps }
        } else {
            panic!("Invalid line: {}", line);
        }
    }
    fn from_file(file_name: &str) -> Vec<Instruction> {
        read_lines(file_name).map(|l| Self::from_line(&l)).collect()
    }

    fn from_line_p2(line: &str) -> Instruction {
        if let Some(n) = line.split_whitespace().skip(2).next() {
            let steps = u32::from_str_radix(&n[2..7], 16).unwrap();
            let dir = Dir::from_p2(&n[7..8]);
            Instruction { steps, dir }
        } else {
            panic!("Invalid line: {}", line);
        }
    }

    fn from_file_p2(file_name: &str) -> Vec<Instruction> {
        read_lines(file_name)
            .map(|l| Self::from_line_p2(&l))
            .collect()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn next(&self, dir: Dir) -> Point {
        match dir {
            Dir::Up => Point {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Down => Point {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Left => Point {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Right => Point {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct Field {
    points: HashMap<i64, HashSet<i64>>,
}

impl Field {
    fn new() -> Field {
        Field {
            points: HashMap::new(),
        }
    }

    fn put(&mut self, point: Point) {
        if let Some(xs) = self.points.get_mut(&point.y) {
            xs.insert(point.x);
        } else {
            self.points.insert(point.y, {
                let mut new_set = HashSet::new();
                new_set.insert(point.x);
                new_set
            });
        }
    }

    fn contains(&self, x: &i64, y: &i64) -> bool {
        if let Some(xs) = self.points.get(y) {
            xs.contains(x)
        } else {
            false
        }
    }
}

fn lagoon_size(instrs: &Vec<Instruction>) -> u64 {
    let mut field = Field::new();
    let mut trench_position = Point { x: 0, y: 0 };
    field.put(trench_position);

    for instr in instrs {
        for _ in 0..instr.steps {
            trench_position = trench_position.next(instr.dir);
            field.put(trench_position)
        }
    }

    let mut count = 0u64;

    for (y, xs) in field.points.iter() {
        let mut row = xs.iter().copied().collect::<Vec<i64>>();
        row.sort();

        //find the start of the gaps:
        let mut group_slices = Vec::new();
        let mut last_group_start = 0;
        for (idx, w) in row.windows(2).enumerate() {
            if let [x1, x2] = w {
                if x1.abs_diff(*x2) > 1 {
                    group_slices.push(&row[last_group_start..idx + 1]);
                    last_group_start = idx + 1;
                }
            }
        }
        group_slices.push(&row[last_group_start..]);

        let mut last_loop_enter = None;
        for g in group_slices {
            count += g.len() as u64;
            let group_start = g.first().unwrap();
            let group_end = g.last().unwrap();
            if let Some(prev_x) = last_loop_enter {
                count += group_start.abs_diff(prev_x) - 1; //don't include the trenches!

                //Check if we leave the loop:
                if (field.contains(group_start, &(y - 1)) && field.contains(group_end, &(y + 1)))
                    || (field.contains(group_start, &(y + 1))
                        && field.contains(group_end, &(y - 1)))
                {
                    last_loop_enter = None
                } else {
                    last_loop_enter = Some(*group_end);
                }
            } else if (field.contains(group_start, &(y - 1)) && field.contains(group_end, &(y + 1)))
                || (field.contains(group_start, &(y + 1)) && field.contains(group_end, &(y - 1)))
            {
                last_loop_enter = Some(*group_end);
            }
        }
    }

    count as u64
}

fn main() {
    let file_name = "./inputs/day18";

    println!(
        "Part 1: {}",
        run_timed(|| lagoon_size(&Instruction::from_file(file_name)))
    );
    println!(
        "Part 1: {}",
        run_timed(|| lagoon_size(&Instruction::from_file_p2(file_name)))
    );
}
