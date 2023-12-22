use std::{
    any::Any,
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use itertools::Itertools;

use crate::{tprint, utils::read_file23};

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

fn part1() -> AocRes {
    let mut grid = _get_data("20.txt");
    grid.run(1_000);
    Ok(grid.counter.values().product())
    // tprint!(grid.counter);
    // Err("unsolved".to_string())
}

#[allow(unreachable_code)]
fn part2() -> AocRes {
    return Err("unsolved".to_string());
    let mut grid = _get_data("20.txt");
    grid.run(100_000);
    for (source, count) in grid
        .target_counter
        .iter()
        .sorted_by(|a, b| Ord::cmp(b.1, a.1))
    {
        tprint!(source, count);
    };
    // Ok(grid.counter.values().product())
}

fn _get_data(fname: &str) -> Grid {
    Grid::from_str(read_file23(fname).join("\n"))
}

// =============================================================================
// ENUMS
// =============================================================================

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
    Button,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
enum PulseType {
    Low,
    High,
}

impl PulseType {
    fn flip(&self) -> PulseType {
        match self {
            PulseType::Low => PulseType::High,
            PulseType::High => PulseType::Low,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum OnOff {
    On,
    Off,
}
impl OnOff {
    fn flip(&self) -> Self {
        match self {
            OnOff::On => OnOff::Off,
            OnOff::Off => OnOff::On,
        }
    }
}

// =============================================================================
// STRUCTS
// =============================================================================

#[derive(Debug, Eq, PartialEq)]
/// a pulse being sent from a module to another module
struct Pulse {
    source: String,
    target: String,
    type_: PulseType,
}

impl Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{:?}-> {}", self.source, self.type_, self.target)
    }
}

/// a communication device that receives/sends signals
#[derive(Debug)]
struct Module {
    type_: ModuleType,
    name: String,
    targets: Vec<String>,
}

/// on low state, flip the current pulse type and emit that to all children
#[derive(Debug)]
struct FlipFlop {
    module: Module,
    state: OnOff,
}

/// if last-received pulses from all its upstream modules are High, then emit a Low pulse signal
/// otherwise, emit a High pulse
#[derive(Debug)]
struct Conjunction {
    module: Module,
    inputs: HashMap<String, PulseType>,
}

/// emit received pulse to all targets
#[derive(Debug)]
struct Broadcast {
    module: Module,
}

/// layout of the all of the communication modules
#[derive(Debug)]
struct Grid {
    modules: HashMap<String, Box<dyn ModuleTrait>>,
    pulses: VecDeque<Pulse>,
    counter: HashMap<PulseType, u32>,
    unknown: HashMap<String, Vec<Pulse>>,
    target_counter: HashMap<String, u32>,
}

/// emit a single low pulse to the `broadcaster`
#[derive(Debug)]
struct Button {
    module: Module,
}

impl Button {
    fn new() -> Self {
        Self {
            module: Module {
                type_: ModuleType::Button,
                name: String::from("button"),
                targets: vec![String::from("broadcaster")],
            },
        }
    }
}

// =============================================================================
// IMPLS
// =============================================================================

impl Grid {
    fn emit(&mut self, pulse: Pulse) {
        self.pulses.push_back(pulse);
    }

    fn run(&mut self, num_presses: u32) {
        for _ in 0..num_presses {
            self._press_button();
            self._process();
        }
    }

    fn _process(&mut self) {
        // println!("{}", "-".repeat(30));
        while let Some(pulse) = self.pulses.pop_front() {
            *self.target_counter.entry(pulse.target.clone()).or_default() += 1;
            if pulse.target == "rx" && pulse.type_ == PulseType::Low {
                panic!("i can't believe this worked")
            }
            // println!("{}", pulse);
            *self.counter.entry(pulse.type_).or_default() += 1;
            if let Some(target) = self.modules.get_mut(&pulse.target) {
                self.pulses.extend(target.receive(pulse));
            } else {
                self.unknown
                    .entry(pulse.target.clone())
                    .or_default()
                    .push(pulse);
            }
        }
    }
    fn _press_button(&mut self) {
        let mut button = Button::new();
        self.pulses.extend(button.receive(Pulse {
            source: "grid".to_owned(),
            target: "button".to_owned(),
            type_: PulseType::Low,
        }));
    }

    fn from_str(s: impl AsRef<str>) -> Self {
        let mut modules = s
            .as_ref()
            .split('\n')
            .map(Module::from_str)
            .map(|m| (m.name(), m))
            .collect();

        Self::_update_conjunctions(&mut modules);
        Self {
            modules,
            pulses: VecDeque::default(),
            counter: HashMap::default(),
            unknown: HashMap::default(),
            target_counter: HashMap::default(),
        }
    }

    fn _update_conjunctions(modules: &mut HashMap<String, Box<dyn ModuleTrait>>) {
        let source_target_map = modules
            .iter()
            .map(|(source, m)| {
                (
                    source.clone(),
                    m.targets().iter().cloned().cloned().collect_vec(),
                )
            })
            .collect::<HashMap<_, _>>();

        for (source, targets) in source_target_map.iter() {
            for t in targets {
                if let Some(m) = modules.get_mut(t) {
                    if m.module_type() != ModuleType::Conjunction {
                        continue;
                    }
                    m.add_input(source.clone());
                }
            }
        }
    }
}

// =============================================================================
// TRAITS
// =============================================================================
use std::fmt::Debug;

trait ModuleTrait: Debug {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse>;
    fn name(&self) -> String;
    fn targets(&self) -> Vec<&String>;
    fn module_type(&self) -> ModuleType;
    fn generate_pulses(&self, targets: &[String], pulse_type: PulseType) -> Vec<Pulse> {
        targets
            .iter()
            .map(|t| Pulse {
                source: self.name(),
                target: t.to_owned(),
                type_: pulse_type,
            })
            .collect()
    }
    fn add_input(&mut self, _s: String) {
        panic!("should only be called by Conjunction");
    }
}

impl Module {
    fn from_str(s: &str) -> Box<dyn ModuleTrait> {
        let (name, target_str) = s.split_once(" -> ").unwrap();
        let c = name.chars().next().unwrap();
        let (module_type, name) = match c {
            'b' => (ModuleType::Broadcast, name),
            '%' => (ModuleType::FlipFlop, &name[1..]),
            '&' => (ModuleType::Conjunction, &name[1..]),
            _ => panic!("unmatched char in ModuleTrait: {c}"),
        };

        let module = Module {
            type_: module_type,
            name: name.to_owned(),
            targets: target_str.split(", ").map(|s| s.to_owned()).collect(),
        };

        match module_type {
            ModuleType::FlipFlop => Box::new(FlipFlop {
                module,
                state: OnOff::Off,
            }),
            ModuleType::Conjunction => Box::new(Conjunction {
                module,
                inputs: HashMap::default(),
            }),
            ModuleType::Broadcast => Box::new(Broadcast { module }),
            ModuleType::Button => panic!("should not receive a button here!"),
        }
    }
}

impl ModuleTrait for Button {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        vec![Pulse {
            source: "button".to_owned(),
            target: "broadcaster".to_owned(),
            ..pulse
        }]
    }

    fn name(&self) -> String {
        String::from("button")
    }

    fn targets(&self) -> Vec<&String> {
        self.module.targets.iter().collect()
    }

    fn module_type(&self) -> ModuleType {
        self.module.type_
    }
}

impl ModuleTrait for Broadcast {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        self.generate_pulses(&self.module.targets, pulse.type_)
    }

    fn name(&self) -> String {
        self.module.name.clone()
    }

    fn targets(&self) -> Vec<&String> {
        self.module.targets.iter().collect()
    }
    fn module_type(&self) -> ModuleType {
        self.module.type_
    }
}

