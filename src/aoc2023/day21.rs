use std::collections::{HashMap, HashSet, VecDeque};

use strum::IntoEnumIterator;

use crate::{utils::read_file23, point::{Point, Dir}, tprint};

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let garden = _get_data("21.txt");
    let res =garden.walk(64u32);
    tprint!(res.values().filter(|v| **v & 1 == 0).count());
    Err("unsolved".to_string())
}

fn part2() -> AocRes {
    Err("unsolved".to_string())
}

fn _get_data(fname: &str) -> Garden {
    Garden::from_str(read_file23(fname).join("\n"))
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
}

impl Garden {
    fn from_str(s: impl AsRef<str>) -> Self {
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
                plots.entry(plot_type).or_default().insert(Point::new(x as i32, y as i32));
            });
        });

        Self { plots , start: start.unwrap(), lower_right: Point::new(max_x as i32, max_y as i32)}
    }

    fn _is_valid(&self, p: &Point) -> bool {
        self.plots.get(&PlotType::Open).unwrap().contains(p) || *p == self.start
    }

    fn walk(&self, max_count: u32) -> HashMap<Point, u32>{
        let mut res: HashMap<Point, u32> = hashmap! {};
        let mut to_process = VecDeque::from([(0u32, self.start)]);
        while let Some((count, point)) = to_process.pop_front() {
            if !self._is_valid(&point) { 
                continue;
            }
            if res.contains_key(&point) {
                continue;
            }
            if count > max_count {
                continue;
            }

            res.insert(point, count);
            to_process.extend(Dir::iter().map(|d| {
                (count + 1, point + d)
            }));
        }
        res
    }
}

