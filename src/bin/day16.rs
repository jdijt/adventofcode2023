use aoc2023::{read_lines, run_timed};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn range_to_visit<'a>(
        &'a self,
        map: &Map,
        start: &'a Point,
    ) -> Box<dyn Iterator<Item = Point> + '_> {
        match self {
            Dir::Up => Box::new((0..start.y).rev().map(|y| Point { x: start.x, y })),
            Dir::Down => Box::new((start.y + 1..map.y_size).map(|y| Point { x: start.x, y })),
            Dir::Left => Box::new((0..start.x).rev().map(|x| Point { x, y: start.y })),
            Dir::Right => Box::new((start.x + 1..map.x_size).map(|x| Point { x, y: start.y })),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Element {
    RightToDownMirror,
    RightToUpMirror,
    Empty,
    HorizontalSplit,
    VerticalSplit,
}

impl Element {
    fn from(c: char) -> Element {
        match c {
            '.' => Element::Empty,
            '\\' => Element::RightToDownMirror,
            '/' => Element::RightToUpMirror,
            '-' => Element::HorizontalSplit,
            '|' => Element::VerticalSplit,
            _ => panic!("Bad map element: {}", c),
        }
    }
    fn new_dirs(&self, current: Dir) -> Vec<Dir> {
        match self {
            Element::RightToDownMirror => match current {
                Dir::Up => vec![Dir::Left],
                Dir::Down => vec![Dir::Right],
                Dir::Left => vec![Dir::Up],
                Dir::Right => vec![Dir::Down],
            },
            Element::RightToUpMirror => match current {
                Dir::Up => vec![Dir::Right],
                Dir::Down => vec![Dir::Left],
                Dir::Left => vec![Dir::Down],
                Dir::Right => vec![Dir::Up],
            },
            Element::HorizontalSplit => match current {
                Dir::Up | Dir::Down => vec![Dir::Left, Dir::Right],
                _ => vec![current],
            },
            Element::VerticalSplit => match current {
                Dir::Left | Dir::Right => vec![Dir::Up, Dir::Down],
                _ => vec![current],
            },
            Element::Empty => vec![current],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Element>>,
    y_size: usize,
    x_size: usize,
}

impl Map {
    fn from_file(f: &str) -> Map {
        let map: Vec<Vec<Element>> = read_lines(f)
            .map(|l| l.chars().map(Element::from).collect())
            .collect();
        let y_size = map.len();
        let x_size = if let Some(xs) = map.first() {
            xs.len()
        } else {
            0
        };

        Map {
            map,
            y_size,
            x_size,
        }
    }

    fn get(&self, point: &Point) -> Option<&Element> {
        self.map.get(point.y).and_then(|xs| xs.get(point.x))
    }
}

fn count_energized(map: &Map, start: Point, start_dir: Dir) -> usize {
    let mut visited: HashSet<(Dir, Point)> = HashSet::new();
    let mut to_visit: VecDeque<(Dir, Point)> = VecDeque::new();

    //Start state: just visited the start node from the start direction.
    visited.insert((start_dir, start));
    for dir in map.get(&start).unwrap().new_dirs(start_dir) {
        to_visit.push_back((dir, start));
    }

    while let Some((dir, path_start)) = to_visit.pop_front() {
        for p in dir.range_to_visit(&map, &path_start) {
            // if we're at an element in the same direction, it is a loop.
            if !visited.insert((dir, p)) {
                break;
            }
            match map.get(&p).unwrap() {
                Element::Empty => {} //carry on.
                other => {
                    for new_dir in other.new_dirs(dir) {
                        to_visit.push_back((new_dir, p))
                    }
                    break;
                }
            }
        }
    }

    visited
        .into_iter()
        .map(|(_, p)| p)
        .collect::<HashSet<Point>>()
        .len()
}

fn main() {
    let map = Map::from_file("./inputs/day16");

    println!(
        "Part 1: {}",
        run_timed(|| count_energized(&map, Point { x: 0, y: 0 }, Dir::Right))
    );
    println!(
        "Part 2: {}",
        run_timed(|| {
            let mut starting_points: Vec<(Point, Dir)> = Vec::new();
            for y in 0..map.y_size {
                let big_x = map.x_size - 1;
                starting_points.push((Point { x: 0, y }, Dir::Right));
                starting_points.push((Point { x: big_x, y }, Dir::Left));
            }
            for x in 0..map.x_size {
                let big_y = map.y_size - 1;
                starting_points.push((Point { x, y: 0 }, Dir::Down));
                starting_points.push((Point { x, y: big_y }, Dir::Up));
            }

            starting_points
                .into_iter()
                .map(|(p, dir)| count_energized(&map, p, dir))
                .max()
                .unwrap()
        })
    );
}
