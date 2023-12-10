use crate::Direction::{East, North, South, West};
use crate::MapTile::{
    Ground, Horizontal, NorthToEast, NorthToWest, SouthToEast, SouthToWest, Start, Vertical,
};
use aoc2023::{read_lines, run_timed};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [North, South, East, West]
    }
    fn opposite(&self) -> Direction {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MapTile {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl MapTile {
    fn connects(&self, dir: &Direction) -> bool {
        let d = *self;
        match dir {
            North => d == Vertical || d == NorthToEast || d == NorthToWest,
            South => d == Vertical || d == SouthToWest || d == SouthToEast,
            East => d == Horizontal || d == NorthToEast || d == SouthToEast,
            West => d == Horizontal || d == NorthToWest || d == SouthToWest,
        }
    }

    fn get_new_dir(&self, old_dir: &Direction) -> Direction {
        match self {
            Vertical => match old_dir {
                North | South => old_dir.clone(),
                _ => panic!("Invalid direction {:?} for Vertical", old_dir),
            },
            Horizontal => match old_dir {
                West | East => old_dir.clone(),
                _ => panic!("Invalid direction {:?} for Horizontal", old_dir),
            },
            NorthToEast => match old_dir {
                South => East,
                West => North,
                _ => panic!("Invalid direction {:?} for NtE", old_dir),
            },
            NorthToWest => match old_dir {
                South => West,
                East => North,
                _ => panic!("Invalid direction {:?} for NtW", old_dir),
            },
            SouthToWest => match old_dir {
                North => West,
                East => South,
                _ => panic!("Invalid direction {:?} for StW", old_dir),
            },
            SouthToEast => match old_dir {
                North => East,
                West => South,
                _ => panic!("Invalid direction {:?} for StE", old_dir),
            },
            _ => panic!("Cannot get new dir for {:?}", self),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn step(&self, dir: &Direction) -> Position {
        match dir {
            North => Position {
                x: self.x,
                y: self.y - 1,
            },
            South => Position {
                x: self.x,
                y: self.y + 1,
            },
            East => Position {
                x: self.x + 1,
                y: self.y,
            },
            West => Position {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<MapTile>>);

impl Map {
    fn from_file() -> Map {
        let parsed = read_lines("./inputs/day10")
            .unwrap()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        '|' => Vertical,
                        '-' => Horizontal,
                        'L' => NorthToEast,
                        'J' => NorthToWest,
                        '7' => SouthToWest,
                        'F' => SouthToEast,
                        'S' => Start,
                        '.' => Ground,
                        _ => panic!("Invalid character: {}", c),
                    })
                    .collect()
            })
            .collect();

        Map(parsed)
    }

    fn get(&self, position: &Position) -> Option<&MapTile> {
        if position.x < 0 || position.y < 0 {
            None
        } else {
            self.0
                .get(position.y as usize)
                .and_then(|xs| xs.get(position.x as usize))
        }
    }

    fn get_valid_directions(&self, pos: &Position) -> Vec<Direction> {
        let mut result = Vec::new();

        for dir in Direction::all() {
            if let Some(tile) = self.get(&pos.step(&dir)) {
                if tile.connects(&dir.opposite()) {
                    result.push(dir)
                }
            }
        }

        result
    }

    fn find_start(&self) -> Option<Position> {
        for (y, xs) in self.0.iter().enumerate() {
            for (x, tile) in xs.iter().enumerate() {
                if *tile == Start {
                    return Some(Position {
                        x: x as i32,
                        y: y as i32,
                    });
                }
            }
        }
        return None;
    }

    fn rows_iter(&self) -> Box<dyn Iterator<Item = (usize, &Vec<MapTile>)> + '_> {
        let iter = self.0.iter().enumerate();

        Box::new(iter)
    }
}

fn compute_loop(map: &Map) -> Vec<Position> {
    let start = map.find_start().unwrap();
    let mut loop_positions = Vec::new();
    let mut current_direction = map.get_valid_directions(&start).first().unwrap().clone();
    let mut current_pos = start.step(&current_direction);
    loop_positions.push(current_pos.clone());

    while current_pos != start {
        current_direction = map
            .get(&current_pos)
            .unwrap()
            .get_new_dir(&current_direction);
        current_pos = current_pos.step(&current_direction);
        loop_positions.push(current_pos.clone())
    }

    loop_positions
}

fn part1(map: &Map) -> i32 {
    let full_loop = compute_loop(map);
    full_loop.len() as i32 / 2
}

fn part2(map: &Map) -> i32 {
    let full_loop_points: HashSet<Position> = HashSet::from_iter(compute_loop(map));
    let mut contained_count = 0;

    for (y, row) in map.rows_iter() {
        let mut loop_intersections = 0;
        let mut loop_enter: Option<MapTile> = None;
        for (x, tile) in row.iter().enumerate() {
            let pos = Position {
                x: x as i32,
                y: y as i32,
            };
            if full_loop_points.contains(&pos) {
                match (tile, loop_enter) {
                    // Hack (start == vertical) that may only work for my input.
                    (Vertical | Start, None) => loop_intersections += 1,
                    (NorthToEast | SouthToEast, None) => loop_enter = Some(tile.clone()),
                    // This is the "enter from north go along horizontal, then exit further south" case
                    // I.e.: a crossing.
                    (NorthToWest, Some(SouthToEast)) | (SouthToWest, Some(NorthToEast)) => {
                        loop_intersections += 1;
                        loop_enter = None;
                    }
                    // This is the "enter from north, go along horizontal, then go back north" case
                    // I.e.: Not a crossing.
                    (SouthToWest, Some(SouthToEast)) | (NorthToWest, Some(NorthToEast)) => {
                        loop_enter = None
                    }
                    (Horizontal, Some(NorthToEast | SouthToEast)) => {}
                    _ => {
                        panic!("Invalid state: {:?}", (tile, loop_enter))
                    }
                };
            } else if loop_intersections > 0 && loop_intersections % 2 != 0 {
                contained_count += 1;
            }
        }
    }

    contained_count
}

fn main() {
    let map = Map::from_file();

    println!("Part 1: {}", run_timed(|| part1(&map)));
    println!("Part 2: {}", run_timed(|| part2(&map)));
}
