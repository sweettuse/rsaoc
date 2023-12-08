use crate::utils::read_file23;

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
    let lines = read_file23(fname);
}
