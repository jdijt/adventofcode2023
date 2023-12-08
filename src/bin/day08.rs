use aoc2023::{read_lines, run_timed};
use std::collections::HashMap;

fn read_input() -> (String, HashMap<String, (String, String)>) {
    let mut lines = read_lines("./inputs/day08").unwrap().map(|l| l.unwrap());

    let route = lines.next().unwrap();
    lines.next();

    let mut adjacency = HashMap::new();

    for l in lines {
        let (from, targets) = l.split_once(" = ").unwrap();
        let (l_target, r_target) = targets
            .strip_prefix("(")
            .unwrap()
            .strip_suffix(")")
            .unwrap()
            .split_once(", ")
            .unwrap();

        adjacency.insert(
            String::from(from),
            (String::from(l_target), String::from(r_target)),
        );
    }

    (route, adjacency)
}

fn process_route(
    start: &String,
    route: &String,
    adjacency: &HashMap<String, (String, String)>,
    target: fn(&String) -> bool,
) -> u64 {
    let mut route_eternal = route.chars().cycle();

    let mut step_count = 0;
    let mut position = start;

    while !target(position) {
        if let Some(direction) = route_eternal.next() {
            if let Some((l_target, r_target)) = adjacency.get(position) {
                if direction == 'L' {
                    position = l_target;
                } else {
                    position = r_target;
                }
                step_count += 1
            }
        }
    }

    step_count
}

fn main() {
    let (route, adjacency) = read_input();

    println!(
        "Part 1 {}",
        run_timed(|| process_route(&String::from("AAA"), &route, &adjacency, |s| s == "ZZZ"))
    );
    println!(
        "Part 2 {}",
        run_timed(|| {
            let route_starts: Vec<&String> =
                adjacency.keys().filter(|s| s.ends_with('A')).collect();

            let route_lengths: Vec<u128> = route_starts
                .into_iter()
                .map(|start| process_route(start, &route, &adjacency, |s| s.ends_with('Z')) as u128)
                .collect();

            let max = *route_lengths.iter().max().unwrap();
            let mut current_max = max;
            while !route_lengths.iter().all(|n| current_max % *n == 0) {
                current_max += max;
            }
            current_max
        })
    )
}
