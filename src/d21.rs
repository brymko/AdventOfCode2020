use crate::Perf;
use core::hash::Hash;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT: &str = include_str!("d21input");
const PREAMBLE_COUNT: usize = 25;

pub fn main() {
    let perf = Perf::default();

    let input: Vec<Food> = INPUT.lines().filter_map(Food::from_str).collect();
    perf.print("setup");

    let mut ings = HashMap::new();
    let mut ing_all = HashMap::new();

    input.iter().for_each(|food| {
        food.ing.iter().for_each(|ing| {
            if let Some(v) = ings.insert(ing, 1) {
                ings.insert(ing, v + 1);
            }
        });

        food.all.iter().for_each(|a| {
            if let Some(v) = ing_all.insert(a, food.ing.clone()) {
                ing_all.insert(a, common_elems(food.ing.as_slice(), v.as_slice()));
            }
        })
    });

    let mut matched_ing_aller = HashMap::new();
    while !ing_all.is_empty() {
        let safe_aller: (String, String) = ing_all
            .iter()
            .find(|(_, ings)| ings.len() == 1)
            .map(|(&a, ings)| (a.clone(), ings[0].clone()))
            .unwrap();

        ing_all.retain(|_, ing| {
            ing.retain(|ing| *ing != safe_aller.1);
            !ing.is_empty()
        });

        matched_ing_aller.insert(safe_aller.0, safe_aller.1);
    }

    let sum: usize = ings
        .iter()
        .filter(|(&ing, _)| !matched_ing_aller.iter().any(|(k, v)| v == ing))
        .map(|(_, count)| count)
        .sum();

    println!("{}", sum);
    perf.print("part1");

    // # Part 2
    // sortbyallergen = sorted([(a, i) for a, i in danger.items()])
    // print(','.join([i for _, i in sortbyallergen]))
    let mut allergen: Vec<&String> = matched_ing_aller.iter().map(|(k, v)| k).collect();
    allergen.sort();

    let part2 = allergen
        .iter()
        .map(|a| &matched_ing_aller[a.as_str()])
        .cloned()
        .collect::<Vec<String>>()
        .join(",");

    println!("{}", part2);
    perf.print("part2");
}

#[derive(Debug, Clone)]
struct Food {
    ing: Vec<String>,
    all: Vec<String>,
}

impl Food {
    fn from_str(s: &str) -> Option<Self> {
        let ing = s
            .split('(')
            .next()?
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let all = s
            .split("(contains ")
            .nth(1)?
            .replace(')', " ")
            .split(',')
            .map(|a| a.trim().to_string())
            .collect();

        Some(Self { ing, all })
    }
}

fn common_elems<C: PartialEq + Eq + Hash + Clone>(a: &[C], b: &[C]) -> Vec<C> {
    let set = HashSet::<&C>::from_iter(a.iter());

    Vec::from_iter(b.iter().filter(|el| set.contains(el)).cloned())
}
