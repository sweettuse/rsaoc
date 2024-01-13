use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{tprint, utils::read_file20};

pub type AocRes = Result<u32, String>;

pub fn main() -> (AocRes, AocRes) {
    (part1(), part2())
}

type BagManifest = HashMap<String, Bag>;

fn part1() -> AocRes {
    let bag_manifest = _get_data("07.txt");
    let mut cache = HashMap::new();

    Ok(bag_manifest
        .keys()
        .map(|name| contains("shiny gold", name, &bag_manifest, &mut cache))
        .filter(|v| *v != 0)
        .count() as u32)

}

fn contains(
    target: &str,
    current: &str,
    bag_manifest: &BagManifest,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(amount) = cache.get(current) {
        return *amount;
    }
    let current_bag = bag_manifest.get(current).unwrap();
    if let Some(target_amount) = current_bag.contents.get(target) {
        let target_amount = *target_amount;
        cache.insert(current.to_string(), target_amount);
        return target_amount;
    }

    let res = current_bag
        .contents
        .iter()
        .map(|(name, amount)| *amount * contains(target, name, bag_manifest, cache))
        .filter(|v| *v != 0)
        .sum();
    cache.insert(current.to_string(), res);
    res
}

fn part2() -> AocRes {
    let bag_manifest = _get_data("07.txt");
    let mut cache = HashMap::new();
    Ok(contains2(bag_manifest.get("shiny gold").unwrap(), 1, &bag_manifest, &mut cache) - 1)
    // Err("unsolved".to_string())
}

fn contains2(
    bag: &Bag,
    amount: u32,
    bag_manifest: &BagManifest,
    cache: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(res) = cache.get(&bag.name) {
        return *res;
    }
    let res = bag.contents.iter().map(|(name, inner_amount)| {
        let inner_bag = bag_manifest.get(name).unwrap();
        contains2(inner_bag, *inner_amount, bag_manifest, cache)
    }).sum::<u32>() + 1u32;

    amount * res
}


#[derive(Debug)]
struct Bag {
    name: String,
    contents: HashMap<String, u32>,
}

impl Bag {
    fn from_str(s: impl AsRef<str>) -> Self {
        let (name, rest) = s.as_ref().split_once(" contain ").unwrap();
        let contents = rest
            .split(", ")
            .map(Self::parse_num_bags)
            .filter(|(_, num)| *num != 0)
            .collect();

        Self {
            name: Self::parse_bag_name(name),
            contents,
        }
    }

    fn parse_bag_name(s: impl AsRef<str>) -> String {
        let fields = s.as_ref().split(' ').collect_vec();
        fields[..fields.len() - 1].join(" ")
    }

    fn parse_num_bags(s: impl AsRef<str>) -> (String, u32) {
        let (num, rest) = s.as_ref().split_once(' ').unwrap();
        (
            Self::parse_bag_name(rest),
            num.parse::<u32>().unwrap_or(0u32),
        )
    }
}
fn _get_data(fname: &str) -> BagManifest {
    let lines = read_file20(fname);
    lines
        .iter()
        .map(|l| {
            let bag = Bag::from_str(l);
            (bag.name.clone(), bag)
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::tprint;

    use super::*;

    #[test]
    fn test_bag() {
        let bag =
            Bag::from_str("dim silver bags contain 2 shiny chartreuse bags, 4 dull magenta bags.");
        assert_eq!(bag.name, "dim silver");
        assert_eq!(bag.contents["dull magenta"], 4);
        let bag = Bag::from_str("dim silver bags contain no bags");
        assert!(bag.contents.is_empty());
    }


}
