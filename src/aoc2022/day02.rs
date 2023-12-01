use crate::utils::{print_type_of, read_file22};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl RPS {
    fn value(&self) -> i32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn vs(&self, other: &Self) -> Outcome {
        if self == other {
            return Outcome::Draw;
        }

        match (self, other) {
            (RPS::Scissors, RPS::Paper) | (RPS::Paper, RPS::Rock) | (RPS::Rock, RPS::Scissors) => {
                Outcome::Win
            }
            _ => Outcome::Lose,
        }
    }

    fn what_to_play_to_get_desired_outcome(&self, outcome: &Outcome) -> RPS {
        for rps in [RPS::Scissors, RPS::Paper, RPS::Rock] {
            if rps.vs(self) == *outcome {
                return rps;
            }
        }
        unreachable!()
    }
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

// fn parse_file(fname: &str) -> Vec<(&str, &str)> {
fn parse_file(fname: &str) -> Vec<Vec<char>> {
    let binding = read_file22(fname);
    let t: Vec<_> = binding
        .iter()
        .map(|v| {
            v.split_whitespace()
                .map(|c| c.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    t
}
pub fn main() -> (i32, i32) {
    (part1(), part2())
}

pub fn part1() -> i32 {
    // read_file22("02.txt");
    let fname = "02.txt";
    let rounds = parse_file(&fname);
    let selection = _create_selection_map();

    let mut res = 0;
    for pair in rounds {
        let l = &selection[&pair[0]];
        let r = &selection[&pair[1]];

        res += r.vs(l).score() + r.value();
    }
    res
}

pub fn part2() -> i32 {
    // read_file22("02.txt");
    let fname = "02.txt";
    let rounds = parse_file(&fname);
    let selection = _create_selection_map();
    let desired_outcome = _create_outcome_map();

    let mut res = 0;
    for pair in rounds {
        let l = &selection[&pair[0]];
        let o = &desired_outcome[&pair[1]];
        let r = l.what_to_play_to_get_desired_outcome(&o);

        res += r.vs(l).score() + r.value();
    }
    res
}

fn _create_selection_map() -> HashMap<char, RPS> {
    let mut selection: HashMap<char, RPS> = HashMap::new();
    selection.insert('A', RPS::Rock);
    selection.insert('B', RPS::Paper);
    selection.insert('C', RPS::Scissors);
    selection.insert('X', RPS::Rock);
    selection.insert('Y', RPS::Paper);
    selection.insert('Z', RPS::Scissors);
    selection
}

fn _create_outcome_map() -> HashMap<char, Outcome> {
    let mut outcome: HashMap<char, Outcome> = HashMap::new();
    outcome.insert('X', Outcome::Lose);
    outcome.insert('Y', Outcome::Draw);
    outcome.insert('Z', Outcome::Win);
    outcome

}
