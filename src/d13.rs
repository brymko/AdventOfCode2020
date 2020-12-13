use crate::Perf;
use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("d13input");
// const INPUT: &str = "3223
// 7,13,x,x,59,x,31,19";

pub fn main() {
    let perf = Perf::default();

    let mut deps_diff = 0;
    let mut lines = INPUT.lines();
    let start: usize = lines.next().unwrap().parse().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|num| {
            let deps = deps_diff;
            deps_diff += 1;
            Some(Bus {
                deps_diff: deps,
                interval: num.parse().ok()?,
            })
        })
        .collect::<Vec<Bus>>();

    perf.print("setup");
    part1(start, busses.as_slice());
    perf.print("part1");
    part2(busses.as_slice());
    perf.print("part2");
}

#[derive(Debug)]
struct Bus {
    deps_diff: usize,
    interval: usize,
}

fn part1(start: usize, busses: &[Bus]) {
    let mut id = 0;
    let mut loweset = usize::MAX;

    for bus in busses {
        let diff = bus.interval - start % bus.interval;
        if diff < loweset {
            loweset = diff;
            id = bus.interval;
        }
    }

    println!("part1: {}", id * loweset);
}

fn part2(busses: &[Bus]) {
    let t = busses
        .iter()
        .fold((0, 1), |mut acc, b| {
            while (acc.0 + b.deps_diff) % b.interval != 0 {
                acc.0 += acc.1;
            }
            acc.1 *= b.interval;
            acc
        })
        .0;

    println!("part2: {}", t);
}
