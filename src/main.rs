use std::io::{stdout, BufWriter};

use ferris_says::say;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello Jasper!");
    let width = message.chars().count();

    let writer = BufWriter::new(stdout.lock());
    let _ = say(&message, width, writer);
}
