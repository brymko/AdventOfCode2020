use crate::Perf;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT: &str = "0,3,1,6,7,5";
// const INPUT: &str = "0,3,6";

pub fn main() {
    let perf = Perf::default();

    let input: Vec<usize> = INPUT.split(',').filter_map(|c| c.parse().ok()).collect();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

fn part1(input: &[usize]) {
    let mut numbers = input.to_vec();
    for i in numbers.len()..2020 {
        let last = numbers.last().unwrap();
        let diff = numbers.iter().rev().skip(1).enumerate().find_map(|(i, k)| {
            if k == last {
                Some(i + 1)
            } else {
                None
            }
        });

        let to_push = diff.unwrap_or(0);
        numbers.push(to_push);
    }

    println!("{:?}", numbers.last().unwrap());
}

fn part2(input: &[usize]) {
    let mut seen: HashMap<usize, usize> = input
        .iter()
        .take(input.len() - 1)
        .enumerate()
        .map(|(i, e)| (*e, i + 1))
        .collect();

    let start = input.iter().rev().next().unwrap();
    let num = (input.len()..30000000).fold(*start, |acc, i| match seen.insert(acc, i) {
        Some(t) => i - t,
        None => 0,
    });

    println!("{:?}", num);
}
