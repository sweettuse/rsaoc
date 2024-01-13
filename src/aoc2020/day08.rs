use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_file20;

pub type AocRes = Result<i32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let mut m = _get_data("08.txt");
    loop {
        if let Some(res) = m.next() {
            return Ok(res);
        }
    }
}

fn part2() -> AocRes {
    let m = _get_data("08.txt");
    let res = (0..m.instructions.len()).find_map(|i| {
        if m.instructions[i].type_ == InstructionType::Acc {
            return None;
        }

        let res: i32;
        let mut local_m = m.clone();
        let cur_i = &mut local_m.instructions[i];
        cur_i.type_ = match cur_i.type_ {
            InstructionType::Nop => InstructionType::Jmp,
            InstructionType::Jmp => InstructionType::Nop,
            InstructionType::Acc => panic!(),
        };

        loop {
            if let Some(val) = local_m.next() {
                res = val;
                break;
            }
        }

        match local_m.called_twice() {
            true => None,
            false => Some(res),
        }

    });
    Ok(res.unwrap())
}

fn _get_data(fname: &str) -> Machine{
    Machine::from_str(read_file20(fname).join("\n"))
}

#[derive(Debug, Clone)]
struct Machine {
    acc: i32,
    instructions: Vec<Instruction>,
    inst_pointer: usize,
    visited: HashMap<usize, u32>,
}


impl Machine {
    fn from_str(s: impl AsRef<str>) -> Self {
        let instructions = s.as_ref().split('\n').map(Instruction::from_str).collect_vec();
        Self {
            acc: 0,
            instructions,
            inst_pointer: 0,
            visited: hashmap! {},
        }
    }
    fn next(&mut self) -> Option<i32> {
        *self.visited.entry(self.inst_pointer).or_default() += 1;

        if self.called_twice() || self.inst_pointer >= self.instructions.len() {
            return Some(self.acc)
        }

        let inst = &self.instructions[self.inst_pointer];
        let offset = match inst.type_ {
            InstructionType::Nop => 1,
            InstructionType::Acc => {
                self.acc += inst.offset;
                1
            },
            InstructionType::Jmp => inst.offset,
        };
        self.inst_pointer = (self.inst_pointer as i32 + offset) as usize;
        None
    }

    fn called_twice(&self) -> bool {
        if let Some(val) = self.visited.get(&self.inst_pointer) {
            return *val > 1;
        }
        false
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    type_: InstructionType,
    offset: i32,
}

impl Instruction {
    fn from_str(s: impl AsRef<str>) -> Self {
        let (inst, offset) = s.as_ref().split_once(' ').unwrap();
        Self {
            type_: InstructionType::from_str(inst),
            offset: offset.parse().unwrap(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum InstructionType {
    Acc,
    Nop,
    Jmp,
}

impl InstructionType {
    fn from_str(s: impl AsRef<str>) -> Self {
        match s.as_ref() {
            "nop" => Self::Nop,
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tprint;

    use super::*;

    #[test]
    fn test_inst_type() { 
        assert_eq!(InstructionType::from_str("nop"), InstructionType::Nop);
    }

    #[test]
    fn test_inst() { 
        let inst = Instruction::from_str("jmp -3");
        assert_eq!(inst.type_, InstructionType::Jmp);
        assert_eq!(inst.offset, -3);
    }
}