use std::collections::HashMap;

use crate::utils::read_file20;

pub fn main() -> (i32, i64) {
    (part1(), part2())
}

fn part1() -> i32 {
    calc_trees("03.txt", 3, 1)
}

fn part2() -> i64 {
    let slopes = vec![
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];

    let mut res: i64 = 1;
    for (right, down) in slopes {
        res *= calc_trees("03.txt", right, down) as i64;
    }
    res
}

fn calc_trees(fname: &str, right: usize, down: usize)  -> i32 { 
    let mut r = down;
    let mut c = right;
    let treemap = TreeMap::from_fname(fname);

    let mut count = 0;
    while let Some(b) = treemap.at(r, c) {
        if *b == Terrain::Tree {
            count += 1;
        }
        r += down;
        c += right;
    }
    count
}

#[derive(Debug, PartialEq)]
enum Terrain {
    Open,
    Tree,
}

impl Terrain {
    pub fn from_char(c: char) -> Self {
        if c == '#' {
            return Self::Tree;
        } else if c == '.'  {
            return Self::Open;
        }
        panic!();
    }
} 

#[derive(Debug)]
struct TreeMap {
    layout: HashMap<(usize, usize), Terrain>,
    max_c: usize,
}

impl TreeMap {
    pub fn at(&self, r: usize, c: usize) -> Option<&Terrain> {
        self.layout.get(&(r, c % self.max_c))
    }

    pub fn from_fname(fname: &str) -> Self {
        let data = read_file20(fname);
        let mut res: HashMap<_, _> = HashMap::new();
        let mut max_c = 0;


        for (r, row) in data.iter().enumerate() {
            max_c = row.len();
            for (c, chr) in row.chars().enumerate() {
                res.insert((r, c), Terrain::from_char(chr));
            }
        }
        TreeMap { layout: res, max_c }
    }
}

