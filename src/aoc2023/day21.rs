use std::collections::{HashMap, HashSet, VecDeque};

use strum::IntoEnumIterator;

use crate::{
    point::{Dir, Point, point_inclusive_mod},
    tprint,
    utils::read_file23,
};

pub type AocRes = Result<u32, String>;
type IsValid = fn(&Garden, &Point) -> bool;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let is_valid = |garden: &Garden, p: &Point| -> bool {
        garden.plots.get(&PlotType::Open).unwrap().contains(p) || *p == garden.start
    };

    let garden = _get_data("21.txt", is_valid);
    let res = garden.walk(64u32);
    Ok(res.values().filter(|v| **v & 1 == 0).count() as u32)
}

fn part2() -> AocRes {
    let is_valid = |garden: &Garden, p: &Point| -> bool {
        let p = &point_inclusive_mod(p, &garden.lower_right);
        garden.plots.get(&PlotType::Open).unwrap().contains(p) || *p == garden.start
    };
    let garden = _get_data("21.txt.test", is_valid);
    let res = garden.walk(100u32);
    Ok(res.values().filter(|v| **v & 1 == 0).count() as u32)
    // Err("unsolved".to_string())
}

fn _get_data(fname: &str, is_valid: IsValid) -> Garden {
    Garden::from_str(read_file23(fname).join("\n"), is_valid)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum PlotType {
    Open,
    Rock,
    Start,
}

impl PlotType {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Rock,
            'S' => Self::Start,
            _ => panic!("unexpected plot char {c}"),
        }
    }
}

#[derive(Debug)]
struct Garden {
    plots: HashMap<PlotType, HashSet<Point>>,
    start: Point,
    lower_right: Point,
    is_valid: IsValid,
}

impl Garden {
    fn from_str(s: impl AsRef<str>, is_valid: IsValid) -> Self {
        let mut plots: HashMap<PlotType, HashSet<Point>> = hashmap! {};
        let mut start: Option<Point> = None;
        let mut max_y = 0;
        let mut max_x = 0;

        s.as_ref().split('\n').enumerate().for_each(|(y, line)| {
            max_y = max_y.max(y);
            line.chars().enumerate().for_each(|(x, c)| {
                max_x = max_x.max(x);
                let plot_type = PlotType::from_char(c);
                if plot_type == PlotType::Start {
                    start = Some(Point::new(x as i32, y as i32));
                }
                plots
                    .entry(plot_type)
                    .or_default()
                    .insert(Point::new(x as i32, y as i32));
            });
        });

        Self {
            plots,
            start: start.unwrap(),
            lower_right: Point::new(max_x as i32, max_y as i32),
            is_valid,
        }
    }

    fn walk(&self, max_count: u32) -> HashMap<Point, u32> {
        let mut res: HashMap<Point, u32> = hashmap! {};
        let mut to_process = VecDeque::from([(0u32, self.start)]);
        while let Some((count, point)) = to_process.pop_front() {
            if !(self.is_valid)(self, &point) || count > max_count {
                continue;
            }
            let mut extend = true;
            let mut insert = true;
            if let Some(v) = res.get(&point) {
                // if cur val is odd and count is also odd
                (extend, insert) = match v & 1 == 1 && count & 1 == 1 {
                    true => (false, false),
                    false => (true, true),
                }
            }
            if insert {
                res.insert(point, count);
            }
            if extend {
                to_process.extend(Dir::iter().map(|d| (count + 1, point + d)));
            }
        }
        res
    }
}
