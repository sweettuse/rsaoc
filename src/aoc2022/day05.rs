use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::utils::{print_type_of, read_file22};

pub fn day05() -> (String, String) {
    (part1(), part2())
}

fn part1() -> String {
    parts1_and_2(false)
}

fn part2() -> String {
    parts1_and_2(true)
}

fn get_tops(map: &HashMap<i32, Stack>) -> String {
    let mut res = String::new();
    for idx in 1.. {
        let c = match map.get(&idx) {
            Some(s) => s.data.last(),
            None => break,
        };
        if let Some(c) = c {
            res.push(*c);
        }
    }
    res
}


fn parts1_and_2(reversed: bool) -> String {
    let (mut stacks, instructions) = parse_file();
    for inst in instructions {
        // dbg!("====================");
        // dbg!("");
        // dbg!(&stacks);
        let from = stacks.get_mut(&inst.from).unwrap();
        let to_move = from.pop(inst.count, reversed);
        // dbg!(&inst, &to_move);
        let to = stacks.get_mut(&inst.to).unwrap();
        to.push_many(to_move);
    }
    // dbg!(&stacks);
    // dbg!(Inst::from_str("_ 1 _ 3 _ 5"));
    get_tops(&stacks)

}

fn parse_file() -> (HashMap<i32, Stack>, Vec<Inst>){
    let data = read_file22("05.txt");
    let mut parts = data.split(|v| v == "");

    (
        parse_stack(parts.next().unwrap()),
        Inst::parse_instructions(parts.next().unwrap())
    )
    // let iter = data.iter();
    // let stack = parse_stack(&iter);

}

fn parse_stack(vec: &[String]) -> HashMap<i32, Stack> {
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3

    let mut res: HashMap<i32, Stack> = HashMap::new();
    let chars: HashSet<char> = HashSet::from_iter('A'..='Z');

    // for l in read_file22("05.txt.test") {
    for l in vec {
        for (i, c) in l.chars().enumerate() {
            if !chars.contains(&c) {
                continue;
            }
            let idx = (i / 4 + 1) as i32;
            if !res.contains_key(&idx) {
                res.insert(idx, Stack::new());
            }
            let vec = res.get_mut(&idx).unwrap();
            vec.push(c);
        }
    }

    // dbg!(&res);
    for s in res.values_mut() {
        s.reverse();
    }
    res
}

#[derive(Debug)]
struct Stack {
    data: Vec<char>,
}

impl Stack {
    fn pop(&mut self, n: i32, reversed: bool) -> Vec<char> {
        let mut res: Vec<char> = Vec::new();
        for _ in 0..n {
            res.push(
                match self.data.pop() {
                    Some(v) => v,
                    None => break,
                }
            );
        }
        if reversed {
            res.reverse();
        }
        res
    }

    fn reverse(&mut self) {
        self.data.reverse();
    }

    fn push(&mut self, c: char) {
        self.data.push(c);
    }

    fn push_many(&mut self, chars: Vec<char>) {
        for c in chars {
            self.push(c);
        }
    }

    fn new() -> Self {
        Stack { data: Vec::new() }
    }

}


#[derive(Debug)]
struct Inst {
    count: i32,
    from: i32,
    to: i32,
}

impl Inst {
    fn from_str(s: &str) -> Self {
        // move 1 from 2 to 1

        let vals: Vec<i32> = s
            .split_whitespace()
            .clone()
            .skip(1)
            .step_by(2)
            .map(|v| v.parse().unwrap())
            .collect();

        Inst {
            count: vals[0],
            from: vals[1],
            to: vals[2],
        }
    }

    fn parse_instructions(vec: &[String]) -> Vec<Self> {
        vec.iter()
            .map(|s| Inst::from_str(&s))
            .collect()
    }
}
