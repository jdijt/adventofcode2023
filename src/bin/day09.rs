use aoc2023::{read_lines, run_timed};

fn derive(input: &Vec<i64>) -> Vec<i64> {
    input
        .windows(2)
        .filter_map(|i| match i {
            [l, r] => Some(*r - *l),
            _ => None,
        })
        .collect()
}

fn extrapolate(input: &Vec<i64>) -> i64 {
    if input.iter().all(|e| *e == 0) {
        0
    } else {
        let next = derive(input);
        let next_diff = extrapolate(&next);

        if let Some(e) = input.last() {
            *e + next_diff
        } else {
            panic!("Extrapolate called with empty collection!");
        }
    }
}

fn main() {
    let sequences: Vec<Vec<i64>> = read_lines("./inputs/day09")
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    println!(
        "Part 1: {}",
        run_timed(|| { sequences.iter().map(extrapolate).sum::<i64>() })
    );
    println!(
        "Part 2: {}",
        run_timed(|| {
            let reversed: Vec<Vec<i64>> = sequences
                .iter()
                .map(|s| {
                    let mut new_s = s.clone();
                    new_s.reverse();
                    new_s
                })
                .collect();

            reversed.iter().map(extrapolate).sum::<i64>()
        })
    );
}
