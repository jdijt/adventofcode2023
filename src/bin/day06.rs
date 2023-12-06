use aoc2023::run_timed;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn win_options_count(&self) -> u64 {
        let is_win = |press_ms: &u64| press_ms * (self.time - press_ms) > self.distance;

        let first_win = (0..self.time).find(is_win).unwrap();
        let last_win = (0..self.time).rfind(is_win).unwrap() + 1; //we want the first not winning for diff to be correct.

        last_win - first_win
    }
}

fn main() {
    let input1: Vec<Race> = vec![
        Race {
            time: 53,
            distance: 313,
        },
        Race {
            time: 89,
            distance: 1090,
        },
        Race {
            time: 76,
            distance: 1214,
        },
        Race {
            time: 98,
            distance: 1201,
        },
    ];

    let input2 = Race {
        time: 53897698,
        distance: 313109012141201,
    };

    println!(
        "Part 1: {}",
        run_timed(|| input1
            .iter()
            .map(|r| r.win_options_count())
            .product::<u64>())
    );
    println!("Part 2: {}", run_timed(|| input2.win_options_count()));
}
