use crate::Perf;
use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("d5input");

pub fn main() {
    let perf = Perf::default();

    let input = INPUT.split('\n').filter_map(Seat::from).collect::<Vec<_>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

#[derive(PartialEq, Debug)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn from(s: &str) -> Option<Seat> {
        let row = s.chars().take(7).enumerate().fold(0, |acc, (i, c)| {
            assert!(c == 'B' || c == 'F');
            if c == 'B' {
                acc | 1 << (6 - i)
            } else {
                acc
            }
        });

        let col = s.chars().rev().take(3).enumerate().fold(0, |acc, (i, c)| {
            assert!(c == 'R' || c == 'L');
            if c == 'R' {
                acc | 1 << (i)
            } else {
                acc
            }
        });

        Some(Seat { row, col })
    }

    fn seat_id(&self) -> usize {
        self.row * 8 + self.col
    }

    fn all() -> Vec<Seat> {
        let mut ret = Vec::with_capacity(128 * 8);

        for row in 0..128 {
            for col in 0..8 {
                ret.push(Seat { row, col });
            }
        }

        ret
    }
}

fn part1(input: &[Seat]) {
    let highest = input.iter().map(|s| s.seat_id()).max();
    println!("part1: {}", highest.unwrap());
}

fn part2(input: &[Seat]) {
    let input_hash: HashSet<usize> = HashSet::from_iter(input.iter().map(|seat| seat.seat_id()));

    let all = Seat::all();
    let my = all
        .iter()
        .filter(|pseat| !input.contains(pseat))
        .find(|empty| {
            input_hash.contains(&(empty.seat_id() + 1))
                && input_hash.contains(&(empty.seat_id() - 1))
        });

    println!("part2: {}", my.unwrap().seat_id());
}
