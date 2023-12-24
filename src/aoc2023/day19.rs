use crate::utils::read_file23;

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    Err("python".to_string())
}

fn part2() -> AocRes {
    Err("python".to_string())
}

fn _get_data(fname: &str) {
    let _lines = read_file23(fname);
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[cfg(test)]
mod test {
    use super::*;

}