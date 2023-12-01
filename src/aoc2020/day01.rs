use crate::utils::read_file20;
use std::collections::HashSet;

pub fn main() -> (Option<i32>, Option<i32>) {
    (part1(), part2())
}

fn part1() -> Option<i32> {
    let data = _get_data("01.txt");
    for v in &data {
        if data.contains(&(2020 - v)) {
            return Some(v * (2020 - v));
        }
    }
    None
}

fn part2() -> Option<i32> {
    let data = _get_data("01.txt");
    for v1 in &data {
        for v2 in &data {
            let target = 2020 - v1 - v2;
            if data.contains(&target) {
                return Some(v1 * v2 * target);
            }
        }
    }
    None
}

fn _get_data(fname: &str) -> HashSet<i32> {
    let numbers = read_file20(fname);
    numbers
        .iter()
        .map(|v| v.parse().expect("integer"))
        .collect()
}
