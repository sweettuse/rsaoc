use std::{collections::HashMap, num::ParseIntError, error::Error};

use crate::{print1, utils::read_file20};

pub fn main() {
    let data = _get_data("04.txt.test");
    // dbg!(data);
    // dbg!(Parsers::int("1", 1));
    // dbg!(Parsers::int("a", 1));
    // dbg!(Parsers::int("aa", 1));
    // let v = data.first().unwrap();
    // let p = PassportData::from(v);
}

fn part1() -> u32 {
    0
}

#[derive(Debug)]
struct PassportData(HashMap<String, String>);

impl PassportData {
    // fn from(s: impl AsRef<str>) -> Self {
    fn from(s: impl AsRef<str>) -> Self {
        PassportData(
            s.as_ref()
                .split(' ')
                .filter_map(|v| {
                    v.split_once(':')
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                })
                .collect(),
        )
    }
    fn is_valid(&self) -> bool {
        true
    }

    fn byr(&self) -> bool {
        // let t = self.0.get("byr").or();
        true
    }
}

struct Parsers {}

impl Parsers {
    // fn int2(s: &str, exact_len: usize) -> Result<usize, Box<dyn Error>> {
    //     s.parse::<usize>()?
    // }

    // fn int3(s: &str, exact_len: usize) Res{
    //     if s.len() != exact_len {
    //         return Err("wrong len for int2");
    //     }
    //     4usize

    // }

    /// not sure if great way to handle errors, but we're doing it
    fn int(s: &str, exact_len: usize) -> Option<usize> {
        if s.len() != exact_len {
            return None;
        }
        s.parse::<usize>().ok()
    }

    fn height(s: &str) {

    }
}

fn _get_data(path: &str) -> Vec<PassportData> {
    read_file20(path)
        .join("\n") // back to full data
        .split("\n\n") // into chunks
        .map(|s| s.to_string().replace('\n', " "))
        .map(PassportData::from)
        .collect()
}

// fn _get_data(path: &str) -> Vec<PassportData> {
//     let mut cur: PassportData = HashMap::new();
//     let mut res: Vec<_> = Vec::new();
//     for line in read_file20(path) {
//         if line.is_empty() {
//             res.push(cur);
//             cur = HashMap::new();
//             continue;
//         }
//         cur = _update_passport(&line, cur);
//     }
//     if !cur.is_empty() {
//         res.push(cur);
//     }
//     res
// }

// /// sample `line`:
// /// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// /// byr:1937 iyr:2017 cid:147 hgt:183cm
// fn _update_passport(line: &str, mut passport: PassportData) -> PassportData {
//     for kv in line.split(' ') {
//         let (k, v) = kv.split_once(':').unwrap();
//         passport.insert(k.to_string(), v.to_string());
//     }
//     passport
// }
