use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    point::{Dir, Point},
    tprint,
    utils::read_file23,
};

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

#[allow(unreachable_code)]
fn part1() -> AocRes {
    return Err("unsolved".to_string());
    // too slow, need to fix, see obsidian day17

    let mut city = _get_data("17.txt");
    city.wend();
    Ok(city.total_heat_loss())
}

fn part2() -> AocRes {
    Err("unsolved".to_string())
}

fn _get_data(fname: &str) -> City {
    City::from_str(read_file23(fname).join("\n"))
}

// =============================================================================
// STRUCTs/ENUMs
// =============================================================================
#[derive(Debug)]
struct City {
    grid: Vec<Vec<Node>>,
}

/// RowColumn
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct RC(i32, i32);

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct CrucibleInfo {
    location: Point,
    dir: Dir,
    streak: u32,
    heat_loss: u32,
}

#[derive(Debug, Eq, PartialEq, Hash, EnumIter)]
enum Turn {
    Straight,
    Right,
    Left,
}

#[derive(Debug)]
struct Node {
    visitors: HashMap<CIKey, u32>,
    heat_loss: u32,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct CIKey(Dir);

// =============================================================================
// IMPLs
// =============================================================================

impl CrucibleInfo {
    // generate next possible crucible infos from existing
    fn next_infos(&self) -> Vec<Self> {
        Turn::iter()
            .filter_map(|t| self._with_new_dir(t))
            .collect()
    }

    fn key(&self) -> CIKey {
        CIKey(self.dir)
    }

    fn _with_new_dir(&self, turn: Turn) -> Option<Self> {
        let dir = self.dir.turn(turn);
        let streak = if dir == self.dir {
            if self.streak == 3 {
                return None;
            }
            self.streak + 1
        } else {
            1
        };

        Some(Self {
            location: self.location + dir,
            dir,
            streak,
            heat_loss: self.heat_loss,
        })
    }
}

impl Node {
    fn update(&mut self, mut ci: CrucibleInfo) -> Option<CrucibleInfo>{
        ci.heat_loss += self.heat_loss;
        let loss = self.visitors.entry(ci.key()).or_insert(ci.heat_loss);
        if *loss < ci.heat_loss {
            None
        } else {
            *loss = ci.heat_loss;
            Some(ci)
        }
    }
}

impl City {
    fn wend(&mut self) {
        let p = Point::new(0, 0);
        let start = CrucibleInfo {
            location: p,
            dir: Dir::East,
            heat_loss: 0u32,
            streak: 0u32,
        };
        let mut queue = VecDeque::from([start]);
        while let Some(ci) = queue.pop_front() {
            for next_ci in ci.next_infos() {
                if let Some(node) = self.get(&next_ci.location) {
                    if let Some(next_ci) = node.update(next_ci) {
                        queue.push_back(next_ci);
                    };
                };
            }
        }
    }

    fn total_heat_loss(&self) -> u32 {
        let node = self.grid.iter().last().unwrap().iter().last().unwrap();
        *node.visitors.values().min().unwrap()
    }

    fn from_str(s: impl AsRef<str>) -> Self {
        let grid = s
            .as_ref()
            .split('\n')
            .map(|line| line.chars().map(|c| {
                Node {
                    visitors: HashMap::default(),
                    heat_loss: c.to_digit(10).unwrap(),
                }
            }).collect_vec())
            .collect_vec();

        Self { grid }
    }

    fn get(&mut self, p: &Point) -> Option<&mut Node> {
        let (row, col) = (p.y, p.x);
        if row < 0 || col < 0 {
            return None;
        }
        if let Some(row_vec) = self.grid.get_mut(row as usize) {
            if let Some(v) = row_vec.get_mut(col as usize) {
                return Some(v);
            }
        }
        None
    }
}

impl Dir {
    fn turn(&self, turn: Turn) -> Dir {
        self.rotate(match turn {
            Turn::Straight => 0,
            Turn::Right => 1,
            Turn::Left => -1,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_turn() {
        assert_eq!(Dir::North.turn(Turn::Right), Dir::East);
        assert_eq!(Dir::North.turn(Turn::Left), Dir::West);
        assert_eq!(Dir::North.turn(Turn::Straight), Dir::North);
    }

    #[test]
    fn test_generics() {
        fn get_biggest<T: PartialOrd>(list: &[T]) -> &T {
            list.iter()
                .reduce(|acc, v| match v < acc {
                    true => acc,
                    false => v,
                })
                .unwrap()
        }
        let v = vec![1, 5, 2, 12];
        let t = get_biggest(&v);
        assert_eq!(12, *t);
    }

    #[test]
    fn test_trait() {
        trait Summarize {
            fn s(&self) -> String;
        }

        struct Mine {
            author: String,
        }

        impl Summarize for Mine {
            fn s(&self) -> String {
                format!("by: {}", self.author)
            }
        }
    }

    #[test]
    fn test_lifetime() {
        fn first_word(s: &str) -> &str {
            s.chars()
                .enumerate()
                .find_map(|(i, c)| match c == ' ' {
                    true => Some(&s[0..i]),
                    false => None,
                })
                .unwrap_or(s)
        }
    }
}
