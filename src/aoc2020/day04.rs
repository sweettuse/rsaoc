use std::collections::{HashMap, HashSet};

use crate::utils::read_file20;

pub fn main() {
    // println!("{:?}", read_file20("04.txt.test"));
    let res: i32 = 
        _get_data("04.txt")
        .iter()
        .map(|v| _is_valid(v) as i32)
        .sum() ;
    // println!("{:?}", res)
}

type PassportData = HashMap<String, String>;

fn _get_data(path: &str) -> Vec<PassportData> {
    let mut cur: PassportData = HashMap::new();
    let mut res: Vec<_> = Vec::new();
    for line in read_file20(path) {
        if line.is_empty() {
            res.push(cur);
            cur = HashMap::new();
            continue;
        }
        cur = _update_passport(&line, cur);
    }
    if !cur.is_empty() {
        res.push(cur);
    }
    res
}

/// sample `line`:
/// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
/// byr:1937 iyr:2017 cid:147 hgt:183cm
fn _update_passport(line: &str, mut passport: PassportData) -> PassportData {
    for kv in line.split(' ') {
        let (k, v) = kv.split_once(':').unwrap();
        passport.insert(k.to_string(), v.to_string());
    }
    passport
}

fn _is_valid(p: &PassportData) -> bool {
    let required_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut missing: HashSet<&str> = HashSet::new();
    for field in required_fields {
        missing.insert(field);
    }
    for k in p.keys() {
        missing.remove(k.as_str());
    }
    missing.is_empty()
}
