use crate::HandType::{
    FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPair,
};
use aoc2023::{read_lines, run_timed};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Card(char);

impl Card {
    fn from(c: char) -> Card {
        Card(c)
    }
    fn ordinal(&self) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            'Z' => 1, //joker
            o => o.to_digit(10).unwrap() as u8,
        }
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordinal().cmp(&other.ordinal())
    }
}

// Discriminants start at 0, so the "best" hand should come last.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Hand {
    kind: HandType,
    cards: Vec<Card>,
    bid: u64,
}

impl Hand {
    fn from(l: String) -> Hand {
        let (h, b) = l.split_once(" ").unwrap();
        let bid = b.parse().unwrap();
        let cards: Vec<Card> = h.chars().map(Card::from).collect();

        let mut card_counts: HashMap<Card, i32> = HashMap::new();
        for c in cards.iter() {
            if !card_counts.contains_key(c) {
                card_counts.insert(c.clone(), 0);
            }
            card_counts.get_mut(c).into_iter().for_each(|c| *c += 1)
        }

        let jokers = match card_counts.get(&Card('Z')) {
            Some(count) => *count,
            _ => 0,
        };

        let mut counts: Vec<i32> = card_counts.into_values().collect();
        counts.sort();
        counts.reverse();

        let kind = match (counts.as_slice(), jokers) {
            ([5], 0 | 5) | ([4, 1], 4 | 1) | ([3, 2], 2 | 3) => FiveOfAKind,
            ([4, 1], 0) | ([2, 2, 1], 2) | ([3, 1, 1], 1 | 3) => FourOfAKind,
            ([3, 2], 0) | ([2, 2, 1], 1) => FullHouse,
            ([3, 1, 1], 0) | ([2, 1, 1, 1], 1 | 2) => ThreeOfAKind,
            ([2, 2, 1], 0) => TwoPair,
            ([2, 1, 1, 1], 0) | ([1, 1, 1, 1, 1], 1) => OnePair,
            ([1, 1, 1, 1, 1], 0) => HighCard,
            o => panic!("Invalid hand! {:?}", o),
        };

        Hand { kind, cards, bid }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.kind != other.kind {
            self.kind.cmp(&other.kind)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

fn main() {
    let hands = read_lines("./inputs/day07")
        .unwrap()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();

    println!(
        "Part 1: {}",
        run_timed(|| {
            let mut hs: Vec<Hand> = hands.iter().cloned().map(Hand::from).collect();
            hs.sort();

            hs.into_iter()
                .enumerate()
                .map(|(idx, h)| h.bid * (idx + 1) as u64)
                .sum::<u64>()
        })
    );
    println!(
        "Part 2: {}",
        run_timed(|| {
            let mut hs: Vec<Hand> = hands
                .iter()
                .cloned()
                .map(|s| Hand::from(s.replace('J', "Z")))
                .collect();
            hs.sort();
            hs.into_iter()
                .enumerate()
                .map(|(idx, h)| h.bid * (idx + 1) as u64)
                .sum::<u64>()
        })
    );
}
