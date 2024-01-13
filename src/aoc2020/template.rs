use crate::utils::read_file20;

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    Err("unsolved".to_string())
}

fn part2() -> AocRes {
    Err("unsolved".to_string())
}

fn _get_data(fname: &str) {
    let _lines = read_file20(fname);
}
