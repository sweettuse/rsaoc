use std::{
    collections::HashMap,
    error::Error,
    ops::{Range, RangeFrom},
};

use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::{tprint, utils::read_file23};

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    Ok(_get_data("15.txt").split(',').map(_hash).sum())
}

fn part2() -> AocRes {
    let mut boxes = (1u32..=256).map(LensBox::new).collect_vec();
    _get_data("15.txt").split(',').for_each(|s| {
        let cmd = Command::from_str(s);
        cmd.execute(&mut boxes);
    });
    Ok(boxes.iter().map(LensBox::calc_focusing_power).sum())
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Op {
    Add,
    Remove,
}

impl Op {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '=' => Some(Self::Add),
            '-' => Some(Self::Remove),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Self::Add => '=',
            Self::Remove => '-',
        }
    }
}

fn _hash(s: impl AsRef<str>) -> u32 {
    s.as_ref().bytes().fold(0u32, |acc, b| {
        let mut res = acc + b as u32;
        res *= 17;
        res % 256
    })
}

#[derive(Debug)]
struct Command {
    label: String,
    op: Op,
    focal_length: Option<u32>,
}

impl Command {
    fn execute(&self, boxes: &mut [LensBox]) {
        let b = &mut boxes[_hash(&self.label) as usize];
        b.apply(self);
    }

    fn from_str(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        let op = s.chars().find_map(Op::from_char).unwrap();
        let (label, focal_length) = s.split_once(op.as_char()).unwrap();

        Self {
            label: label.to_string(),
            op,
            focal_length: focal_length.parse().ok(),
        }
    }
}

#[derive(Debug)]
struct LensBox {
    num: u32, // one indexed
    contents: HashMap<String, u32>,
    label_order: HashMap<String, u32>,
    count: RangeFrom<u32>,
}

impl LensBox {
    fn new(num: u32) -> Self {
        Self {
            num,
            contents: hashmap! {},
            label_order: hashmap! {},
            count: 0u32..,
        }
    }

    fn calc_focusing_power(&self) -> u32 {
        let mut labels = self.label_order.iter().map(|(k, v)| (*v, k)).collect_vec();
        labels.sort();
        labels
            .iter()
            .map(|(_, label)| *label)
            .enumerate()
            .map(|(i, label)| {
                let i = i as u32;
                let focal_length = self.contents.get(label).unwrap();
                self.num * (i + 1) * focal_length
            })
            .sum()
    }

    fn apply(&mut self, cmd: &Command) {
        match cmd.op {
            // todo: better way to deal with avoiding unnecessary clones
            Op::Add => {
                self.contents.insert(cmd.label.clone(), cmd.focal_length.unwrap());
                if self.label_order.get(&cmd.label).is_none() {
                    let num = self._next_num();
                    self.label_order.insert(cmd.label.clone(), num);
                }
            }
            Op::Remove => {
                self.label_order.remove(&cmd.label);
                self.contents.remove(&cmd.label);
            },
        };
    }

    fn _next_num(&mut self) -> u32 {
        self.count.next().unwrap()
    }
}
fn _get_data(fname: &str) -> String {
    read_file23(fname).join("\n")
}
