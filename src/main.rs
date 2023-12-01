use std::time::Instant;
use clap::Parser;

mod day01;
mod advent_util;

#[derive(Parser)]
struct Args {
    #[arg(last = true)]
    assignment: String,
}

fn run_timed(func: fn()) {
    println!("Starting\n");
    let now= Instant::now();
    func();
    let elapsed = now.elapsed();
    println!("\nDone!");
    println!("Total time: {} micros", elapsed.as_micros())
}

fn main() {
    let args = Args::parse();

    match args.assignment.as_str() {
        "1" => run_timed(day01::run),
        o => println!("Unknown assignment {}", o),
    }
}
