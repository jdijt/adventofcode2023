use aoc2023::run_timed;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn win_options_count(&self) -> u64 {
        let mut count = 0u64;
        for press_ms in 0..self.time{
            if press_ms * (self.time - press_ms) > self.distance {
                count += 1;
            }
        }
        return count;
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