impl ModuleTrait for FlipFlop {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        if pulse.type_ == PulseType::High {
            return Vec::new();
        }

        self.state = self.state.flip();

        self.generate_pulses(
            &self.module.targets,
            match self.state {
                OnOff::On => PulseType::High,
                OnOff::Off => PulseType::Low,
            },
        )
    }
    fn name(&self) -> String {
        self.module.name.clone()
    }
    fn targets(&self) -> Vec<&String> {
        self.module.targets.iter().collect()
    }
    fn module_type(&self) -> ModuleType {
        self.module.type_
    }
}

impl ModuleTrait for Conjunction {
    fn receive(&mut self, pulse: Pulse) -> Vec<Pulse> {
        let t = self.inputs.get_mut(&pulse.source).unwrap();
        *t = pulse.type_;
        let output_type = match self.inputs.values().all(|v| *v == PulseType::High) {
            true => PulseType::Low,
            false => PulseType::High,
        };
        self.generate_pulses(&self.module.targets, output_type)
    }
    fn name(&self) -> String {
        self.module.name.clone()
    }
    fn targets(&self) -> Vec<&String> {
        self.module.targets.iter().collect()
    }
    fn module_type(&self) -> ModuleType {
        self.module.type_
    }
    fn add_input(&mut self, s: String) {
        self.inputs.insert(s, PulseType::Low);
    }
}