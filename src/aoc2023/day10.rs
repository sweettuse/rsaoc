use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, Neg, Sub},
};

use itertools::Itertools;
use num_integer::Integer;
use once_cell::sync::Lazy;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{print1, tprint, utils::read_file23};

type AocRes = Result<u32, &'static str>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let mut system = _get_data("10.txt");
    system.calc_distance();
    system
        .graph
        .values()
        .filter_map(|n| n.distance)
        .max()
        .ok_or("part1 failure!")
}

fn _get_data(fname: &str) -> System {
    let lines = read_file23(fname);
    System::from_str(&lines.join("\n"))
}

// =============================================================================
// STRUCTS/ENUMS
// =============================================================================
type Graph = HashMap<Point, Node>;
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

#[derive(Debug)]
struct Node {
    point: Point,
    distance: Option<u32>,
    type_: char,
    /// where the pipes lead to
    outputs: HashSet<Dir>,
    /// where the pipes are connected. a subset of `outputs`
    connections: HashSet<Dir>,
}

#[derive(Debug)]
/// the pipe system in the problem
struct System {
    graph: Graph,
    start: Point,
}

/// convenience constructor for `Point`
fn p<T: Into<i32>>(x: T, y: T) -> Point {
    Point {
        x: x.into(),
        y: y.into(),
    }
}

// =============================================================================
// IMPLs
// =============================================================================

impl System {
    fn from_str(s: &str) -> Self {
        let (graph, start) = Self::_init_graph(s);
        Self::_make_connections(Self { graph, start })
    }

    // BFS the path from the start around the loop and calculate the distance
    // to each node
    fn calc_distance(&mut self) {
        let mut to_check = VecDeque::from(vec![(self.start, 0u32)]);
        while let Some((p, distance)) = to_check.pop_front() {
            if let Some(node) = self.graph.get_mut(&p) {
                if let Some(d) = node.distance {
                    if d <= distance {
                        continue;
                    }
                }
                node.distance = Some(distance);
                node.connections.iter().for_each(|d| {
                    to_check.push_back((node.point + *d, distance + 1));
                })
            }
        }
    }
}

/// initialization logic/"private interface"?
impl System {
    /// create the unconnected graph.
    /// return the graph and the starting point
    fn _init_graph(s: &str) -> (Graph, Point) {
        let mut graph: HashMap<_, _> = HashMap::new();
        let mut start: Option<Point> = None;
        s.split('\n').enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                let point = p(x as i32, y as i32);
                let node = Node {
                    point,
                    distance: None,
                    type_: c,
                    outputs: Self::_get_outputs(c),
                    connections: HashSet::new(),
                };
                if node.type_ == 'S' {
                    start = Some(point);
                }

                graph.insert(point, node);
            })
        });
        (graph, start.unwrap())
    }

    fn _get_outputs(pipe_type: char) -> HashSet<Dir> {
        match pipe_type {
            '|' => hashset! {Dir::North, Dir::South},
            '-' => hashset! {Dir::East, Dir::West},
            'L' => hashset! {Dir::North, Dir::East},
            'J' => hashset! {Dir::North, Dir::West},
            '7' => hashset! {Dir::West, Dir::South},
            'F' => hashset! {Dir::South, Dir::East},
            'S' => hashset! {Dir::North, Dir::South, Dir::East, Dir::West},
            '.' => hashset! {},
            _no_match => panic!("no known outputs for type: {_no_match:?}"),
        }
    }

    fn _make_connections(mut self) -> Self {
        let mut connections: HashMap<Point, HashSet<Dir>> = hashmap! {};

        self.graph.values().for_each(|node| {
            node.outputs.iter().for_each(|d| {
                let d = *d;
                if let Some(neighbor) = self.graph.get(&(node.point + d)) {
                    if neighbor.outputs.contains(&-d) {
                        connections.entry(node.point).or_default().insert(d);
                    }
                }
            });
        });
        connections.into_iter().for_each(|(p, dirs)| {
            if let Some(node) = self.graph.get_mut(&p) {
                node.connections = dirs;
            }
        });
        self
    }

    fn debug_str(&self) -> String {
        let x_max = self.graph.values().map(|n| n.point.x).max().unwrap();
        let y_max = self.graph.values().map(|n| n.point.y).max().unwrap();
        for y in 0..=y_max {
            let mut s = String::new();
            for x in 0..=x_max {
                s.push(match self.graph.get(&p(x, y)) {
                    Some(n) if n.distance.is_some() => n.type_,
                    _ => ' ',
                });
            }
            println!("{:?}", s.to_string());
        }

        String::new()
    }
}

// =============================================================================
// OPERATOR OVERLOADS
// =============================================================================

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Dir> for Point {
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        self + rhs.offset()
    }
}

impl Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Dir> for Point {
    type Output = Self;

    fn sub(self, rhs: Dir) -> Self::Output {
        self - rhs.offset()
    }
}

impl Neg for Dir {
    type Output = Dir;

