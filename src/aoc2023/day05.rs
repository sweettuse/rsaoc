use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

use itertools::Itertools;
use strum_macros::EnumString;

use crate::{print1, utils::read_file23};

pub fn main() -> (u64, Result<u64, String>) {
    (part1(), part2())
}

fn part1() -> u64 {
    let almanac = Almanac::from_fname("05.txt");
    // return 0u64;
    let t: HashSet<_> = HashSet::from_iter(almanac.seeds.clone());
    _find_lowest_location(&almanac, &t)
}

/// need totally different approach
/// idk, map out all ranges and work on them directly vs individual elements of the ranges?
fn part2() -> Result<u64, String> {
    Err("unsolved".to_string())
    // let almanac = Almanac::from_fname("05.txt");
    // let mut seeds: Vec<u64> = Vec::new();
    // let starts: HashSet<u64> = almanac
    //     .seeds
    //     .clone()
    //     .chunks(2)
    //     .filter_map(|start_end|
    //         match start_end {
    //             [start, end] => Some((*start, *end)),
    //             _ => None,
    //         }
    //     )
    //     .flat_map(|(start, end)| start..=(start + end))
    //     .collect();
    // print1!(starts.len());
    // // for chunk in almanac.seeds.clone().chunks(2) {
    // //     print1!(chunk);
    // // }

    // 0u64
}

fn _find_lowest_location(almanac: &Almanac, seeds: &HashSet<u64>) -> u64 {
    seeds
        .iter()
        .map(|seed_num| almanac.convert(*seed_num, ResourceType::Location).value)
        .min()
        .unwrap()
}

// =============================================================================
// TYPES
// =============================================================================
#[derive(Debug, Eq, PartialEq, EnumString, Clone, Copy, Hash)]
#[strum(ascii_case_insensitive)]
enum ResourceType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Resource {
    type_: ResourceType,
    value: u64,
}

/// allow conversions between typed ranges
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Pair {
    from: Range,
    to: Range,
}

/// resource and its range
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Range {
    type_: ResourceType,
    start: u64,
    range: u64,
}

/// all the mappings from/to resources in a pair
#[derive(Debug, Default)]
struct ResourceMap {
    data: HashMap<(ResourceType, ResourceType), Vec<Pair>>,
    graph: HashMap<ResourceType, ResourceType>,
}

/// the whole shebang
#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u64>,
    map: ResourceMap,
}

// =============================================================================
// IMPLs
// =============================================================================

impl Range {
    fn calc_offset(&self, resource: &Resource) -> Option<u64> {
        if resource.type_ != self.type_ {
            return None;
        }
        if !(self.start <= resource.value && resource.value < self.start + self.range) {
            return None;
        }
        Some(resource.value - self.start)
    }
}

impl Pair {
    fn parse_groups(s: &str) -> (ResourceType, ResourceType, Vec<Self>) {
        let mut lines = s.split('\n');
        let (from_type, to_type) = _parse_types(lines.next().unwrap());
        let pairs: Vec<Self> = lines
            .map(|l| {
                let (to_start, from_start, range) =
                    _parse_ints(l).iter().copied().next_tuple().unwrap();
                Self {
                    from: Range {
                        type_: from_type,
                        start: from_start,
                        range,
                    },
                    to: Range {
                        type_: to_type,
                        start: to_start,
                        range,
                    },
                }
            })
            .collect();
        (from_type, to_type, pairs)
    }

    fn convert(&self, from: &Resource, to: ResourceType) -> Option<Resource> {
        if (from.type_, to) != (self.from.type_, self.to.type_) {
            return None;
        }
        if let Some(offset) = self.from.calc_offset(from) {
            return Some(Resource {
                type_: to,
                value: self.to.start + offset,
            });
        }
        None
    }
}

impl ResourceMap {
    fn convert(&self, from: &Resource, to: ResourceType) -> Resource {
        let mut res: Resource = *from;
        for (_, cur_to) in self.path(from.type_, to) {
            res = self._convert_helper(&res, cur_to);
        }
        res
    }

    fn _convert_helper(&self, from: &Resource, to: ResourceType) -> Resource {
        let pairs = match self.data.get(&(from.type_, to)) {
            Some(ranges) => ranges,
            None => panic!("no conversion possible! {from:?} {to:?}"),
        };

        for p in pairs {
            if let Some(res) = p.convert(from, to) {
                return res;
            }
        }

        Resource {
            type_: to,
            value: from.value,
        }
    }

    fn path(&self, from: ResourceType, to: ResourceType) -> Vec<(ResourceType, ResourceType)> {
        if from == to {
            return vec![];
        }

        let mut cur_from = from;
        let mut res = vec![];
        while cur_from != to {
            if let Some(cur_to) = self.graph.get(&cur_from) {
                res.push((cur_from, *cur_to));
                cur_from = *cur_to;
            } else {
                break;
            }
        }
        res
    }
}

impl Almanac {
    fn from_fname(fname: &str) -> Self {
        let group_data = read_file23(fname).join("\n");
        let mut groups = group_data.split("\n\n");
        let seeds = _parse_ints(groups.next().unwrap());
        let mut resource_map = ResourceMap::default();
        for g in groups {
            let (from_type, to_type, pairs) = Pair::parse_groups(g);
            resource_map.data.insert((from_type, to_type), pairs);
            resource_map.graph.insert(from_type, to_type);
        }
        Self {
            seeds,
            map: resource_map,
        }
    }
    fn convert(&self, seed_num: u64, to: ResourceType) -> Resource {
        let seed = Resource {
            type_: ResourceType::Seed,
            value: seed_num,
        };
        self.map.convert(&seed, to)
    }
}

// =============================================================================
// FNs
// =============================================================================

fn _parse_ints(line: &str) -> Vec<u64> {
    line.split(' ')
        .filter_map(|v| v.parse::<u64>().ok())
        .collect()
}

/// example: "temperature-to-humidity map:"
fn _parse_types(s: &str) -> (ResourceType, ResourceType) {
    let (data, _) = s.split_once(' ').unwrap();
    let (from, _, to) = data.split('-').next_tuple().unwrap();

    (
        ResourceType::from_str(from).unwrap(),
        ResourceType::from_str(to).unwrap(),
    )
}
