
pub mod utils;
pub mod aoc2022;
pub mod aoc2020;
pub mod aoc2023;
pub use fs_err;

macro_rules! run20 {
    ($day:ident) => {
        println!("aoc2020: {:?}: {:?}", stringify!($day), aoc2020::$day::main());
    };
}

macro_rules! run {
    ($day:ident) => {
        println!("aoc2023: {:?}: {:?}", stringify!($day), aoc2023::$day::main());
    };
}

fn main() {
    run_aoc2020();
    run_aoc2023();
    println!("============================");
}

fn run_aoc2023() {
    println!("============2023============");
    run!(day01);
}

fn run_aoc2020() {
    println!("============2020============");
    run20!(day01);
    run20!(day02);
    run20!(day03);
    run20!(day04);
}
// fn main_orig() {
//     println!("Hello, world!");
//     println!("day 01 {:?}", aoc2022::day01());
//     println!("day 02 {:?}", aoc2022::day02::day02());
//     println!("day 03 {:?}", aoc2022::day03::day03());
//     println!("day 04 {:?}", aoc2022::day04::day04());
//     println!("day 05 {:?}", aoc2022::day05::day05());
//     println!("day 06 {:?}", aoc2022::day06::day06());
//     println!("day 07 {:?}", aoc2022::day07::day07());
//     println!("====================");
//     println!("ftl: day 01 {:?}", aoc2022::fasterthanlime::day01::day01());
//     println!("ftl: day 02 {:?}", aoc2022::fasterthanlime::day02::day02());

// }


