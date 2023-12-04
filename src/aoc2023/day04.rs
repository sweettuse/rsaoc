use crate::{print1, utils::read_file23};
use std::collections::{HashMap, HashSet};

pub fn main() -> (u64, u64) {
    (part1("04.txt"), part2("04.txt"))
}

fn part1(fname: &str) -> u64 {
    _get_data(fname).iter().map(|c| c.value()).sum()
}

fn part2(fname: &str) -> u64 {
    let cards = _get_data(fname);
    let mut counts = Counter::from_cards(&cards);
    for c in cards {
        counts.update(&c);
    }
    counts.0.values().sum()
}

#[derive(Debug, Default)]
struct Counter(HashMap<u64, u64>);

impl Counter {
    fn from_cards(cards: &Vec<Card>) -> Self {
        let mut res = Counter::default();
        for c in cards {
            res.0.insert(c.id, 1);
        }
        res
    }

    fn update(&mut self, card: &Card) {
        let start_id = card.id + 1;
        let cur_count = *self.0.get(&card.id).unwrap();

        for id in start_id..start_id + card.num_matching {
            if let Some(count) = self.0.get_mut(&id) {
                *count += cur_count;
            } else {
                break;
            }
        }
    }
}

fn _get_data(fname: &str) -> Vec<Card> {
    read_file23(fname)
        .iter()
        .map(|s| Card::from_str(s.as_str()))
        .collect()
}

fn _str_to_numbers(s: &str) -> HashSet<u64> {
    s.split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

#[derive(Debug)]
struct Card {
    id: u64,
    numbers: HashSet<u64>,
    winning: HashSet<u64>,
    num_matching: u64,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let (card_str, rest) = s.split_once(':').unwrap();
        let (my_nums, winning_nums) = rest.split_once('|').unwrap();
        let numbers = _str_to_numbers(my_nums);
        let winning = _str_to_numbers(winning_nums);
        let num_matching = numbers.intersection(&winning).count() as u64;
        Card {
            id: *_str_to_numbers(card_str).iter().next().unwrap(),
            numbers,
            winning,
            num_matching,
        }
    }

    fn value(&self) -> u64 {
        if self.num_matching == 0 {
            return 0;
        }
        2u64.pow((self.num_matching - 1) as u32)
    }
}
