use crate::utils::{print_type_of, read_file22};

// assignment range
#[derive(Debug)]
struct ARange {
    start: i32,
    end: i32,
}

impl ARange {
    fn from_str(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        ARange {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }

    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlap(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.start
            || self.start <= other.end && self.end >= other.end
            || other.start <= self.start && self.end <= other.end
    }
}

// assignment pair
#[derive(Debug)]
struct APair {
    l: ARange,
    r: ARange,
}

impl APair {
    fn from_str(s: &str) -> Self {
        let (l, r) = s.split_once(',').unwrap();

        Self {
            l: ARange::from_str(l),
            r: ARange::from_str(r),
        }
    }

    fn fully_contains(&self) -> bool {
        self.l.fully_contains(&self.r) || self.r.fully_contains(&self.l)
    }

    fn overlap(&self) -> bool {
        self.l.overlap(&self.r)
    }
}

fn parse_file(fname: &str) -> Vec<APair> {
    let data = read_file22(&fname);
    data.iter().map(|s| APair::from_str(&s)).collect()
}

pub fn day04() -> (i32, i32) {
    (
        parts1_and_2(APair::fully_contains),
        parts1_and_2(APair::overlap),
    )
}

fn parts1_and_2(func: fn(&APair) -> bool) -> i32 {
    let pairs = parse_file("04.txt");
    pairs.iter().map(|p| func(&p) as i32).sum()
}
