use std::{fs, thread::sleep, time::Duration};

pub fn run() {
    let file_contents = fs::read_to_string("./inputs/day1").unwrap();
    let seconds = file_contents.parse::<u64>().unwrap();
    sleep(Duration::from_secs(seconds))
}
