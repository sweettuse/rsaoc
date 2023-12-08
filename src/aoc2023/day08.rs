use std::{collections::HashMap, slice, str::FromStr, vec};

use itertools::Itertools;
use num_integer::lcm;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{print1, utils::read_file23};

pub type AocRes = Result<usize, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let node_map = _get_data("08.txt");
    Ok(node_map.part1())
}

fn part2() -> AocRes {
    let node_map = _get_data("08.txt");
    Ok(node_map.part2())
}

fn _get_data(fname: &str) -> NodeMap {
    let lines = read_file23(fname);
    NodeMap::from_str(lines.join("\n").as_str()).unwrap()
}

enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Directions {
    directions: Vec<char>,
    idx: usize,
}

struct DirectionsIter<'a> {
    directions: &'a Vec<char>,
    idx: usize,
}

#[derive(Debug, Clone)]
struct Node {
    source: String,
    left: String,
    right: String,
}

#[derive(Debug)]
struct NodeMap {
    directions: Directions,
    nodes: HashMap<String, Node>,
}

// =================================================================================================
// Impl
// =================================================================================================
impl NodeMap {
    fn part1(&self) -> usize {
        self._traverse("AAA", true)
    }

    // get the lcm of all the paths
    fn part2(&self) -> usize {
        self.nodes
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|start| self._traverse(start, false))
            .fold(1, lcm)
    }

    fn _traverse(&self, start: &str, is_zzz: bool) -> usize {
        let mut cur = self.nodes.get(start).unwrap();

        let is_end = match is_zzz {
            true => |cur: &Node| cur.source == "ZZZ",
            false => |cur: &Node| cur.source.ends_with('Z'),
        };

        for (i, d) in self.directions.iter().enumerate() {
            cur = self._get(match d {
                Direction::Left => &cur.left,
                Direction::Right => &cur.right,
            });
            if is_end(cur) {
                return i + 1;
            }
        }
        panic!()
    }

    fn _get(&self, s: &str) -> &Node {
        self.nodes.get(s).unwrap()
    }
}

// =================================================================================================
// FromStr
// =================================================================================================

impl FromStr for Directions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            directions: s.chars().collect(),
            idx: 0,
        })
    }
}

impl FromStr for Node {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\w+").unwrap());
        let (source, left, right) = RE
            .find_iter(s)
            .map(|m| m.as_str().to_owned())
            .collect_tuple()
            .unwrap();
        Ok(Self {
            source,
            left,
            right,
        })
    }
}

impl FromStr for NodeMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split('\n');
        let directions = Directions::from_str(lines.next().unwrap()).unwrap();
        lines.next();
        let mut nodes: HashMap<_, _> = HashMap::new();
        lines.for_each(|s| {
            let n = Node::from_str(s).unwrap();
            nodes.insert(n.source.clone(), n);
        });

        Ok(Self { directions, nodes })
    }
}

// =================================================================================================
// Iterator
// =================================================================================================
impl<'a> DirectionsIter<'a> {
    fn new(directions: &'a Vec<char>) -> Self {
        DirectionsIter { directions, idx: 0 }
    }
}

impl<'a> Iterator for DirectionsIter<'a> {
    type Item = Direction;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.directions.get(self.idx);
        self.idx = (self.idx + 1) % self.directions.len();

        result.map(|c| match *c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        })
    }
}

impl Directions {
    fn iter(&self) -> DirectionsIter {
        DirectionsIter::new(&self.directions)
    }
}