    fn neg(self) -> Self::Output {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}
impl Dir {
    fn offset(&self) -> Point {
        match self {
            Dir::North => p(0, -1),
            Dir::South => p(0, 1),
            Dir::East => p(1, 0),
            Dir::West => p(-1, 0),
        }
    }
}

// =============================================================================
// PART 2
// =============================================================================

fn part2() -> AocRes {
    // options:
    // - explode then fill
    // - count traversed walls
    // - F-7


    let mut system = _get_data("10.txt");
    system.calc_distance();
    let graph = system.graph;
    let wall_counter = do_the_accumulate(&graph);

    Ok(wall_counter.values().filter(|n2| { n2.is_within() }).count() as u32)
    // tprint!(wall_counter.get(&Point {x: 7, y: 4}));
    // tprint!(wall_counter.get(&Point {x: 4, y: 7}));
}

fn _blocking(dir: Dir) -> (Dir, Dir) {
    match dir {
        Dir::East | Dir::West => (Dir::North, Dir::South),
        Dir::North | Dir::South => (Dir::East, Dir::West),
    }
}

fn do_the_accumulate(graph: &Graph) -> WallCounter{
    let mut wall_counter = _init_wall_counter(graph);
    let (max_x, max_y) = _get_max_x_max_y(graph);
    for x in 0..=max_x {
        _accumulate_walls(
            Point { x: x as i32, y: 0 },
            Dir::South,
            graph,
            &mut wall_counter,
        );
        _accumulate_walls(
            Point {
                x: x as i32,
                y: max_y as i32,
            },
            Dir::North,
            graph,
            &mut wall_counter,
        );
    }
    for y in 0..=max_y {
        _accumulate_walls(
            Point { x: 0, y: y as i32 },
            Dir::East,
            graph,
            &mut wall_counter,
        );
        _accumulate_walls(
            Point {
                x: max_x as i32,
                y: y as i32,
            },
            Dir::West,
            graph,
            &mut wall_counter,
        );
    }
    wall_counter

}


/// this is gross
/// the idea here is to count the number of walls of the loop a point has to pass through to get out
/// determining which walls each point is intersecting is done from all 4 directions (not sure if necessary)
/// say you're looking from the west heading east. 
/// each time i pass a south then north that means i'm blocked
/// so e.g. F-J would be blocking or | would be blocking
/// but F-7 e.g. would *not* be blocking. i crossed a south and then another south with no north in between, meaning
/// that i'd be able to squeech along the upper side.
/// that's the algo in a nutshell, implemented grossly
fn _accumulate_walls(start: Point, dir: Dir, graph: &Graph, wall_counter: &mut WallCounter) {
    let mut cur = start;
    let mut walls_seen: u32 = 0;

    let (d1, d2) = _blocking(dir);
    let mut counts = hashmap!{
        d1 => 0,
        d2 => 0,
    };
    while let Some(n) = graph.get(&cur) {

        let (con_d1, con_d2) = (n.connections.contains(&d1), n.connections.contains(&d2));



        if n.distance.is_some() && (con_d1 || con_d2){
            if con_d1 && con_d2 {
                counts.insert(d1, 0);
                counts.insert(d2, 0);
                walls_seen += 1;
            } else {
                let cur_d = match con_d1 {
                    true => d1,
                    false => d2,
                };
                let cur_count = counts.entry(cur_d).or_insert(0);
                if *cur_count == 1 {
                    *cur_count = 0;
                } else {
                    *cur_count += 1;
                    if counts.values().sum::<i32>() == 2 {
                        counts.insert(d1, 0);
                        counts.insert(d2, 0);
                        walls_seen += 1;
                    }
                }
            }
        }

        if n.distance.is_none() {
            let n2 = wall_counter.get_mut(&cur).unwrap();
            *n2.counts.entry(dir).or_insert(0) += walls_seen;
        }
        cur = cur + dir;
    }
}

#[derive(Debug)]
struct Node2 {
    point: Point,
    counts: HashMap<Dir, u32>,
}

type WallCounter = HashMap<Point, Node2>;

fn _init_wall_counter(graph: &Graph) -> WallCounter {
    let mut res = HashMap::new();
    graph.values().for_each(|n| {
        if n.distance.is_some() {
            return;
        }
        let mut counts = HashMap::new();
        Dir::iter().for_each(|d| {
            counts.insert(d, 0u32);
        });
        res.insert(
            n.point,
            Node2 {
                point: n.point,
                counts,
            },
        );
    });
    res
}

fn _get_max_x_max_y(graph: &Graph) -> (u32, u32) {
    let mut max_x = 0u32;
    let mut max_y = 0u32;
    graph.keys().for_each(|p| {
        max_x = max_x.max(p.x as u32);
        max_y = max_y.max(p.y as u32);
    });

    (max_x, max_y)
}

impl Node2 {
    fn is_within(&self) -> bool {
        self.counts.values().all(Integer::is_odd)
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn opposite_dir() {
        assert_eq!(-Dir::North, Dir::South);
    }

    #[test]
    fn test_point_add_sub() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 9, y: 4 };
        assert_eq!(Point { x: 10, y: 5 }, p1 + p2);
        assert_eq!(Point { x: 8, y: 3 }, p2 - p1);
    }

    #[test]
    fn test_pipe_map() {
        assert_eq!(System::_get_outputs('7'), hashset! {Dir::West, Dir::South});
    }
}
