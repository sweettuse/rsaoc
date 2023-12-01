use std::fs;

const BASE_PATH: &str = "/Users/acushner/software/rust/rsaoc/input";

#[macro_export]
macro_rules! print1 {
    ($v:expr) => {
        println!("'{}': {:?}", stringify!($v), $v);
    };
}

// read file and split by lines
pub fn read_file(path: &str, year: u16) -> Vec<String> {
    let contents = fs::read_to_string(_full_path(path, year)).expect("file read");
    contents.lines().map(String::from).collect()
}
pub fn read_file20(path: &str) -> Vec<String> {
    read_file(path, 2020)
}

pub fn read_file22(path: &str) -> Vec<String> {
    read_file(path, 2022)
}

pub fn read_file23(path: &str) -> Vec<String> {
    read_file(path, 2023)
}


pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn to_int_vec(group: &[String]) -> Vec<i32> {
    group.iter().map(|v| v.parse().expect("integer")).collect()
}

fn _full_path(suffix: &str, year: u16) -> String {
    format!("{}/{}/{}", BASE_PATH, year, suffix)
}
