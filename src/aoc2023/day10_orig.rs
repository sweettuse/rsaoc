use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::{Add, Sub},
};

use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::{print1, tprint, utils::read_file23};

type AocRes = Result<u32, &'static str>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let mut system = _get_data("10.txt");
    Ok(system.part1())
}

fn part2() -> AocRes {
    Err("unsolved")
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

#[derive(Debug)]
struct Node {
    point: Point,
    distance: Option<u32>,
    type_: char,
    connections: HashSet<Point>,
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
impl Node {
    fn neighbors(&self) -> Vec<Point> {
        if self.type_ == 'S' {
            return [p(1, 0), p(-1, 0), p(0, 1), p(0, -1)].iter().map(|p| *p + self.point).collect();
        }
        match PIPE_MAP.get(&self.type_) {
            Some((o1, o2)) => vec![self.point + *o1, self.point + *o2],
            None => panic!("no match for type {:?}", self.type_),
        }
    }
}

impl System {
    fn from_str(s: &str) -> Self {
        let (graph, start) = Self::_init_graph(s);
        let res = Self { graph, start };
        res._connect_nodes()
    }

    fn part1(&mut self) -> u32 {
        self.calc_distance();
        self
            .graph
            .values()
            .filter_map(|n| n.distance)
            .max()
            .unwrap()
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
                node.connections.iter().for_each(|p| {
                    to_check.push_back((*p, distance + 1));
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
                if c == '.' {
                    return;
                }
                let point = p(x as i32, y as i32);
                let node = Node {
                    point,
                    distance: None,
                    type_: c,
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

    /// go through all the nodes and connect them to other nodes
    fn _connect_nodes(mut self) -> Self {
        let mut point_neighbors_map: HashMap<_, Vec<Point>> = HashMap::new();
        self.graph.values().for_each(|node| {
            point_neighbors_map.insert(node.point, node.neighbors().to_vec());
        });
        point_neighbors_map.iter().for_each(|(from, neighbors)| {
            neighbors.iter().for_each(|n| {
                self._connect(from, n);
            });
        });
        self
    }

    /// connect a node to its neighbor and vice versa
    fn _connect(&mut self, point: &Point, neighbor: &Point) {
        //! check if each point is in the graph AND each node at those points contains
        //! the other as a neighbor
        match (self.graph.get(point), self.graph.get(neighbor)) {
            (Some(p), Some(n)) => {
                if !(p.neighbors().contains(&n.point) && n.neighbors().contains(&p.point)) {
                    return;
                }
            },
            (_, _) => return,
        };

        let mut _update = |p, n: &Point| {
            self.graph
                .get_mut(p)
                .unwrap()
                .connections
                .insert(*n);

        };
        _update(point, neighbor);
        _update(neighbor, point);
    }

    /// get the string representation of the graph like the problem
    fn debug_str(&self) -> String {
        let x_max = self.graph.values().map(|n| n.point.x).max().unwrap();
        let y_max = self.graph.values().map(|n| n.point.y).max().unwrap();
        for y in 0..=y_max {
            let mut s = String::new();
            for x in 0..=x_max {
                s.push(match self.graph.get(&p(x, y)) {
                    Some(n) if n.distance.is_some()  => n.type_ ,
                    _ => ' ',
                });
            }
            println!("{:?}", s.to_string());
        }

        String::new()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// =============================================================================
// TESTS
// =============================================================================

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_point_add_sub() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 9, y: 4 };
        assert_eq!(Point { x: 10, y: 5 }, p1 + p2);
        assert_eq!(Point { x: 8, y: 3 }, p2 - p1);
    }

    #[test]
    fn test_pipe_map() {
        assert_eq!(*PIPE_MAP.get(&'7').unwrap(), (p(-1, 0), p(0, 1)));
    }

    #[test]
    fn test_example_1() {
        let mut system = _get_data("10.txt.a");
        system.calc_distance();
        assert_eq!(system.part1(), 4);
    }

    #[test]
    fn test_example_2() {
        let mut system = _get_data("10.txt.test1");
        system.calc_distance();
        assert_eq!(system.part1(), 8);
    }

}

static PIPE_MAP: Lazy<HashMap<char, (Point, Point)>> = Lazy::new(|| {
    let n = Point { x: 0, y: -1 };
    let e = Point { x: 1, y: 0 };
    let s = Point { x: 0, y: 1 };
    let w = Point { x: -1, y: 0 };
    let pairs = vec![
        ('|', (n, s)),
        ('-', (e, w)),
        ('L', (n, e)),
        ('J', (n, w)),
        ('7', (w, s)),
        ('F', (s, e)),
    ];
    pairs.into_iter().collect()
});
