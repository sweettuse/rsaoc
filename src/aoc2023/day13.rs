use std::{collections::HashSet, error::Error, iter::zip, ops::Index, str::FromStr};

use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{tprint, utils::read_file23};

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let notes = _get_data("13.txt");
    Ok(notes.patterns.iter().map(Pattern::value).sum())
}

fn part2() -> AocRes {
    let notes = _get_data("13.txt");
    let p = notes.patterns.first().unwrap();
    // tprint!(p.rows);
    // tprint!(p.cols);
    // tprint!(p.value2());
    // println!("{:b}", 346);
    // println!("00{:b}", 90);
    // tprint!(Pattern::_find_reflection(&[281, 265, 103, 502, 502, 103, 281]));

    // let p = &notes.patterns[0];
    // for axis in Axis::iter() {
    //     println!("{}", "=".repeat(30));
    //     tprint!(axis);
    //     let source = match axis {
    //         Axis::Row => &p.rows,
    //         Axis::Col => &p.cols,
    //     };
    //     tprint!(source);
    //     let res = Pattern::_find_reflection(source);
    //     tprint!(res);
    //     show_grid(source);
    //     // println!("{:012b}", source[0]);
    //     for vals__ in p._sub_off_by_ones(axis) {
    //         println!("{}", "-".repeat(30));
    //         tprint!(vals__);
    //         let res = Pattern::_find_reflection(&vals__);
    //         tprint!(res);
    //         show_grid(&vals__);
    //         // show_axis(&vals__[0]);
    //         // println!("{:012b}", vals__[0]);
    //         // println!();
    //         // tprint!();
    //     }
    // }

    // Err("unsolved".to_string())
    Ok(notes.patterns.iter().map(Pattern::value2).sum())
}

fn show_grid(vals: &[u32]) {
    vals.iter().for_each(show_axis);
}

fn show_axis(val: &u32) {
    let s = format!("{:012b}", val);
    println!("{:?}", s.replace('0', ".").replace('1', "#"));

}

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq)]
enum Axis {
    Row,
    Col,
}

#[derive(Debug)]
struct Pattern {
    data: Vec<Vec<char>>,
    rows: Vec<u32>,
    cols: Vec<u32>,
}

#[derive(Debug)]
struct Notes {
    patterns: Vec<Pattern>,
}

impl Notes {
    fn from_str(s: impl AsRef<str>) -> Self {
        Self {
            patterns: s
                .as_ref()
                .split("\n\n")
                .map(Pattern::from_str)
                .collect_vec(),
        }
    }
}

impl Pattern {
    fn from_str(s: impl AsRef<str>) -> Self {
        let data = s
            .as_ref()
            .split('\n')
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let transposed = (0..data[0].len())
            .map(|i| data.iter().map(|row| row[i]).collect())
            .collect_vec();

        let rows = Self::_get_axis_hash(&data);
        let cols = Self::_get_axis_hash(&transposed);
        Self { data, rows, cols }
    }

    fn value(&self) -> u32 {
        for axis in Axis::iter() {
            let source = match axis {
                Axis::Row => &self.rows,
                Axis::Col => &self.cols,
            };

            if let Some(res) = Self::_find_reflection(source) {
                // let end = source.len();
                // let slice = match axis {
                //     Axis::Row => res as usize..end,
                //     Axis::Col => 0_usize..(res as usize) + 1,
                // };
                // tprint!(res, &source[slice]);
                return res
                    * match axis {
                        Axis::Row => 100,
                        Axis::Col => 1,
                    };
            }
        }
        panic!("nothing found!");
    }

    fn value2(&self) -> u32 {
        let orig_res = self.value();
        let (orig_axis, orig_res) = match orig_res >= 100 {
            true => (Axis::Row, orig_res / 100),
            false => (Axis::Col, orig_res),
        };
        // tprint!(orig_res, orig_axis);

        for axis in Axis::iter() {
            for replaced in self._sub_off_by_ones(axis) {
                if let Some(res) = Self::_find_reflection(&replaced) {
                    if orig_axis == axis && orig_res == res {
                        continue;
                    }
                    return res
                        * match axis {
                            Axis::Row => 100,
                            Axis::Col => 1,
                        };
                }
            }
        }
        panic!("nothing found!");
    }
    fn _sub_off_by_ones(&self, axis: Axis) -> Vec<Vec<u32>> {
        let source = match axis {
            Axis::Row => &self.rows,
            Axis::Col => &self.cols,
        };

        self.calc_diff_pairs(source)
            .iter()
            .copied()
            .flat_map(|(a, b)| Self::_replace(source, a, b))
            .collect()
    }

    fn _replace(vals: &[u32], a: u32, b: u32) -> Vec<Vec<u32>> {
        vals.iter()
            .copied()
            .enumerate()
            .filter_map(|(i, v)| {
                if !(v == a || v == b) {
                    return None;
                }
                let replacement = match v == a {
                    true => b,
                    false => a,
                };

                let mut cur = Vec::from(vals);
                cur[i] = replacement;
                // println!("{}", "-".repeat(30));
                // tprint!(a, b);
                // println!("orig: {:?}", vals);
                // println!("new : {:?}", cur);
                Some(cur)
            })
            .collect()
    }

    /// get pairs of numbers that are off by one
    fn calc_diff_pairs(&self, v: &[u32]) -> Vec<(u32, u32)> {
        let unique = v
            .iter()
            .copied()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect_vec();

        unique
            .iter()
            .copied()
            .enumerate()
            .flat_map(|(i, x)| {
                unique.iter().copied().skip(i + 1).filter_map(move |y| {
                    match (x ^ y).count_ones() == 1 {
                        true => Some((x, y)),
                        false => None,
                    }
                })
            })
            .collect()
    }

    fn display_diff_pairs(&self) {
        println!("{}", "-".repeat(30));
        println!("rows");
        self.calc_diff_pairs(&self.rows).iter().for_each(|pair| {
            println!("{:?}", pair);
        });
        println!("cols");
        self.calc_diff_pairs(&self.cols).iter().for_each(|pair| {
            println!("{:?}", pair);
        });
        println!();
    }
}

/// helpers
impl Pattern {
    fn _get_axis_hash(data: &[Vec<char>]) -> Vec<u32> {
        data.iter()
            .map(|row| {
                row.iter().fold(0u32, |acc, c| {
                    let num = match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!("unmatched {c}"),
                    };
                    (acc << 1) + num
                })
            })
            .collect()
    }

    fn _find_reflection(source: &[u32]) -> Option<u32> {
        let r = reversed(source);
        if let Some(res) = Self::_find_reflection_helper(source, &r) {
            return Some(res + (source.len() as u32 - res) / 2);
        }

        if let Some(res) = Self::_find_reflection_helper(&r, source) {
            return Some((source.len() as u32 - res) / 2);
        }
        None
    }

    fn _find_reflection_helper(source: &[u32], r: &[u32]) -> Option<u32> {
        let skip = source.len() % 2;
        (0..source.len() - 1).skip(skip).step_by(2).find_map(|i| {
            let found = zip(source.iter().skip(i), r.iter()).all(|(a, b)| a == b);
            match found {
                true => Some(i as u32),
                false => None,
            }
        })
    }
}

fn reversed<T: Clone>(v: &[T]) -> Vec<T> {
    let mut res = v.to_vec();
    res.reverse();
    res
}

fn _get_data(fname: &str) -> Notes {
    Notes::from_str(read_file23(fname).join("\n"))
}