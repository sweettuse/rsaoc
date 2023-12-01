use std::collections::HashMap;

use crate::utils::read_file23;
use crate::print1;

pub fn main() -> (u32, u32) {
    (part1("01.txt"), part2("01.txt"))
}
fn part1(path: &str) -> u32 {
    let data = read_file23(path);
    data.iter().map(|s| get_calibration_value1(s)).sum()
}

fn part2(path: &str) -> u32 {
    let data = read_file23(path);
    data.iter().map(|s| get_calibration_value2(s)).sum()
}

fn get_calibration_value1(s: &str) -> u32 {
    let mut chars: Vec<char> = s.chars().collect();
    let mut vals: Vec<u32> = Vec::new();

    for _ in 0..2 {
        for c in chars.iter() {
            if let Some(val) = c.to_digit(10) {
                vals.push(val);
                break;
            }
        }
        chars.reverse();
    }
    if let [a, b] = vals[..2] {
        return a * 10 + b;
    }
    panic!()
}


type NumberMap = HashMap<&'static str, u8>;

fn get_number_hash_map() -> NumberMap {
    let mut res: NumberMap = HashMap::new();
    res.insert("one", 1);
    res.insert("two", 2);
    res.insert("three", 3);
    res.insert("four", 4);
    res.insert("five", 5);
    res.insert("six", 6);
    res.insert("seven", 7);
    res.insert("eight", 8);
    res.insert("nine", 9);
    res.insert("0", 0);
    res.insert("1", 1);
    res.insert("2", 2);
    res.insert("3", 3);
    res.insert("4", 4);
    res.insert("5", 5);
    res.insert("6", 6);
    res.insert("7", 7);
    res.insert("8", 8);
    res.insert("9", 9);
    res
}

fn get_rev_number_hash_map() -> NumberMap {
    let mut res: NumberMap = HashMap::new();
    res.insert("7", 7);
    res.insert("neves", 7);
    res.insert("xis", 6);
    res.insert("eerht", 3);
    res.insert("2", 2);
    res.insert("owt", 2);
    res.insert("8", 8);
    res.insert("thgie", 8);
    res.insert("0", 0);
    res.insert("enin", 9);
    res.insert("9", 9);
    res.insert("3", 3);
    res.insert("1", 1);
    res.insert("4", 4);
    res.insert("5", 5);
    res.insert("6", 6);
    res.insert("ruof", 4);
    res.insert("evif", 5);
    res.insert("eno", 1);
    res
}


fn _flip_str(s: &str) -> String {
    s.chars().rev().collect()
}

fn get_calibration_value2(s: &str) -> u32 {
    let a = 10 * _get_cv_helper(s, get_number_hash_map());
    let b = _get_cv_helper(_flip_str(s).as_str(), get_rev_number_hash_map());
    a + b
    
    
}

fn _get_cv_helper(s: &str, number_map: NumberMap) -> u32 {
    if let Some((_, cv)) = number_map
        .iter()
        .map(|(number_name, number_val)| (s.find(number_name), number_val))
        .filter_map(|(idx, val)| idx.map(|v| (v, val)))
        .min()
    {
        return *cv as u32;
    }
    panic!()
}

// fn get_number_names() -> Vec<String> {
//     [
//         "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1", "2", "3",
//         "4", "5", "6", "7", "8", "9",
//     ]
//     .iter()
//     .map(|s| s.to_string())
//     .collect()
// }