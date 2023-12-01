use color_eyre::eyre::Context;

static BASE_PATH: &str = "/Users/acushner/software/rust/aoc/input/2022/";

fn file_path(fname: &str) -> String {
    BASE_PATH.to_owned() + fname
}

use itertools::Itertools;

pub fn day01() -> color_eyre::Result<()> {
    part1()
}

pub fn part1() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string(file_path("01.txt"))?;

    let max = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v)
            }
            sum
        })
        .max();

    println!("heaviest elf: {max:?}");

    Ok(())
}

struct GroupSumIter<I> {
    inner: I,
}

impl<I> Iterator for GroupSumIter<I>
where
    I: Iterator<Item = Option<u64>>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum: Self::Item = 0;
        let mut set = false;
        while let Some(Some(v)) = self.inner.next() {
            sum += v;
            set = true;
        }
        if set {
            Some(sum)
        } else {
            None
        }
    }
}

// with custom iterator
pub fn day01_06() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string(file_path("01.txt"))?;

    let lines = input.lines().map(|v| v.parse::<u64>().ok());

    let heaviest_elf = GroupSumIter { inner: lines }.max();

    println!("most stuff: {heaviest_elf:?}");

    Ok(())
}

// fewer `collect` calls
pub fn day01_05() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = std::fs::read_to_string(file_path("01.txt"))?;

    let lines = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let heaviest_elf = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u64>())
        .max();

    println!("most stuff: {heaviest_elf:?}");

    Ok(())
}

pub fn day01_04() -> color_eyre::Result<()> {
    // https://fasterthanli.me/series/advent-of-code-2022/part-1
    color_eyre::install()?;
    let input = std::fs::read_to_string(file_path("01.txt"))?;
    let lines = input
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .collect::<Vec<_>>();

    let groups = lines.split(|v| v.is_none()).collect::<Vec<_>>();

    println!("groups: {groups:?}");

    // todo!("left off at \"And we can find the maximum here without even collecting\"");

    Ok(())
}

pub fn day01_3() -> color_eyre::Result<()> {
    // https://fasterthanli.me/series/advent-of-code-2022/part-1
    // let data = read_input().unwrap();
    color_eyre::install()?;
    let input = std::fs::read_to_string(file_path("01.txt"))?;
    let lines = input.lines().collect::<Vec<_>>();
    let groups = lines.split(|l| l.is_empty()).collect::<Vec<_>>();
    let groups = groups
        .into_iter()
        .map(|g| g.iter().map(|v| v.parse::<u64>().ok()))
        .collect::<Vec<_>>();

    println!("groups: {groups:?}");

    Ok(())
}

pub fn day01_2() -> color_eyre::Result<()> {
    // https://fasterthanli.me/series/advent-of-code-2022/part-1
    color_eyre::install()?;
    let input = std::fs::read_to_string(file_path("01.txt"))?;

    for group in input.split("\n\n") {
        println!("GROUP");
        for l in group.lines() {
            println!(" - {l}");
        }
    }

    Ok(())
}
pub fn day01_1() -> color_eyre::Result<()> {
    // https://fasterthanli.me/series/advent-of-code-2022/part-1
    // let data = read_input().unwrap();
    color_eyre::install()?;
    let input = std::fs::read_to_string(file_path("01.txt"))?;

    let mut lines = input.lines();
    // while let Some(line) = lines.next() {
    //     println!("Got 'em {}", line)
    // }
    // which becomes:

    for line in lines {
        println!("Got 'em {}", line)
    }

    Ok(())
}

pub fn intro() {
    pub fn day01() -> color_eyre::Result<()> {
        // https://fasterthanli.me/series/advent-of-code-2022/part-1
        // let data = read_input().unwrap();
        color_eyre::install()?;
        let _data = read_color_eyre_input()?;
        Ok(())
    }

    struct PathedIoErr {
        path: String,
        inner: std::io::Error,
    }

    // how to implement a trait for a struct
    // alternately could use #[derive(Debug)]
    impl std::fmt::Debug for PathedIoErr {
        // fn _fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //     f.debug_struct("PathedIoErr")
        //         .field("path", &self.path)
        //         .field("inner", &self.inner)
        //         .finish()
        // }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "for file {:?}: {}", self.path, self.inner)
        }
    }

    fn read_input() -> Result<String, PathedIoErr> {
        let path = "jeb/tuse.txt";
        match std::fs::read_to_string(path) {
            Ok(s) => Ok(s),
            Err(e) => Err(PathedIoErr {
                path: path.into(),
                inner: e,
            }),
        }
    }

    // crate with better output formatting
    fn read_fs_err_input() -> Result<String, std::io::Error> {
        fs_err::read_to_string("failed/file.txt")
    }

    // colorful errors?
    fn read_color_eyre_input() -> color_eyre::Result<String> {
        let path = "not/here.txt";
        let input = std::fs::read_to_string(path).wrap_err("reading ".to_owned() + path)?;
        Ok(input)
    }
}
