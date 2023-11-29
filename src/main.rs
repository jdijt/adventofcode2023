use std::io::{stdout, BufWriter};
use ferris_says::say;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(last=true)]
    assignment: String
}

fn main() {
    let args = Args::parse();
    let stdout = stdout();
    let mut message = String::from("Hello ");
    message.push_str(&args.assignment);

    let width = message.chars().count();

    let writer = BufWriter::new(stdout.lock());
    let _ = say(&message, width, writer);
}
