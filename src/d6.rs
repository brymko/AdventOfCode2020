use crate::Perf;
use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("d6input");

pub fn main() {
    let perf = Perf::default();

    let input = INPUT
        .split("\n\n")
        .map(|group| group.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

fn part1(input: &[Vec<char>]) {
    let count = input.iter().fold(0, |acc, g| {
        acc + HashSet::<&char>::from_iter(g.iter().filter(|c| c.is_ascii_alphabetic())).len()
    });
    println!("part1 {}", count);
}

fn part2(input: &[Vec<char>]) {
    let count = input.iter().fold(0, |acc, g| {
        acc + g
            .split(|&c| c == '\n')
            .filter(|s| !s.is_empty())
            .fold(HashSet::<char>::from_iter('a'..='z'), |acc, g| {
                HashSet::from_iter(g.iter().filter(|c| acc.get(c).is_some()).copied())
            })
            .len()
    });
    println!("part2 {}", count);
}
