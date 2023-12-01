use crate::utils::read_file22;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn day03() -> (i32, i32) {
    (part1(), part2())
}

#[derive(Debug)]
struct Sack {
    comp1: Vec<char>,
    comp2: Vec<char>,
}

impl Sack {
    fn from_str(s: &str) -> Self {
        let mid = s.chars().count() / 2;
        let chars: Vec<_> = s.chars().collect();
        Sack {
            comp1: chars[..mid].to_vec(),
            comp2: chars[mid..].to_vec(),
        }
    }

    fn duped_char(&self) -> char {
        let s1: HashSet<_> = self.comp1.clone().into_iter().collect();
        let s2: HashSet<_> = self.comp2.clone().into_iter().collect();
        let inter: Vec<&char> = s1.intersection(&s2).collect();
        if inter.len() == 0 {
            panic!("nothing duped!");
        } else if inter.len() > 1 {
            panic!("more than 1 thing duped!");
        }
        **inter.get(0).unwrap()
    }
}

fn parse_file(fname: &str) -> Vec<Sack> {
    read_file22(fname).iter().map(|s| Sack::from_str(s)).collect()
}

fn part1() -> i32 {
    let s = Sack::from_str("123345");
    s.duped_char();
    let res = parse_file("03.txt");
    let priorities = letter_priorities();
    res.iter().map(|sack| priorities[&sack.duped_char()]).sum()

    // dbg!(&s.duped_value());
    // dbg!(&priorities);
}

fn part2() -> i32 {
    let data = parse_file("03.txt");
    let priorities = letter_priorities();
    let mut total = 0;
    for group in data.chunks(3) {
        let mut group_chars: Vec<HashSet<char>> = Vec::new();
        for g in group {
            let cur: HashSet<char> =
                HashSet::from_iter(g.comp1.iter().chain(&g.comp2).copied());
            group_chars.push(cur);
        }

        let mut res: Option<HashSet<char>> = None;
        for s in group_chars {
            res = match res {
                None => Some(s),
                Some(v) => Some(v.intersection(&s).copied().collect()),
            };
        }
        let c = *res
            .unwrap()
            .iter()
            .next()
            .unwrap();
        total += priorities[&c];
    }
    total
}

fn letter_priorities() -> HashMap<char, i32> {
    let mut map: HashMap<_, _> = HashMap::new();
    let count = 1..;
    let chars = 'a'..='z';
    for (i, c) in count.zip(chars.chain('A'..='Z')) {
        map.insert(c, i);
    }
    map
}
