use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;

use crate::{tprint, utils::read_file20};

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    Ok(_get_data("04.txt")
        .iter()
        .map(|v| _is_valid(v) as i32)
        .sum())
}

fn part2() -> AocRes {
    Ok(_get_data("04.txt")
        .iter()
        .map(|v| _is_valid2(v) as i32)
        .sum())
}

type PassportData = HashMap<String, String>;

fn _get_data(path: &str) -> Vec<PassportData> {
    let mut cur: PassportData = PassportData::new();
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

const REQUIRED_FIELDS: Lazy<Vec<&'static str>> =
    Lazy::new(|| vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);

const VALID_EYE_COLORS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    hashset! {
        "amb",
        "blu",
        "brn",
        "gry",
        "grn",
        "hzl",
        "oth",
    }
});

fn _is_valid(p: &PassportData) -> bool {
    let mut missing: HashSet<&str> = HashSet::new();
    for field in &*REQUIRED_FIELDS {
        missing.insert(field);
    }
    for k in p.keys() {
        missing.remove(k.as_str());
    }
    missing.is_empty()
}

fn _is_valid2(p: &PassportData) -> bool {
    _is_valid(p)
        && VALIDATORS.iter().all(|(k, f)| match p.get(&k.to_string()) {
            Some(v) => f(v),
            None => true,
        })
}

type Validator = Box<dyn Fn(&str) -> bool + Send + Sync>;

fn _create_validators<'a>() -> HashMap<&'a str, Validator> {
    let byr = _create_range_validator(1920, 2002);
    let iyr = _create_range_validator(2010, 2020);
    let eyr = _create_range_validator(2020, 2030);
    let hgt_cm = _create_range_validator(150, 193);
    let hgt_in = _create_range_validator(59, 76);
    let hgt = Box::new(move |s: &str| {
        let split_point = s.len() - 2;
        let (start, suffix) = s.split_at(split_point);
        if suffix == "cm" {
            return hgt_cm(start);
        }
        if suffix == "in" {
            return hgt_in(start);
        }
        false
    });
    let hcl = Box::new(|s: &str| {
        s.starts_with('#') && s.len() == 7 && u32::from_str_radix(&s[1..], 16).is_ok()
    });
    let ecl = Box::new(|s: &str| VALID_EYE_COLORS.contains(s));
    let pid = Box::new(|s: &str| s.len() == 9 && s.parse::<u64>().is_ok());
    let cid = Box::new(|_s: &str| true);
    hashmap! {
        "byr" => byr,
        "iyr" => iyr,
        "eyr" => eyr,
        "hgt" => hgt,
        "hcl" => hcl,
        "ecl" => ecl,
        "pid" => pid,
        "cid" => cid,
    }
}

static VALIDATORS: Lazy<HashMap<&'static str, Validator>> = Lazy::new(|| _create_validators());

fn _create_range_validator(inclusive_min: i32, inclusive_max: i32) -> Validator {
    Box::new(move |s: &str| match s.parse::<i32>() {
        Ok(v) => inclusive_min <= v && v <= inclusive_max,
        Err(_) => false,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_validator() {
        let f = _create_range_validator(2, 12);
        assert!(f("2"));
        assert!(f("12"));
        assert!(f("3"));
        assert!(!f("abc"));
        assert!(!f("1"));
        assert!(!f("13"));
    }

    #[test]
    fn test_validators() {
        let validators = _create_validators();
        assert!(validators["iyr"]("2010"));
        assert!(!validators["iyr"]("2030"));
        assert!(!validators["hgt"]("2030"));
        assert!(!validators["hgt"]("abcd"));
        assert!(!validators["hgt"]("ABCcm"));
        assert!(validators["hgt"]("160cm"));
        assert!(!validators["hgt"]("140cm"));
        assert!(validators["ecl"]("amb"));
        assert!(!validators["ecl"]("notvalid"));
        assert!(validators["pid"]("000000000"));
        assert!(validators["pid"]("000000001"));
        assert!(!validators["pid"]("00000001"));
        assert!(!validators[&String::from("pid").as_str()]("00000001"));
    }
}
