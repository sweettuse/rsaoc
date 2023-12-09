use once_cell::sync::Lazy;

use crate::{utils::read_file23, print1};

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    _get_data("07.txt.test");
    Err("unsolved".to_string())
}

fn part2() -> AocRes {
    Err("unsolved".to_string())
}

fn _get_data(fname: &str) -> Vec<Hand> {
    let lines = read_file23(fname);
    lines.iter().map(Hand::from_str).collect()

}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u64,
}

trait FromHand {
    fn from_hand(hand: &Hand) -> HandType;
}

impl FromHand for HandType {
    fn from_hand(hand: &Hand) -> HandType {
        todo!()
    }
}

impl Hand {
    fn from_str(s: impl AsRef<str>) -> Self {
        let (cards, bid) = s.as_ref().split_once(' ').unwrap();
        Self {
            cards: cards.to_string(),
            bid: bid.parse().unwrap(),
        }
    }
}
