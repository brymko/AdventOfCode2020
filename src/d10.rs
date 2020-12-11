use crate::Perf;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT: &str = include_str!("d10input");

pub fn main() {
    let perf = Perf::default();

    let input = INPUT.lines().filter_map(|l| l.parse().ok()).collect::<Vec<usize>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

fn part1(input: &[usize]) {
    let mut set: HashSet<&usize> = HashSet::from_iter(input.iter());
    let mut ones = 1;
    let mut threes = 1;

    input.iter().for_each(|num| {
        if set.get(&(num + 1)).is_some() {
            ones += 1;
        } else if set.get(&(num + 3)).is_some() {
            threes += 1;
        }
    });

    println!("part1: {}", ones * threes);
}

fn part2(input: &[usize]) {
    let mut set: HashSet<usize> = HashSet::from_iter(input.iter().copied());
    let mut visited: HashSet<usize> = HashSet::new();

    let mut possi = 1usize;
    let mut coll = input.to_vec();
    coll.insert(0, 0);
    coll.sort();
    let mut last = 0;
    let mut skip_next = false;

    coll.iter().for_each(|num| {
        // count nr of paths till num + 3
        let mut paths = 0;
        let nums: Vec<usize> = (1..=3).filter_map(|offset| { 
            set.get(&(num + offset))
        }).copied().collect();

        let ll = last;
        last = nums.len();
        if !skip_next {
            let paths = 
                if nums.len() == 3 && ll == 3 {
                    skip_next = true;
                    possi /= 2;
                    7
                } else if nums.len() == 3 {
                    2
                } else if nums.len() == 2 && ll == 2 {
                    1 
                } else {
                    nums.len()
                };
            if paths > 0 {
                possi *= paths 
            }
        } else {
            skip_next = false;
        }

    });

    println!("part2: {}", possi);
}
