use crate::print1;
use crate::utils::read_file23;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;
use std::{collections::HashMap, ops::Range};

pub fn main() -> (i32, i32) {
    (part1("03.txt"), part2("03.txt"))
}

fn part1(path: &str) -> i32 {
    let engine = _get_data(path);
    let mut part_nums: HashSet<Number> = HashSet::new();
    for sym_coord in engine.symbol_coords.clone() {
        for v in engine.get_surrounding(sym_coord) {
            if let Value::Number(n) = v {
                part_nums.insert(n);
            }
        }
    }

    part_nums.iter().map(|n| n.num).sum()
}

fn part2(path: &str) -> i32 {
    let engine = _get_data(path);
    let mut res = 0;
    for sym_coord in engine.symbol_coords.clone() {
        if let Some(Value::Symbol(symbol)) = engine.get(sym_coord) {
            if *symbol != '*' {
                continue;
            }
        } else {
            continue;
        }

        let surrounding: HashSet<Number> = engine
            .get_surrounding(sym_coord)
            .iter()
            .filter_map(|v| match v {
                Value::Number(n) => Some(*n),
                _ => None,
            })
            .collect();

        if surrounding.len() != 2 {
            continue;
        }

        let (n1, n2) = surrounding.iter().map(|n| n.num).collect_tuple().unwrap();
        res += n1 * n2;
    }
    res
}

fn _get_data(path: &str) -> Engine {
    let mut engine = Engine::default();
    for (y, line) in read_file23(path).iter().enumerate() {
        engine.parse_line(line, y as i32);
    }
    engine
}

fn _find_numbers_with_positions(line: &str) -> Vec<(usize, usize, i32)> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(line)
        .map(|mat| (mat.start(), mat.end(), mat.as_str().parse::<i32>().unwrap()))
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Number {
    num: i32,
    x_start: usize,
    x_end: usize,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum Value {
    Number(Number),
    Symbol(char),
    Period,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Default)]
struct Engine {
    data: HashMap<Coord, Value>,
    symbol_coords: HashSet<Coord>,
    period_coords: HashSet<Coord>,
}

impl Engine {
    fn get(&self, coord: Coord) -> Option<&Value> {
        self.data.get(&coord)
    }

    fn get_surrounding(&self, coord: Coord) -> Vec<Value> {
        (coord.x - 1..coord.x + 2)
            .cartesian_product(coord.y - 1..coord.y + 2)
            .filter_map(|(x, y)| {
                let xy = Coord { x, y };
                if xy == coord {
                    return None;
                }
                return self.get(xy);
            })
            .copied()
            .collect()
    }

    fn parse_line(&mut self, line: &str, y: i32) {
        self._parse_numbers(line, y);
        self._parse_symbols(line, y);
    }

    fn _parse_numbers(&mut self, line: &str, y: i32) {
        for (start, end, num) in _find_numbers_with_positions(line) {
            for x in start..end {
                self.data.insert(
                    Coord { x: x as i32, y },
                    Value::Number(Number {
                        num,
                        x_start: start,
                        x_end: end,
                        y,
                    }),
                );
            }
        }
    }

    fn _parse_symbols(&mut self, line: &str, y: i32) {
        for (x, val) in line.chars().enumerate() {
            if val.is_ascii_digit() {
                continue;
            }
            let coord = Coord { x: x as i32, y };
            let val = match val {
                '.' => {
                    self.period_coords.insert(coord);
                    Value::Period
                }
                _ => {
                    self.symbol_coords.insert(coord);
                    Value::Symbol(val)
                }
            };
            self.data.insert(coord, val);
        }
    }
}
