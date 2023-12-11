use aoc2023::{read_lines, run_timed};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn from(x: usize, y: usize) -> Point {
        Point {
            x: x as u64,
            y: y as u64,
        }
    }

    fn distance(&self, other: &Point) -> u64 {
        self.y.abs_diff(other.y) + self.x.abs_diff(other.x)
    }
}

#[derive(Debug)]
struct StarMap {
    stars: Vec<Point>,
    y_size: u64,
    x_size: u64,
}

impl StarMap {
    fn from_file() -> StarMap {
        let mut stars: Vec<Point> = Vec::new();
        if let Ok(lines) = read_lines("./inputs/day11") {
            for (y, lr) in lines.enumerate() {
                if let Ok(l) = lr {
                    for (x, c) in l.chars().enumerate() {
                        if c == '#' {
                            stars.push(Point::from(x, y));
                        }
                    }
                }
            }
        }

        StarMap::from_points(stars)
    }

    fn from_points(stars: Vec<Point>) -> StarMap {
        let y_size = stars.iter().map(|p| p.y).max().unwrap();
        let x_size = stars.iter().map(|p| p.x).max().unwrap();

        StarMap {
            stars,
            y_size,
            x_size,
        }
    }

    fn expand(&self, factor: u64) -> StarMap {
        let mut xs: HashSet<u64> = (0..self.x_size).collect();
        let mut ys: HashSet<u64> = (0..self.y_size).collect();

        for Point { x, y } in self.stars.iter() {
            xs.remove(&x);
            ys.remove(&y);
        }

        let expanded_stars = self
            .stars
            .iter()
            .map(|Point { x, y }| {
                let x_expand = xs.iter().filter(|i| **i < *x).count() as u64;
                let y_expand = ys.iter().filter(|i| **i < *y).count() as u64;

                Point {
                    x: x + (x_expand * (factor - 1)),
                    y: y + (y_expand * (factor - 1)),
                }
            })
            .collect();

        Self::from_points(expanded_stars)
    }
}

fn distance_sum(star_map: &StarMap, factor: u64) -> u64 {
    let mut expanded_stars = star_map.expand(factor).stars;
    let mut distance_sum = 0;

    while !expanded_stars.is_empty() {
        let elem = expanded_stars.pop().unwrap();
        for other in expanded_stars.iter() {
            distance_sum += elem.distance(other)
        }
    }

    distance_sum
}

fn main() {
    let stars = StarMap::from_file();

    println!("Part 1: {}", run_timed(|| distance_sum(&stars, 2)));
    println!("Part 2: {}", run_timed(|| distance_sum(&stars, 1_000_000)));
}
