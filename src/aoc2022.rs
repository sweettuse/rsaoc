use std::cmp::Reverse;

use crate::utils::{read_file22, to_int_vec};

pub mod fasterthanlime;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub fn day01() -> (i32, i32) {
    let lines = read_file22("01.txt");
    let groups: Vec<_> = lines.split(|val| val.is_empty()).collect();

    let mut groups: Vec<i32> =  groups.iter().map(|v| to_int_vec(&v).iter().sum()).collect();

    let part1 = *groups.iter().max().unwrap();

    groups.sort_by_key(|v| Reverse(*v));

    (part1, groups[..3].iter().sum())
}

