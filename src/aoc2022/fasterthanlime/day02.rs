use color_eyre::eyre::Context;
use std::{fs, str::FromStr};

static BASE_PATH: &str = "/Users/acushner/software/rust/aoc/input/2022/";

fn file_path(fname: &str) -> String {
    BASE_PATH.to_owned() + fname
}

pub fn day02() -> color_eyre::Result<()> {
    // color_eyre::install()?;
    let data = fs::read_to_string(file_path("02.txt.test")).expect("valid input");
    // Ok(data)
    let res: usize = data
        .lines()
        .map(Round::from_str)
        .map(|round| round.expect("valid round").our_score())
        .sum();
    dbg!(res);
    Ok(())
}

pub fn day02_2() -> color_eyre::Result<()> {
    // color_eyre::install()?;
    let data = fs::read_to_string(file_path("02.txt.test")).expect("valid input");
    // Ok(data)
    let res: usize = data
        .lines()
        .map(|line| line.parse::<Round>())
        .map(|round| round.expect("valid round").our_score())
        .sum();
    dbg!(res);
    Ok(())
}

pub fn day02_1() -> color_eyre::Result<()> {
    // color_eyre::install()?;
    for round in fs::read_to_string(file_path("02.txt.test"))?
        .lines()
        .map(|line| line.parse::<Round>())
    {
        let round = round?;
        println!(
            "{round:?}: outcome={outcome:?}, score={score}",
            outcome = round.outcome(),
            score = round.our_score()
        );
    }

    Ok(())
}

#[derive(Clone, Debug, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Debug, Copy)]
struct Round {
    their: Move,
    our: Move,
}

#[derive(Clone, Debug, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("not a valid move: {c:?}")),
        }
    }
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(their), Some(' '), Some(our), None) = (chars.next(), chars.next(),chars.next(),chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <their>SP<our>EOF, got {s:?}"));
        };

        Ok(Self {
            their: their.try_into()?,
            our: our.try_into()?,
        })
    }
}

impl Move {
    fn points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Scissors, Self::Paper)
                | (Self::Paper, Self::Rock)
        )
    }

    fn outcome(self, their: Move) -> Outcome {
        if self.beats(their) {
            Outcome::Win
        } else if their.beats(self) {
            Outcome::Lose
        } else {
            Outcome::Draw
        }
    }
}

impl Outcome {
    fn points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Lose => 0,
            Outcome::Draw => 3,
        }
    }
}

impl Round {
    fn outcome(self) -> Outcome {
        self.our.outcome(self.their)
    }

    fn our_score(self) -> usize {
        self.our.points() + self.outcome().points()
    }
}

fn read_color_eyre_input() -> color_eyre::Result<String> {
    let path = "not/here.txt";
    let input = std::fs::read_to_string(path).wrap_err("reading ".to_owned() + path)?;
    Ok(input)
}
