use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::utils::{print_type_of, read_file22};

pub fn day07() -> (i32, i32) {
    (0, 0)
}

// struct Walker {
//     dirs: Vec<String>,
// }
//
// enum Line {
//     Dir(Dir),
//     File(File),
//     Cmd(Cmd),
// }
//
// struct Dir {
//     name: String,
//     children: Vec<DirOrFile>,
// }
//
// struct File {
//     name: String,
//     size: usize,
// }
//
// struct Cmd {
//     cmd: String,
//     args: Vec<String>,
// }
//
// fn part1() {
//     let dirs: Vec<String> = Vec::new();
// }
//
// fn parse_file(fname: &str) -> Vec<Line> {
//     let data = read_file22(&fname);
//     let mut res = Vec::new();
//
//     res
// }
//
// impl Dir {
//     fn from_str(line: String) -> Self {
//         Dir {
//             name: line.split_whitespace().last().unwrap().to_string(),
//             children: Vec::new(),
//         }
//     }
// }
// impl File {
//     fn from_str(s: &String) -> Self {
//         let (size, name) = s.split_once(' ').unwrap();
//         File{
//             name: name.to_string(),
//             size: size.parse().unwrap(),
//         }
//     }
// }
// impl Cmd {
//     fn from_str(s: &String) -> Self {
//         // let (_, cmd, )
//     }
// }
