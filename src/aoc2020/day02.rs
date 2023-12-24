use std::str::FromStr;
use crate::utils::read_file20;

pub fn main() -> (i32, i32) {
    // let p: Result<Password, _> = s.parse();
    // (1, 2)
    // play();
    (parts1_and_2(Password::is_valid), parts1_and_2(Password::is_valid2))
}

fn play() {
    let s = "3-11 z: zzzzzdzzzzlzz";
    let p: Password = s.parse().unwrap();
    println!("{:?}, {}", p, p.is_valid());
    println!("{:?}, {}", p, p.is_valid2());
 }

fn parts1_and_2<F: Fn(&Password) -> bool>(f: F) -> i32 {
    _get_data("02.txt").iter().map(|v| f(v) as i32).sum()
}

#[derive(Debug)]
struct Password {
    min: i32,
    max: i32,
    c: char,
    password: String,
}

/// take a string like 2-14 and parse it into a tuple of ints
fn parse_range(s: &str) -> (i32, i32) {
    let mut s = s.split('-');

    fn _helper(t: Option<&str>) -> i32 {
        t.unwrap().parse().unwrap()
    }

    (_helper(s.next()), _helper(s.next()))
}

/// sample data:
/// 3-11 z: zzzzzdzzzzlzz
/// 3-7 x: xjxbgpxxgtx
/// 3-4 v: vvmv
/// 3-5 t: tgkfq
impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(' ');
        let range = s.next().unwrap();
        let c = s.next().unwrap().chars().nth(0).unwrap();
        let pw = s.next().unwrap();
        let (min, max) = parse_range(range);


        Ok(Self { min, max, c, password: pw.to_string()})
    }

}

impl Password {
    pub fn is_valid(&self) -> bool { 
        let count = self.num_chars();
        (self.min <= count) && (count <= self.max)
    }
    pub fn is_valid2(&self) -> bool {
        let bytes = self.password.as_bytes();

        let mut _valid = |n: i32| -> i32 {
            let n = n as usize;
            if let Some(found) = bytes.get(n - 1) {
                return ((*found as char) == self.c) as i32
            }
            0
        };

        (_valid(self.min) + _valid(self.max)) == 1
    }

    fn num_chars(&self) -> i32 {
        self.password.chars().map(|v| (v == self.c) as i32).sum()
    }
}

fn _get_data(fname: &str) -> Vec<Password> {
    read_file20(fname).iter().map(|v| v.parse().unwrap()).collect()
}