use std::collections::HashMap;

use nom::{
    self,
    bytes::complete::{tag, take_till, take_until},
    Err, IResult,
};

use crate::{print1, utils::read_file23};

pub fn main() -> (u32, u32) {
    (part1("02.txt"), part2("02.txt"))
}

fn part1(path: &str) -> u32 {
    let max_reveal = Reveal {
        red: 12,
        green: 13,
        blue: 14,
    };
    _get_data(path)
    .iter()
    .filter_map(|g| {
        match g.is_valid(&max_reveal) {
            true => Some(g.id),
            false => None
        }
    })
    .sum()
}

fn part2(path: &str) -> u32 {
    _get_data(path)
        .iter()
        .map(|g| g.bounding_reveal().power())
        .sum()
}

fn _get_data(path: &str) -> Vec<Game> {
    read_file23(path)
        .iter()
        .map(|line| _parse_line(line.as_str()).unwrap().1)
        .collect()
}

/// ============================================================================
/// STRUCTS
/// ============================================================================
#[derive(Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

#[derive(Debug, Clone)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn is_valid(&self, max_reveal: &Reveal) -> bool {
        self.reveals.iter().all(|r| r.is_valid(max_reveal))
    }

    fn bounding_reveal(&self) -> Reveal {
        let mut res = Reveal { red: 0, green: 0, blue: 0 };
        for g in self.reveals.iter() {
            res = res.update_max_of(g);
        }
        res

        // TODO why not this?
        // error:
        //  cannot borrow `*acc` as mutable, as it is behind a `&` reference
        //  `acc` is a `&` reference, so the data it refers to cannot be borrowed as mutable
        // let res = Reveal { red: 0, green: 0, blue: 0 };
        // self.reveals
        // .iter_mut()
        // .fold(&res, |acc, cur| acc.update_max_of(cur));  // ERROR
        // return res;

        // TODO why not this?
        // error:
        //  mismatched types
        //  expected `&Reveal`, found `Reveal`
        // self.reveals
        // .iter()
        // .reduce(|acc, cur| acc.max_of(cur))  // ERROR
        // .unwrap()
        // .clone()
     }
}

impl Reveal {
    fn is_valid(&self, max_reveal: &Self) -> bool {
        self.red <= max_reveal.red && self.green <= max_reveal.green && self.blue <= max_reveal.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    /// get the min number of cubes to support both reveals
    fn max_of(&self, other: &Self) -> Reveal {
        Reveal {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
    fn update_max_of(mut self, other: &Self) -> Self {
        self.red = self.red.max(other.red);
        self.green = self.green.max(other.green);
        self.blue = self.blue.max(other.blue);
        self
    }
}

/// ============================================================================
/// PARSER
/// playing around with parser combinators then quickly switching
/// to normal text parsing
/// ============================================================================
fn _parse_line(input: &str) -> IResult<&str, Game> {
    let (input, id) = _parse_game_id(input)?;
    let mut reveals: Vec<Reveal> = Vec::new();

    for r in input.split("; ") {
        let mut map: HashMap<&str, u32> = HashMap::new();
        for cube_count in r.split(", ") {
            let (count, color) = _parse_reveal(cube_count);
            map.insert(color, count);
        }
        reveals.push(Reveal {
            red: *map.get("red").unwrap_or(&0),
            green: *map.get("green").unwrap_or(&0),
            blue: *map.get("blue").unwrap_or(&0),
        })
    }
    Ok((input, Game { id, reveals }))
}

/// parser combinator here
fn _parse_game_id(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = take_until(":")(input)?;
    let (input, _) = _skip_to_next_int(input)?;
    Ok((input, id.parse::<u32>().expect("an integer")))
}

fn _parse_reveal(cube_count: &str) -> (u32, &str) {
    let mut it = cube_count.split(' ');
    (
        it.next().unwrap().parse::<u32>().unwrap(),
        it.next().unwrap(),
    )
}

fn _skip_to_next_int(input: &str) -> IResult<&str, &str> {
    let (input, _) = take_till(|v: char| v.is_ascii_digit())(input)?;
    Ok((input, ""))
}
