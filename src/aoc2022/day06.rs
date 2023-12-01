use std::collections::{HashMap, hash_map::Entry, HashSet};
use std::iter::FromIterator;
use std::time::Instant;

use crate::utils::{print_type_of, read_file22};

pub fn day06() -> (usize, usize) {

    let start = Instant::now();
    let _ = (parts1_and_2_crappy(4), parts1_and_2_crappy(100));
    dbg!(Instant::now() - start);

    let start = Instant::now();
    let _ = (parts1_and_2(4), parts1_and_2(100));
    dbg!(Instant::now() - start);

    let start = Instant::now();
    let res = (parts1_and_2_crappy(4), parts1_and_2_crappy(14));
    dbg!(Instant::now() - start);
    res
}

type CharCount = HashMap<char, i32>;

fn parts1_and_2(size: usize) -> usize {
    let data = read_file22("06.txt");
    let line = data.first().unwrap();

    let end_iter = line.chars().skip(size);
    let start_iter = line.chars();
    let mut chars = init_map(&data, size);

    for (i, (start, end)) in start_iter.zip(end_iter).enumerate() {
        if sequence_found(&chars, size) {
            return i + size;
        }

        *chars.entry(end).or_insert(0) += 1;

        chars.entry(start).and_modify(|e| *e -= 1);
        if chars[&start] == 0 {
            chars.remove(&start);
        }
    }

    usize::MAX
}

fn parts1_and_2_crappy(size: usize) -> usize {
    let data = read_file22("06.txt");
    let chars = Vec::from_iter(data.first().unwrap().chars());
    for end in size..chars.len() {
        let hs: HashSet<_> = HashSet::from_iter(&chars[(end - size)..end]);
        if hs.len() == size {
            return end;
        }
    }
    usize::MAX

}

fn init_map(data: &[String], size: usize) -> CharCount {
    let iter = data.first().unwrap().chars();
    let mut chars: CharCount = HashMap::new();
    for c in iter.take(size) {
        *chars.entry(c).or_insert(0) += 1;
    }
    chars
}

fn sequence_found(chars: &CharCount, size: usize) -> bool {
    chars.len() == size
}

