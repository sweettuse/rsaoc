use cached::cached;
use std::cmp::Ordering;
use std::collections::HashMap;

use maplit::hashmap;
use strum_macros::EnumIter;

use crate::print1;
use crate::utils::read_file23;

type AocRes = Result<u64, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let mut cards = _get_data("07.txt");
    cards.sort();
    let res = cards
        .iter()
        .zip(1..)
        .map(|(c, m)| c.bid * m)
        .sum::<u64>();

    Ok(res)
}

fn _get_data(fname: &str) -> Vec<Hand> {
    let data = read_file23(fname);
    data.iter().map(Hand::from_str).collect()
}

fn part2() -> AocRes {
    Err("unsolved".to_string())
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl Hand {
    fn from_str(s: impl AsRef<str>) -> Self {
        let (cards, bid) = s.as_ref().split_once(' ').unwrap();
        Self {
            cards: cards.to_string(),
            bid: bid.parse().unwrap(),
        }
    }
    fn hand_type(&self) -> HandType {
        match self._count_cards().as_slice() {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            v => {
                if v.contains(&2) {
                    HandType::OnePair
                } else {
                    panic!("uh oh")
                }
            }
        }
        // todo!()
    }

    fn card_ranks(&self) -> Vec<u64> {
        let m = card_val_map();
        self.cards.chars().map(|c| *m.get(&c).unwrap()).collect()
    }

    fn _count_cards(&self) -> Vec<i32> {
        let mut res: HashMap<char, _> = HashMap::new();
        for c in self.cards.chars() {
            *res.entry(c).or_insert(0) += 1;
        }
        let mut v: Vec<_> = res.values().copied().collect();
        v.sort();
        v
    }
}

cached! {
    CARD_MAP;
    fn card_val_map() -> HashMap<char, u64> = {
        let cards = "23456789TJQKA";
        let mut res: HashMap<_, _> = HashMap::new();
        cards.chars().zip(0..).for_each(|(c, v)| { res.insert(c, v); });
        res
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => (),
            other => return other,
        };
        for (s, o) in self.card_ranks().iter().zip(other.card_ranks().iter()) {
            if s == o {
                continue;
            }
            return match s < o {
                true => Ordering::Less,
                false => Ordering::Greater,
            };
        }
        Ordering::Equal
    }
}
