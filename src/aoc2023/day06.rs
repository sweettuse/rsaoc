use itertools::Itertools;

use crate::{print1, utils::read_file23};

pub fn main() -> (i64, i64) {
    (part1("06.txt"),
    part2("06.txt"))
}

fn part1(fname: &str) -> i64 {
    let lines = read_file23(fname);
    let (l1, l2) = lines.iter().collect_tuple().unwrap();
    let (times, distances) = (_parse_ints(l1), _parse_ints(l2));
    _calc_total_ways(&times, &distances)
}

fn part2(fname: &str) -> i64 {
    let data = read_file23(fname);
    let (l1, l2) = data.iter().collect_tuple().unwrap();
    _calc_ways(_get_part2_int(l1), _get_part2_int(l2))
}

fn _get_part2_int(l: &str) -> i64 {
    let (_, nums) = l.split_once(':').unwrap();
    nums.replace(' ', "").parse::<i64>().unwrap()
}

fn _calc_total_ways(times: &[i64], distances: &[i64]) -> i64 {
    times
        .iter()
        .copied()
        .zip(distances.iter().copied())
        .map(|(t, d)| _calc_ways(t, d))
        .product::<i64>()
}

fn _calc_ways(t: i64, d: i64) -> i64 {
    let (min, max) = _calc_roots(t, d);
    let offset = _get_ends_offset(t, min, d) + _get_ends_offset(t, max, d);
    max - min + 1 - offset
}

fn _get_ends_offset(t_total: i64, t_charge: i64, dist: i64) -> i64 {
    let v = -(t_charge.pow(2) - t_total * t_charge);
    match v > dist {
        true => 0,
        _ => 1,
    }
}

/// distance = (t_total - t_charge) * t_charge
/// t_charge ** 2 - t_total * t_charge + distance
fn _calc_roots(t: i64, d: i64) -> (i64, i64) {
    let (r1, r2) = _calc_roots_float(t, d);
    (r1.min(r2).ceil() as i64, r1.max(r2).floor() as i64)
}

fn _calc_roots_float(t: i64, d: i64) -> (f64, f64) {
    let inner = ((t.pow(2) - 4 * d) as f64).sqrt();
    let t = t as f64;

    let r1 = (t + inner) / 2.;
    let r2 = (t - inner) / 2.;

    (r1.min(r2), r1.max(r2))
}


fn _parse_ints(s: &str) -> Vec<i64> {
    s.split(' ').filter_map(|v| v.parse::<i64>().ok()).collect()
}
