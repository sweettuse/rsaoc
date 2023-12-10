use std::iter::repeat;

use itertools::{unfold, Itertools};

use crate::{print1, utils::read_file23};

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let data = _get_data("09.txt");
    Ok(data.iter().map(|d| extrapolate(d, _calc_next)).sum())
}

fn part2() -> AocRes {
    let data = _get_data("09.txt");
    Ok(data.iter().map(|d| extrapolate(d, _calc_prev)).sum())
}

// extrapolate the next value in the series
fn extrapolate<F>(nums: &[i32], extrap_fn: F) -> i32
where
    F: Fn(Vec<Vec<i32>>) -> i32,
{
    let mut all_diffs = vec![Vec::from(nums)];
    while let Some(diffs) = _get_diffs(all_diffs.last().unwrap()) {
        all_diffs.push(diffs);
    }
    extrap_fn(all_diffs)
}

fn _calc_next(nums: Vec<Vec<i32>>) -> i32 {
    nums.iter().map(|v| v.last().unwrap()).sum()
}

fn _calc_prev(nums: Vec<Vec<i32>>) -> i32 {
    let signs = repeat(vec![1, -1]).flatten().cycle();
    nums.iter()
        .zip(signs)
        .map(|(nums, sign)| nums.first().unwrap() * sign)
        .sum()
}

// get diffs between adjacent numbers
// return None if all vals are 0 else return the result
fn _get_diffs(nums: &Vec<i32>) -> Option<Vec<i32>> {
    let res: Vec<i32> = nums
        .as_slice()
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect();

    match res.iter().any(|v| *v != 0) {
        true => Some(res),
        _ => None,
    }
}

// parse lines to, e.g. [[1, 2, 3], [4, 5, 6]]
fn _get_data(fname: &str) -> Vec<Vec<i32>> {
    read_file23(fname)
        .iter()
        .map(|line| {
            line.split(' ')
                .map(|num_str| num_str.parse().unwrap())
                .collect()
        })
        .collect()
}
