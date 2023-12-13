use aoc2023::{read_lines, run_timed};
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    id: usize,
    winning_number_count: usize,
    score: usize,
}

impl Card {
    fn from(line: &str) -> Card {
        let (card_identifier, numbers) = line.split_once(":").unwrap();
        let id = card_identifier
            .strip_prefix("Card")
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        fn split_to_numbers<I, C>(line: &str) -> C
        where
            I: FromStr,
            <I as FromStr>::Err: Debug,
            C: FromIterator<I>,
        {
            line.trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        }

        let (winning_numbers, numbers) = numbers
            .trim()
            .split_once("|")
            .map(|(wn_line, n_line)| {
                (
                    split_to_numbers::<i32, HashSet<i32>>(wn_line),
                    split_to_numbers::<i32, Vec<i32>>(n_line),
                )
            })
            .unwrap();

        let winning_number_count = numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();

        let score = if winning_number_count >= 1 {
            1 << (winning_number_count - 1)
        } else {
            0
        };

        return Card {
            id,
            winning_number_count,
            score,
        };
    }
}

fn part2(cards: &Vec<Card>) -> u32 {
    let mut additional_cards: Vec<u32> = Vec::with_capacity(cards.len());
    for card in cards {
        additional_cards.insert(card.id - 1, 1);
    }

    for card in cards {
        let id = card.id;
        let count = *additional_cards.get(id - 1).unwrap();
        let card_score = card.winning_number_count;
        for idx in id..(id + card_score) {
            additional_cards.get_mut(idx).unwrap().add_assign(count);
        }
    }

    return additional_cards.iter().sum();
}

fn main() {
    let cards: Vec<Card> = read_lines("./inputs/day04")
        .map(|l| Card::from(l.trim()))
        .collect();

    println!(
        "Part 1: {}",
        run_timed(|| cards.iter().map(|c| c.score).sum::<usize>())
    );
    println!("Part 2: {}", run_timed(|| part2(&cards)));
}
