use std::collections::HashSet;

use itertools::Itertools;

use crate::{tprint, utils::read_file20};

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    Ok(_get_data("05.txt")
        .iter()
        .map(_parse_boarding_pass)
        .max()
        .unwrap())
}

fn part2() -> AocRes {
    let nums = _get_data("05.txt")
        .iter()
        .map(_parse_boarding_pass)
        .collect::<HashSet<u32>>();
    let min_row = nums.iter().map(|v| v >> 3).min().unwrap() + 1;
    let max_row = nums.iter().map(|v| v >> 3).max().unwrap() - 1;
    let (min_id, max_id) = (min_row << 3, (max_row << 3) + 7);
    let res = (min_id + 1..max_id).find(|id| {
        nums.contains(&(id - 1))
            && !nums.contains(id)
            && nums.contains(&(id + 1))
            && nums.contains(&(id - 8))
            && nums.contains(&(id + 8))
    });
    Ok(res.unwrap())
    // Ok(res.unwrap())
}

fn _get_data(fname: &str) -> Vec<String> {
    read_file20(fname)
}

fn _parse_boarding_pass(s: impl AsRef<str>) -> u32 {
    let s = s.as_ref().replace(['F', 'L'], "0").replace(['B', 'R'], "1");
    u32::from_str_radix(&s, 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(567, _parse_boarding_pass("BFFFBBFRRR"));
    }
}
