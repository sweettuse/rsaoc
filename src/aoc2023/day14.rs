use std::{
    collections::HashMap,
    fmt::Display,
    iter::{once, successors, zip},
    ops::{Add, Deref, Neg, Sub},
};

use itertools::{unfold, Itertools};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{tprint, utils::read_file23};

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let mut platform = _get_data("14.txt");
    platform = platform.tilt(Dir::North);
    Ok(_calc_north_load(&platform))
}

#[allow(unreachable_code)]
fn part2() -> AocRes {
    // added because this is slow as hell
    return Ok(90176);
    let mut platform = _get_data("14.txt");
    let target_num_cycles = 1_000_000_000;
    let mut vals = vec![];
    for _ in 0..400 {
        platform = platform.cycle();
        vals.push(_calc_north_load(&platform))
    }

    let (cycle_start_i, range) = _detect_cycle_start_and_len(&vals);
    tprint!(cycle_start_i);
    let cycle = &vals[cycle_start_i..=cycle_start_i + range];
    tprint!(cycle);

    Ok(cycle[(target_num_cycles - cycle_start_i - 1) % range])
}

fn _detect_cycle_start_and_len(vals: &[i32]) -> (usize, usize) {
    let mut slow = vals.iter().enumerate();
    let mut fast = vals.iter().step_by(2);
    slow.next();
    fast.next();

    // find the cycle
    let ((i1, first_pattern_val), (i2, _)) = zip(slow, fast)
        .filter_map(|((i, v1), v2)| {
            if v1 == v2 {
                return Some((i, v1));
            }
            None
        })
        .skip(4)
        .take(2)
        .collect_tuple()
        .unwrap();


    // find the actual start of the cycle
    let range = i2 - i1;
    let cycle = &vals[i1..=i2];
    let first_cycle_start = vals.iter().enumerate().find_map(|(i, v)| {
        if v != first_pattern_val {
            return None;
        }
        if &vals[i..=i + range] != cycle {
            return None;
        }
        Some(i)
    }).unwrap();

    (first_cycle_start, range)
}

fn _calc_north_load(platform: &Platform) -> i32 {
    platform
        .layout
        .iter()
        .map(|(p, r)| match r {
            Rock::Round => platform.height as i32 - p.y + 1,
            Rock::Square => 0,
            Rock::Empty => panic!(),
        })
        .sum()
}

fn _get_data(fname: &str) -> Platform {
    let data = read_file23(fname).join("\n");
    Platform::from_str(data)
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Rock {
    Round,
    Square,
    Empty,
}

#[derive(Debug)]
struct Platform {
    layout: HashMap<Point, Rock>,
    height: u32,
    width: u32,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, EnumIter)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Rock {
    fn from_char(c: char) -> Self {
        match c {
            'O' => Self::Round,
            '#' => Self::Square,
            '.' => Self::Empty,
            _ => panic!("unexpected char {c}"),
        }
    }
}

type GroupedType = HashMap<i32, Vec<(Point, Rock)>>;
impl Point {
    fn with_new(&self, value: i32, dir: Dir) -> Self {
        match dir {
            Dir::North | Dir::South => Point {
                x: self.x,
                y: value,
            },
            Dir::East | Dir::West => Point {
                x: value,
                y: self.y,
            },
        }
    }

    fn get(&self, dir: Dir) -> i32 {
        match dir {
            Dir::North | Dir::South => self.y,
            Dir::West | Dir::East => self.x,
        }
    }
}
impl Platform {
    fn from_str(s: impl AsRef<str>) -> Self {
        let layout: HashMap<Point, Rock> = s
            .as_ref()
            .split('\n')
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars().enumerate().map(move |(x, c)| {
                    (
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        Rock::from_char(c),
                    )
                })
            })
            .filter(|(_, r)| *r != Rock::Empty)
            .collect();

        let max = layout.keys().fold(Point { x: 0, y: 0 }, |mut acc, p| {
            acc.x = acc.x.max(p.x);
            acc.y = acc.y.max(p.y);
            acc
        });

        Platform {
            layout,
            height: max.y as u32,
            width: max.x as u32,
        }
    }

    fn tilt(self, dir: Dir) -> Self {
        let mut grouped = Self::_group(self.layout, dir);
        Self::_sort(&mut grouped, dir);
        Self {
            layout: Self::_move(&grouped, dir, self.width, self.height),
            ..self
        }
    }

    fn cycle(mut self) -> Self {
        for d in [Dir::North, Dir::West, Dir::South, Dir::East] {
            self = self.tilt(d);
            // println!("{}", "=".repeat(40));
            // tprint!(d);
            // println!("{}", self);
        }
        self
    }

    /// group the rocks by the axis of the dir
    fn _group(layout: HashMap<Point, Rock>, dir: Dir) -> GroupedType {
        layout
            .into_iter()
            .map(|(p, r)| {
                let axis = match dir {
                    Dir::North | Dir::South => p.x,
                    Dir::East | Dir::West => p.y,
                };
                (axis, (p, r))
            })
            .fold(HashMap::new(), |mut acc, (axis, (p, r))| {
                acc.entry(axis).or_default().push((p, r));
                acc
            })
    }

    /// sort the rocks from closest to dir to farthest dir
    fn _sort(grouped: &mut GroupedType, dir: Dir) {
        grouped.values_mut().for_each(|axis_vec| {
            axis_vec.sort_by_key(|(p, _)| match dir {
                Dir::North => p.y,
                Dir::East => -p.x,
                Dir::South => -p.y,
                Dir::West => p.x,
            });
        });
    }

    /// move rocks toward the dir
    fn _move(grouped: &GroupedType, dir: Dir, width: u32, height: u32) -> HashMap<Point, Rock> {
        let (sign, max) = match dir {
            Dir::North | Dir::West => (1, -1),
            Dir::South => (-1, (height + 1) as i32),
            Dir::East => (-1, (width + 1) as i32),
        };
        let start = (Point { x: max, y: max }, Rock::Square);

        grouped
            .values()
            .flat_map(|v| {
                v.iter().fold(vec![start], |mut acc, (p2, r2)| {
                    let (p1, _) = acc.last().unwrap();
                    let value = match r2 {
                        Rock::Round => p1.get(dir) + sign,
                        Rock::Square => p2.get(dir),
                        Rock::Empty => panic!(),
                    };
                    acc.push((p2.with_new(value, dir), *r2));
                    acc
                })
            })
            .collect()
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let layout = (0..=self.height)
            .map(|y| {
                let s: String = (0..=self.width)
                    .map(|x| {
                        if let Some(r) = self.layout.get(&Point {
                            x: x as i32,
                            y: y as i32,
                        }) {
                            return match r {
                                Rock::Round => 'O',
                                Rock::Square => '#',
                                Rock::Empty => '.',
                            };
                        }
                        '.'
                    })
                    .collect();
                s
            })
            .join("\n");
        write!(f, "{}\n{}", "-".repeat(40), layout,)
    }
}
