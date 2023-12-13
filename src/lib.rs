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
        write!(
            f,
            "Result: {}, time: {} micros",
            self.res,
            self.time.as_micros()
        )
    }
}

pub fn run_timed<T: Display, F: Fn() -> T>(func: F) -> TimedResult<T> {
    let now = Instant::now();
    let res = func();
    let time = now.elapsed();

    TimedResult { res, time }
}

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<Path>,
{
    File::open(filename)
        .map(|f| io::BufReader::new(f).lines())
        .expect("Error opening input file")
        .map(|l| l.expect("Error reading line from input file"))
}
