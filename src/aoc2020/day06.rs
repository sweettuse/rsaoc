use std::collections::HashSet;

use itertools::Itertools;

use crate::utils::read_file20;

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    Ok(_get_data("06.txt").split("\n\n").map(_parse_group).sum())
}

fn part2() -> AocRes {
    Ok(_get_data("06.txt").split("\n\n").map(_parse_group2).sum())
}

fn _get_data(fname: &str) -> String {
    read_file20(fname).join("\n")
}

/// how many questions were answered yes to
fn _parse_group(s: impl AsRef<str>) -> u32 {
    let res = s
        .as_ref()
        .split('\n')
        .flat_map(|l| l.chars())
        .collect::<HashSet<char>>();
    res.len() as u32
}

/// how many questions were _all_ answered yes to
fn _parse_group2(s: impl AsRef<str>) -> u32 {
    let lines = s.as_ref().split('\n').collect_vec();
    if lines.is_empty() {
        return 0;
    }
    let res = "abcdefghijklmnopqrstuvwxyz".chars().collect::<HashSet<_>>();
    let all_yes = lines.iter().fold(res, |acc, v| {
        acc.intersection(&v.chars().collect::<HashSet<_>>())
            .cloned()
            .collect()
    });
    all_yes.len() as u32
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_group() {
        let s = "ab\nac";
        assert_eq!(_parse_group(s), 3);
    }
}
