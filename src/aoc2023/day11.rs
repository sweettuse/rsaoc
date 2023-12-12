use std::collections::{HashMap, HashSet};

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
    // tprint!(universe.get_all_occupied());
    let res: i64 = universe.calc_pairs().iter().map(|(p1, p2)| universe.calc_distance(*p1, *p2)).sum();
    Ok(res)
}

fn _get_data(fname: &str, expansion_coefficient: i64) -> Universe {
    let lines = read_file23(fname);
    Universe::from_str(lines.join("\n"), expansion_coefficient)
}

// =============================================================================
// STRUCTs/ENUMs
// =============================================================================

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Object {
    Galaxy,
    Space,
}

#[derive(Debug)]
struct Universe {
    umap: HashMap<Point, Object>,
    x_expansion: Expansion,
    y_expansion: Expansion,
}

#[derive(Debug)]
struct Expansion(Vec<i64>);

#[derive(Debug)]
struct Cell {}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

// =============================================================================
// IMPLs
// =============================================================================

impl Universe {
    fn calc_distance(&self, p1: Point, p2: Point) -> i64 {
        (p2.x - p1.x).abs()
            + (p2.y - p1.y).abs()
            + self.x_expansion.expand(p1.x, p2.x)
            + self.y_expansion.expand(p1.y, p2.y)
    }

    fn calc_pairs(&self) -> Vec<(Point, Point)> {
        let occupied = self.get_all_occupied();
        let mut res = vec![];
        for i in 0..occupied.len() {
            for j in i+1..occupied.len() {
                res.push((occupied[i], occupied[j]));
            }
        }
        res
    }

    fn get_all_occupied(&self) -> Vec<Point> {
        self.umap.iter().filter_map(|(p, obj)| {
            match obj {
                Object::Galaxy=> Some(*p),
                _ => None,
            }
        }).collect()
    }
}

impl Expansion {
    fn expand(&self, start: i64, end: i64) -> i64 {
        let (start, end) = (start.min(end) as usize, start.max(end) as usize);
        self.0[end] - self.0[start]
    }
}

impl Object {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("couldn't match char {c}"),
        }
    }
}

/// constructor
impl Universe {
    fn from_str(s: impl AsRef<str>, expansion_coefficient: i64) -> Self {
        let mut umap = HashMap::new();
        let mut xmax = 0;
        let mut ymax = 0;
        let mut occupied_xs: HashSet<i64> = HashSet::new();
        let mut occupied_ys: HashSet<i64> = HashSet::new();

        s.as_ref().split('\n').enumerate().for_each(|(y, line)| {
            ymax = y as i64;
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = Object::from_char(c);
                let (x, y) = (x as i64, y as i64);
                umap.insert(Point { x, y }, obj);
                if obj == Object::Galaxy {
                    occupied_xs.insert(x);
                    occupied_ys.insert(y);
                }
                xmax = x;
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
            umap,
            x_expansion,
            y_expansion,
        }
    }
}
