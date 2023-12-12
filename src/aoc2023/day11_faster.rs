use std::{collections::{HashMap, HashSet}, time::Instant};

use crate::{tprint, utils::read_file23};

pub type AocRes = Result<i64, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    _both("11.txt", 2)
}

fn part2() -> AocRes {
    _both("11.txt", 1_000_000)
}

fn _both(fname: &str, expansion_coefficient: i64) -> AocRes {
    let universe = _get_data(fname, expansion_coefficient);
    Ok(universe
        .calc_pairs()
        .iter()
        .map(|(p1, p2)| universe.calc_distance(*p1, *p2))
        .sum())
}

fn _get_data(fname: &str, expansion_coefficient: i64) -> Universe {
    let lines = read_file23(fname);
    Universe::from_str(lines.join("\n"), expansion_coefficient)
}

// =============================================================================
// STRUCTs/ENUMs
// =============================================================================

#[derive(Debug)]
struct Universe {
    occupied: Vec<(i64, i64)>,
    x_expansion: Expansion,
    y_expansion: Expansion,
}

#[derive(Debug)]
struct Expansion(Vec<i64>);

type Point = (i64, i64);


// =============================================================================
// IMPLs
// =============================================================================

impl Universe {
    fn calc_distance(&self, p1: Point, p2: Point) -> i64 {
        (p2.0 - p1.0).abs()
            + (p2.1 - p1.1).abs()
            + self.x_expansion.expand(p1.0, p2.0)
            + self.y_expansion.expand(p1.1, p2.1)
    }

    fn calc_pairs(&self) -> Vec<(Point, Point)> {
        let mut res = vec![];
        for i in 0..self.occupied.len() {
            for j in i + 1..self.occupied.len() {
                res.push((self.occupied[i], self.occupied[j]));
            }
        }
        res
    }

}

impl Expansion {
    fn expand(&self, start: i64, end: i64) -> i64 {
        (self.0[end as usize] - self.0[start as usize]).abs()
    }
}

/// constructor
impl Universe {
    fn from_str(s: impl AsRef<str>, expansion_coefficient: i64) -> Self {
        let mut occupied = vec![];
        let mut xmax = 0;
        let mut ymax = 0;
        let mut occupied_xs: HashSet<i64> = HashSet::new();
        let mut occupied_ys: HashSet<i64> = HashSet::new();

        s.as_ref().split('\n').enumerate().for_each(|(y, line)| {
            ymax = y as i64;
            line.chars().enumerate().for_each(|(x, c)| {
                xmax = x as i64;
                if c != '#' {
                    return;
                }
                let (x, y) = (x as i64, y as i64);
                occupied.push((x, y));
                occupied_xs.insert(x);
                occupied_ys.insert(y);
            })
        });

        // get the cumulative sum of the expansion
        let calculate_expansion = |max, occupied: HashSet<i64>| {
            let mut cum_sum = 0;
            let res = (0..=max)
                .map(|v| {
                    cum_sum += match occupied.contains(&v) {
                        true => 0,
                        false => expansion_coefficient - 1,
                    };
                    cum_sum
                })
                .collect();
            Expansion(res)
        };

        let x_expansion = calculate_expansion(xmax, occupied_xs);
        let y_expansion = calculate_expansion(ymax, occupied_ys);

        Self {
            occupied,
            x_expansion,
            y_expansion,
        }
    }
}
