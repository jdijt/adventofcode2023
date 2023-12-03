use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::{Duration, Instant};

pub struct TimedResult<T> {
    res: T,
    time: Duration,
}

impl<T: Display> Display for TimedResult<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Result: {}, time: {} nanos", self.res, self.time.as_nanos())
    }
}

pub fn run_timed<T: Display, F: Fn() -> T>(func: F) -> TimedResult<T> {
    let now = Instant::now();
    let res = func();
    let time = now.elapsed();

    TimedResult { res, time }
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
