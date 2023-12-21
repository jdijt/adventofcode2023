use aoc2023::{read_lines, run_timed};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ptr::write;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum MapTile {
    Rock,
    Garden,
}

impl MapTile {
    fn from(c: char) -> MapTile {
        match c {
            'S' | '.' => MapTile::Garden,
            '#' => MapTile::Rock,
            _ => panic!("Invalid map tile: {}", c),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn valid_steps(&self, context: &Garden) -> Vec<Point> {
        let mut maybe_valid_steps: Vec<Point> = Vec::new();
        if self.x > 1 {
            maybe_valid_steps.push(Point::from(self.x - 1, self.y))
        }
        if self.x < context.x_len - 1 {
            maybe_valid_steps.push(Point::from(self.x + 1, self.y))
        }
        if self.y > 1 {
            maybe_valid_steps.push(Point::from(self.x, self.y - 1))
        }
        if self.y < context.x_len - 1 {
            maybe_valid_steps.push(Point::from(self.x, self.y + 1))
        }

        maybe_valid_steps
            .into_iter()
            .filter(|p| context.is_free(p))
            .collect()
    }
}

struct Garden {
    data: Vec<Vec<MapTile>>,
    x_len: usize,
    y_len: usize,
}

impl Garden {
    fn from_file(file_name: &str) -> (Point, Garden) {
        let mut garden = Garden {
            data: vec![],
            x_len: 0,
            y_len: 0,
        };
        let mut start = None;

        for (y, line) in read_lines(file_name).enumerate() {
            let mut plots = vec![];
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some(Point::from(x, y))
                }
                plots.push(MapTile::from(c))
            }
            garden.y_len += 1;
            garden.x_len = plots.len();
            garden.data.push(plots);
        }

        (start.unwrap(), garden)
    }

    fn is_free(&self, point: &Point) -> bool {
        match self.data.get(point.y).and_then(|xs| xs.get(point.x)) {
            Some(MapTile::Garden) => true,
            _ => false,
        }
    }
}

fn find_reachable(garden: &Garden, start: Point, steps_target: u32) -> u32 {
    let mut todo = VecDeque::new();
    todo.push_back((start, 0));

    let mut reachable = HashSet::new();
    let mut visited = HashSet::new();

    while let Some((point, steps_taken)) = todo.pop_front() {
        if reachable.contains(&point) {
            continue;
        }
        if visited.contains(&point) && steps_target % steps_taken == 0
            || steps_taken == steps_target
        {
            reachable.insert(point);
            continue;
        }

        for next in point.valid_steps(garden) {
            todo.push_back((next, steps_taken + 1));
        }
        visited.insert(point);
    }

    reachable.len() as u32
}

fn main() {
    let (start, garden) = Garden::from_file("./inputs/day21_test");
    println!(
        "Part 1: {}",
        run_timed(|| find_reachable(&garden, start, 6))
    )
}
