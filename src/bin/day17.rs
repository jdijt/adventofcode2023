use aoc2023::{read_lines, run_timed};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::SeekFrom::Start;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    fn get_valid_neighbours(&self, map: &Costs, incoming: Dir) -> Vec<(Dir, Point)> {
        let mut new_points = Vec::new();
        if self.x > 0 && incoming != Dir::Right {
            new_points.push((Dir::Left, Point::from(self.x - 1, self.y)))
        }
        if self.x < map.x_len - 1 && incoming != Dir::Left {
            new_points.push((Dir::Right, Point::from(self.x + 1, self.y)))
        }
        if self.y > 0 && incoming != Dir::Down {
            new_points.push((Dir::Up, Point::from(self.x, self.y - 1)))
        }
        if self.y < map.y_len - 1 && incoming != Dir::Up {
            new_points.push((Dir::Down, Point::from(self.x, self.y + 1)))
        }
        new_points
    }
}

#[derive(Eq, PartialEq)]
struct State {
    cost: u32,
    point: Point,
    last_direction: Dir,
    dir_steps: u32,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    // Need to flip ordering for
    // if cost + estimation == equal, take the one with the lower estimate.
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&(self.cost))
    }
}

struct Costs {
    data: Vec<Vec<u32>>,
    y_len: usize,
    x_len: usize,
}

impl Costs {
    fn read_from_file(file_name: &str) -> Costs {
        let data = read_lines(file_name)
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>();
        let y_size = data.len();
        let x_size = if let Some(xs) = data.first() {
            xs.len()
        } else {
            0
        };

        Costs {
            data,
            y_len: y_size,
            x_len: x_size,
        }
    }

    fn cost_at(&self, point: &Point) -> &u32 {
        self.data
            .get(point.y)
            .and_then(|xs| xs.get(point.x))
            .unwrap_or(&u32::MAX)
    }
}

fn find_cheapest_route(costs: &Costs) -> u32 {
    let target = Point::from(0, 0);
    let start = Point::from(costs.x_len - 1, costs.y_len - 1);
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
    let mut known_costs: Vec<Vec<u32>> = (0..costs.y_len)
        .map(|y| {
            (0..costs.x_len)
                .map(|x| {
                    if x == start.x && y == start.y {
                        *costs.cost_at(&Point::from(x, y))
                    } else {
                        u32::MAX
                    }
                })
                .collect()
        })
        .collect();

    for (dir, p) in start.get_valid_neighbours(costs, Dir::Right) {
        let cost = costs.cost_at(&p);
        to_visit.push(State {
            cost: *cost,
            point: p,
            last_direction: dir,
            dir_steps: 1,
        });
        if let Some(mut c) = known_costs.get_mut(p.y).and_then(|xs| xs.get_mut(p.x)) {
            *c = *cost
        }
    }

    while let Some(state) = to_visit.pop() {
        let next_options = state
            .point
            .get_valid_neighbours(costs, state.last_direction);

        for (dir, neighb) in next_options {
            let new_state = State {
                cost: state.cost + costs.cost_at(&neighb),
                point: neighb,
                last_direction: dir,
                dir_steps: if dir == state.last_direction {
                    state.dir_steps + 1
                } else {
                    1
                },
            };
            if new_state.dir_steps > 3 {
                continue;
            }
            if let Some(mut known_cost) = known_costs
                .get_mut(neighb.y)
                .and_then(|xs| xs.get_mut(neighb.x))
            {
                if new_state.cost <= *known_cost {
                    *known_cost = new_state.cost;
                    to_visit.push(new_state);
                }
            }
        }
    }

    *known_costs.get(target.y).unwrap().get(target.x).unwrap()
}

fn main() {
    let costs = Costs::read_from_file("./inputs/day17");

    println!("Part 1: {}", run_timed(|| find_cheapest_route(&costs)))
}
