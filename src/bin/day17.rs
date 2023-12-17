use aoc2023::{read_lines, run_timed};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
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
    fn get_valid_neighbours(&self, incoming: Dir, map: &Costs) -> Vec<(Dir, Point)> {
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
    fn dist(&self, other: &Self) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct State {
    cost: u32,
    point: Point,
    dir: Dir,
    steps: u32,
    dist: u32,
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
        (other.dist + other.cost).cmp(&(self.cost + self.dist))
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct StateKey {
    point: Point,
    dir: Dir,
    steps: u32,
}

impl From<State> for StateKey {
    fn from(value: State) -> Self {
        StateKey {
            point: value.point,
            dir: value.dir,
            steps: value.steps,
        }
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

fn find_cheapest_route(
    costs: &Costs,
    start: Point,
    target: Point,
    min_steps: u32,
    max_steps: u32,
) -> u32 {
    let mut to_visit: BinaryHeap<State> = BinaryHeap::new();
    let mut known_costs: HashMap<StateKey, u32> = HashMap::new();
    let start_down = State {
        cost: 0,
        steps: 0,
        point: start,
        dir: Dir::Down,
        dist: start.dist(&target),
    };
    let start_right = State {
        cost: 0,
        steps: 0,
        point: start,
        dir: Dir::Right,
        dist: start.dist(&target),
    };
    to_visit.push(start_down);
    to_visit.push(start_right);
    known_costs.insert(start_right.into(), 0);
    known_costs.insert(start_down.into(), 0);

    while let Some(st) = to_visit.pop() {
        if st.point == target {
            return st.cost;
        }
        //Already encountered a cheaper use of this point
        if known_costs.get(&st.into()).is_some_and(|&c| c < st.cost) {
            continue;
        }

        for (dir, next) in st.point.get_valid_neighbours(st.dir, costs) {
            let new_st = State {
                cost: st.cost + costs.cost_at(&next),
                point: next,
                dir,
                steps: if dir == st.dir { st.steps + 1 } else { 1 },
                dist: next.dist(&target),
            };
            //Skip condition: too many steps, or we know a cheaper alt.
            //Or (p2) we make a turn before min steps.
            if new_st.steps > max_steps
                || known_costs
                    .get(&new_st.into())
                    .is_some_and(|&c| c <= new_st.cost)
                || (new_st.dir != st.dir && st.steps < min_steps)
            {
                continue;
            }
            to_visit.push(new_st);
            known_costs.insert(new_st.into(), new_st.cost);
        }
    }

    u32::MAX
}

fn main() {
    let costs = Costs::read_from_file("./inputs/day17");

    println!(
        "Part 1: {}",
        run_timed(|| find_cheapest_route(
            &costs,
            Point::from(0, 0),
            Point::from(costs.x_len - 1, costs.y_len - 1),
            1,
            3
        ))
    );
    println!(
        "Part 2: {}",
        run_timed(|| find_cheapest_route(
            &costs,
            Point::from(0, 0),
            Point::from(costs.x_len - 1, costs.y_len - 1),
            4,
            10
        ))
    );
}
