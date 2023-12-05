use aoc2023::{read_lines, run_timed};
use std::cmp;

struct Game {
    id: i32,
    min_red: i32,
    min_green: i32,
    min_blue: i32,
}

impl Game {
    fn new(id: i32) -> Self {
        Game {
            id,
            min_blue: 0,
            min_red: 0,
            min_green: 0,
        }
    }

    fn add_red(&mut self, count: i32) {
        self.min_red = cmp::max(self.min_red, count)
    }
    fn add_green(&mut self, count: i32) {
        self.min_green = cmp::max(self.min_green, count)
    }
    fn add_blue(&mut self, count: i32) {
        self.min_blue = cmp::max(self.min_blue, count)
    }
    fn power(&self) -> i32 {
        self.min_blue * self.min_green * self.min_red
    }
}

fn parse_line(l: &String) -> Game {
    let (game_header, revelations) = l.split_once(":").unwrap();
    let game_id = game_header
        .split_once(" ")
        .map(|(_, num)| num.trim().parse::<i32>().unwrap())
        .unwrap();

    let mut game = Game::new(game_id);

    for revelation in revelations.split(';') {
        for count in revelation.trim().split(',') {
            let (c_s, color) = count.trim().split_once(" ").unwrap();
            let c_i = c_s.parse::<i32>().unwrap();

            match color {
                "blue" => game.add_blue(c_i),
                "green" => game.add_green(c_i),
                "red" => game.add_red(c_i),
                _other => panic!("Unknown color {}", color),
            }
        }
    }

    return game;
}

fn part1(games: &Vec<Game>) -> i32 {
    games
        .iter()
        .filter(|g| g.min_blue <= 14 && g.min_green <= 13 && g.min_red <= 12)
        .map(|g| g.id)
        .sum()
}

fn part2(games: &Vec<Game>) -> i32 {
    games.iter().map(|g| g.power()).sum()
}

fn main() {
    let games = read_lines("./inputs/day02")
        .map(|ls| ls.map(|l| parse_line(&l.unwrap())).collect::<Vec<Game>>())
        .unwrap();

    println!("Part 1: {}", run_timed(|| part1(&games)));
    println!("Part 2: {}", run_timed(|| part2(&games)));
}
