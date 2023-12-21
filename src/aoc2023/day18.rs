use std::{
    collections::HashSet,
    fmt::{Debug, Display},
    iter::repeat,
};

use itertools::{chain, Itertools};
use strum::IntoEnumIterator;

use crate::{
    point::{Dir, Point},
    tprint,
    utils::read_file23,
};

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let mut plan = _get_data("18.txt");
    plan.execute();
    plan.excavate();
    Ok(plan.dug.len() as u32)
    // Err("unsolved".to_string())
}

fn part2() -> AocRes {
    Err("unsolved".to_string())
}

fn _get_data(fname: &str) -> DigPlan {
    DigPlan::from_str(read_file23(fname).join("\n"))
}

// #[derive(Debug)]
struct Instruction {
    dir: Dir,
    num: u32,
    color: u32,
}

struct DigPlan {
    instructions: Vec<Instruction>,
    dug: HashSet<Point>,
    min_point: Option<Point>,
    max_point: Option<Point>,
}

impl DigPlan {
    fn from_str(s: impl AsRef<str>) -> Self {
        Self {
            instructions: s
                .as_ref()
                .split('\n')
                .map(Instruction::from_str)
                .collect_vec(),
            dug: HashSet::default(),
            min_point: None,
            max_point: None,
        }
    }

    // DIG
    fn execute(&mut self) {
        self._populate_instructions();
        self._populate_min_max();
        self._normalize();
    }

    fn excavate(&mut self) {
        let max = self.max_point.unwrap();
        let min = self.min_point.unwrap();
        let mut all_points = (min.x..=max.x)
            .cartesian_product(min.y..=max.y)
            .map(|(x, y)| Point::new(x, y))
            .collect::<HashSet<_>>()
            .difference(&self.dug)
            .copied()
            .collect::<HashSet<_>>();

        while !all_points.is_empty() {
            let (is_inside, found) = self._flood_fill(all_points.iter().next().unwrap());
            all_points = all_points.difference(&found).copied().collect();
            if is_inside {
                self.dug.extend(found);
            }
        }
    }

    // fill in adjacent cells and report if they are inside or outside
    fn _flood_fill(&self, start: &Point) -> (bool, HashSet<Point>) {
        let mut is_inside = true;
        let mut stack = vec![*start];
        let mut res = HashSet::<Point>::new();
        let mut seen = HashSet::<Point>::new();
        let diags = [
            Dir::North + Dir::East,
            Dir::North + Dir::West,
            Dir::South + Dir::East,
            Dir::South + Dir::West,
        ];
        let dir_offsets = Dir::iter().map(|d| d.offset()).collect_vec();
        while let Some(cur) = stack.pop() {
            if seen.contains(&cur) {
                continue;
            }

            seen.insert(cur);

            if !(self._valid_point(&cur)) {
                is_inside = false;
                continue;
            }
            if self.dug.contains(&cur) {
                continue;
            }

            stack.extend(chain(dir_offsets.iter(), &diags).map(|p| cur + *p));
            res.insert(cur);
        }
        (is_inside, res)
    }

    fn _valid_point(&self, p: &Point) -> bool {
        let max = self.max_point.unwrap();
        let min = self.min_point.unwrap();
        !(p.x < min.x || p.y < min.y || p.x > max.x || p.y > max.y)
    }

    fn _populate_instructions(&mut self) {
        self.dug.insert(Point::new(0, 0));
        self.dug.extend(
            self.instructions
                .iter()
                .flat_map(|inst| repeat(inst.dir).take(inst.num as usize))
                .scan(Point::new(0, 0), |acc, dir| {
                    *acc = *acc + dir;
                    Some(*acc)
                }),
        );
    }

    fn _populate_min_max(&mut self) {
        let mut min_point = Point::new(i32::MAX, i32::MAX);
        let mut max_point = Point::new(i32::MIN, i32::MIN);
        self.dug.iter().for_each(|p| {
            min_point.x = min_point.x.min(p.x);
            min_point.y = min_point.y.min(p.y);
            max_point.x = max_point.x.max(p.x);
            max_point.y = max_point.y.max(p.y);
        });
        self.min_point = Some(min_point);
        self.max_point = Some(max_point);
    }

    fn _normalize(&mut self) {
        if let Some(min_point) = self.min_point {
            if self.min_point.unwrap() == Point::new(0, 0) {
                return;
            }
            self.dug = self.dug.iter().map(|p| *p - min_point).collect();
            self.max_point = Some(self.max_point.unwrap() - min_point);
            self.min_point = Some(Point::new(0, 0));
        }
    }
}

impl Instruction {
    fn from_str(s: impl AsRef<str>) -> Self {
        let (udlr, num_str, color_str) = s.as_ref().split(' ').collect_tuple().unwrap();
        let color_str = color_str.replace(['#', '(', ')'], "");
        Self {
            dir: Dir::from_udlr(udlr),
            num: num_str.parse::<u32>().unwrap(),
            color: u32::from_str_radix(&color_str, 16).unwrap(),
        }
    }
}

impl Dir {
    fn from_udlr(c: &str) -> Self {
        match c {
            "U" => Dir::North,
            "D" => Dir::South,
            "L" => Dir::West,
            "R" => Dir::East,
            _ => panic!("unmatched {c}"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Instruction {{ dir: {:?}, num: {}, color: #{:x} }}",
            self.dir, self.num, self.color,
        )
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

#[cfg(test)]
mod test {
    use crate::tprint;

    use super::*;

    #[test]
    fn test_inst() {
        println!("{}", Instruction::from_str("R 6 (#70c710)"));
        tprint!(Instruction::from_str("R 6 (#70c710)"));
    }

    #[test]
    fn test_test() {
        let mut plan = _get_data("18.txt.test");
        plan.execute();
        assert_eq!(plan.dug.len(), 38);
    }

    #[test]
    fn test_test_full() {
        let mut plan = _get_data("18.txt.test");
        plan.execute();
        plan.excavate();
        assert_eq!(plan.dug.len(), 62);
    }
}
