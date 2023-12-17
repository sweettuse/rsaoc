use std::{
    cell::RefCell,
    collections::{HashMap, HashSet, VecDeque},
    iter::zip,
    ops::{Add, Neg, Sub},
    rc::Rc,
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
type SharedCell = Rc<RefCell<Cell>>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let cave = _get_data("16.txt.test");
    tprint!(cave);
    Err("unsolved".to_string())
}

fn part2() -> AocRes {
    Err("unsolved".to_string())
}

fn _get_data(fname: &str) -> Cave {
    Cave::from_str(read_file23(fname).join("\n"))
}

// =============================================================================
// STRUCTS/ENUMS
// =============================================================================
type Layout = HashMap<Point, SharedCell>;

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

#[derive(Debug)]
struct Cell {
    p: Point,
    od: OpticalDevice,
    visitors: HashSet<Dir>,
    connections: HashMap<Dir, Point>,
}

#[derive(Debug, Copy, Clone)]
struct Beam {
    dir: Dir,
    location: Point,
}

// =============================================================================
// IMPLs
// =============================================================================

impl Cave {
    fn from_str(s: impl AsRef<str>) -> Self {
        let mut layout = Self::_parse_layout(s.as_ref());
        let start = Point::new(0, 0);
        layout.insert(start, Cell::shared(&start, OpticalDevice::Empty));
        Self::_connect_cells(&mut layout);

        Self { layout }
    }

    fn process(&self, beam: Beam) {
        let mut queue = VecDeque::from([&beam]);
        while let Some(beam) = queue.pop_front() {
            let cell = self.layout.get(&beam.location);
            if cell.is_none() {
                continue;
            }
            let mut cell = cell.unwrap().borrow_mut();
            if cell.seen(beam) {
                continue;
            }
            cell.visitors.insert(beam.dir);
        }

    }

    // fn _find_start(beam: &Beam) {

    // }
}

impl Cell {
    fn shared(p: &Point, od: OpticalDevice) -> SharedCell {
        Rc::new(RefCell::new(Self::from(p, od)))
    }

    fn from(p: &Point, od: OpticalDevice) -> Self {
        Self {
            p: *p,
            od,
            visitors: HashSet::new(),
            connections: HashMap::new(),
        }
    }

    fn seen(&self, beam: &Beam) -> bool {
        let contains = |dir: &Dir| self.visitors.contains(dir);
        match self.od {
            OpticalDevice::RightMirror | OpticalDevice::LeftMirror => contains(&beam.dir),
            OpticalDevice::VerticalSplitter => match beam.dir {
                Dir::North | Dir::South => contains(&beam.dir),
                Dir::East | Dir::West => contains(&Dir::East) || contains(&Dir::West),
            },
            OpticalDevice::HorizontalSplitter => match beam.dir {
                Dir::East | Dir::West => contains(&beam.dir),
                Dir::North | Dir::South => contains(&Dir::North) || contains(&Dir::South),
            },
            OpticalDevice::Empty => !self.visitors.is_empty(),
        }
    }

    fn beams(&self, beam: &Beam) -> Vec<Beam> {
        self.od.new_dirs(beam.dir).iter().filter_map(|dir| {
            if let Some(next) = self.connections.get(dir) {
                return Some(Beam { dir: *dir, location: *next });
            }
            None
        }).collect()
    }
}

// helpers
impl Cave {
    fn _parse_layout(s: &str) -> Layout {
        s.split('\n')
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if let Some(od) = OpticalDevice::from_char(c) {
                        let p = Point::new(x as i32, y as i32);
                        return Some((p, Cell::shared(&p, od)));
                    }
                    None
                })
            })
            .collect()
    }

    fn _connect_cells(layout: &mut Layout) {
        Self::_connect_cells_dir(layout, Dir::North);
        Self::_connect_cells_dir(layout, Dir::East);
    }

    fn _connect_cells_dir(layout: &mut Layout, dir: Dir) {
        let mut _grouped = Self::_group_direction(layout, dir);
        _grouped
            .drain()
            .map(|(_, v)| Self::_sort_by_direction(v, dir))
            .for_each(|v| {
                let (left_to_right, right_to_left) = match dir {
                    Dir::North | Dir::South => (Dir::South, Dir::North),
                    Dir::East | Dir::West => (Dir::West, Dir::East),
                };
                v.iter().tuple_windows().for_each(|(lc, rc)| {
                    let mut mlc = lc.borrow_mut();
                    mlc.connections.insert(left_to_right, rc.borrow().p);
                    drop(mlc);

                    let mut mrc = rc.borrow_mut();
                    mrc.connections.insert(right_to_left, lc.borrow().p);
                    drop(mrc);
                })
            });
    }

    fn _group_direction(layout: &mut Layout, dir: Dir) -> HashMap<i32, Vec<SharedCell>> {
        layout
            .iter()
            .map(|(p, c)| {
                let axis = match dir {
                    Dir::North | Dir::South => p.y,
                    Dir::East | Dir::West => p.x,
                };
                (axis, c)
            })
            .fold(HashMap::new(), |mut acc, (axis, c)| {
                acc.entry(axis).or_default().push(Rc::clone(c));
                acc
            })
    }

    fn _sort_by_direction(mut vals: Vec<SharedCell>, dir: Dir) -> Vec<SharedCell> {
        vals.sort_by(|cell1, cell2| {
            let c1 = cell1.borrow();
            let c2 = cell2.borrow();
            match dir {
                Dir::North | Dir::South => c1.p.y.cmp(&c2.p.y),
                Dir::East | Dir::West => c1.p.x.cmp(&c2.p.x),
            }
        });
        vals
    }
}

impl OpticalDevice {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self::VerticalSplitter),
            '-' => Some(Self::HorizontalSplitter),
            '\\' => Some(Self::LeftMirror),
            '/' => Some(Self::RightMirror),
            _ => None,
        }
    }
    fn as_char(&self) -> char {
        match self {
            Self::VerticalSplitter => '|',
            Self::HorizontalSplitter => '-',
            Self::LeftMirror => '\\',
            Self::RightMirror => '/',
            Self::Empty => '.',
        }
    }

    /// map incoming dir of travel to outgoing dir[s] of travel
    fn new_dirs(&self, dir_of_travel: Dir) -> Vec<Dir> {
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
