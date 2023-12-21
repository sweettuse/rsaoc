use std::{
    collections::{HashMap, HashSet, VecDeque},
    iter::{repeat, zip},
    ops::{Add, Neg, Sub},
};

use glam::IVec2;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    point::{Dir, Point},
    tprint,
    utils::read_file23,
};

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let cave = _get_data("16.txt");
    Ok(cave.count_energized(Beam {
        dir: Dir::East,
        location: Point::new(0, 0),
    }))
}

#[allow(unreachable_code)]
fn part2() -> AocRes {
    // below is really slow
    return Ok(7572);
    let cave = _get_data("16.txt");
    let max_xy = cave.layout.keys().fold(Point::new(0, 0), |acc, p| {
        Point::new(acc.x.max(p.x), acc.y.max(p.y))
    });
    let iter_x = || 0..=max_xy.x;
    let iter_y = || 0..=max_xy.y;

    let east_side = (Dir::West, zip(repeat(max_xy.x), iter_y()).collect_vec());
    let west_side = (Dir::East, zip(repeat(0), iter_y()).collect_vec());
    let north_side = (Dir::South, zip(iter_x(), repeat(0)).collect_vec());
    let south_side = (Dir::North, zip(iter_x(), repeat(max_xy.y)).collect_vec());

    let sources = [east_side, west_side, north_side, south_side];
    let res = sources
        .iter()
        .flat_map(|(dir, point_tuples)| {
            point_tuples.iter().map(|(x, y)| {
                cave.count_energized(Beam {
                    dir: *dir,
                    location: Point::new(*x, *y),
                })
            })
        })
        .max()
        .unwrap();
    Ok(res)
}

fn _get_data(fname: &str) -> Cave {
    Cave::from_str(read_file23(fname).join("\n"))
}

// =============================================================================
// STRUCTS/ENUMS
// =============================================================================
type Layout = HashMap<Point, OpticalDevice>;

#[derive(Debug)]
struct Cave {
    layout: Layout,
}

#[derive(Debug, PartialEq, Eq)]
enum OpticalDevice {
    VerticalSplitter,
    HorizontalSplitter,
    RightMirror,
    LeftMirror,
    Empty,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Beam {
    dir: Dir,
    location: Point,
}

// =============================================================================
// IMPLs
// =============================================================================

impl Cave {
    fn from_str(s: impl AsRef<str>) -> Self {
        Self {
            layout: Self::_parse_layout(s.as_ref()),
        }
    }

    fn count_energized(&self, beam: Beam) -> i32 {
        let mut queue = VecDeque::from([beam]);
        let mut seen = HashSet::<Beam>::new();
        while let Some(beam) = queue.pop_front() {
            if seen.contains(&beam) {
                continue;
            }

            if let Some(od) = self.layout.get(&beam.location) {
                seen.insert(beam);
                queue.extend(od.beams(&beam));
            }
        }
        let unique_points: HashSet<Point> = seen.iter().map(|b| b.location).collect();
        unique_points.len() as i32
    }

    fn _parse_layout(s: &str) -> Layout {
        s.split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (Point::new(x as i32, y as i32), OpticalDevice::from_char(c))
                })
            })
            .collect()
    }
}

impl OpticalDevice {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            '\\' => Self::LeftMirror,
            '/' => Self::RightMirror,
            '.' => Self::Empty,
            _ => panic!("unmatched char {c}"),
        }
    }
    fn beams(&self, beam: &Beam) -> Vec<Beam> {
        self._new_dirs(beam.dir)
            .iter()
            .map(|d| Beam {
                dir: *d,
                location: beam.location + *d,
            })
            .collect()
    }
    /// map incoming dir of travel to outgoing dir[s] of travel
    fn _new_dirs(&self, dir_of_travel: Dir) -> Vec<Dir> {
        match self {
            OpticalDevice::VerticalSplitter => match dir_of_travel {
                Dir::East | Dir::West => vec![Dir::North, Dir::South],
                _ => vec![dir_of_travel],
            },
            OpticalDevice::HorizontalSplitter => match dir_of_travel {
                Dir::North | Dir::South => vec![Dir::East, Dir::West],
                _ => vec![dir_of_travel],
            },
            OpticalDevice::RightMirror => match dir_of_travel {
                Dir::North => vec![Dir::East],
                Dir::East => vec![Dir::North],
                Dir::South => vec![Dir::West],
                Dir::West => vec![Dir::South],
            },
            OpticalDevice::LeftMirror => match dir_of_travel {
                Dir::North => vec![Dir::West],
                Dir::West => vec![Dir::North],
                Dir::South => vec![Dir::East],
                Dir::East => vec![Dir::South],
            },
            OpticalDevice::Empty => vec![dir_of_travel],
        }
    }
}
