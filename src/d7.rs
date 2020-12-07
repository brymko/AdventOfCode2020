use crate::Perf;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT: &str = include_str!("d7input");

#[derive(Debug)]
struct Bags {
    hm: HashMap<String, Vec<(u32, String)>>,
}

pub fn main() {
    let perf = Perf::default();

    let mut input = Bags { hm: HashMap::new() };
    input.hm = HashMap::from_iter(INPUT.lines().filter_map(|l| {
        let mut elems = l.split(" bags contain ");
        let bname = elems.next()?;
        let subbags = elems
            .next()?
            .split(|c: char| c.is_ascii_punctuation())
            .filter(|e| !e.is_empty())
            .filter_map(|e| {
                let e = e.strip_prefix(' ').unwrap_or(e);
                let amount: u32 = e[0..1].parse().ok()?;
                let name = e[2..].split(" bag").next()?.to_string();
                Some((amount, name))
            })
            .collect::<Vec<(u32, String)>>();

        Some((bname.to_string(), subbags))
    }));

    println!("{:#?}", input);

    perf.print("setup");
    part1(&input);
    perf.print("part1");
    part2(&input);
    perf.print("part2");
}

fn reverse_hashmap(
    hm: &HashMap<String, Vec<(u32, String)>>,
) -> HashMap<String, Vec<(u32, String)>> {
    let mut ret: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    for (key, value) in hm {
        for (amount, name) in value {
            match ret.get_mut(name) {
                Some(value) => {
                    value.push((*amount, key.clone()));
                }
                None => {
                    ret.insert(name.clone(), vec![(*amount, key.clone())]);
                }
            };
        }
    }

    ret
}

fn part1(input: &Bags) {
    let mut needles = vec!["shiny gold"];
    let rhm = reverse_hashmap(&input.hm);
    let mut hs = HashSet::new();
    let mut amount = 0;

    while let Some(cur) = needles.pop() {
        hs.insert(cur);
        if let Some(derived_from) = rhm.get(cur) {
            for derived in derived_from {
                needles.push(&derived.1);
            }
        }
    }

    println!("part1 {}", hs.len() - 1);
}

fn part2(input: &Bags) {
    println!("part2 {}", get_num_bags(input, "shiny gold"));
}

fn get_num_bags(input: &Bags, bname: &str) -> u32 {
    let mut needles = vec![bname];
    let rhm = reverse_hashmap(&input.hm);
    let mut amount = 0;

    while let Some(cur) = needles.pop() {
        if let Some(bags) = input.hm.get(cur) {
            for bag in bags {
                amount += bag.0 + bag.0 * get_num_bags(input, bag.1.as_str());
            }
        }
    }

    amount
}
