use crate::Perf;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT: &str = include_str!("d14input");
// const INPUT: &str = "
// mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1";

pub fn main() {
    let perf = Perf::default();

    let input: Vec<Instruction> = INPUT.lines().filter_map(Instruction::from).collect();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

#[derive(Debug)]
enum Instruction {
    Mask(usize, usize),
    Write(usize, usize),
}

impl Instruction {
    fn from(s: &str) -> Option<Self> {
        if s.len() < 3 {
            return None;
        }
        match &s[..3] {
            "mem" => {
                let addr: usize = s.split(&['[', ']'][..]).nth(1).unwrap().parse().ok()?;
                let what: usize = s.split(' ').rev().next().unwrap().parse().ok()?;
                Some(Self::Write(addr, what))
            }
            "mas" => {
                let as_mask = usize::from_str_radix(
                    s.split(' ')
                        .rev()
                        .next()
                        .unwrap()
                        .replace('X', "1")
                        .as_str(),
                    2,
                )
                .ok()?;
                let as_val: usize = usize::from_str_radix(
                    s.split(' ')
                        .rev()
                        .next()
                        .unwrap()
                        .replace('X', "0")
                        .as_str(),
                    2,
                )
                .ok()?;

                Some(Self::Mask(as_mask, as_val))
            }
            _ => unreachable!(),
        }
    }
}

fn part1(input: &[Instruction]) {
    let mut mask = 0;
    let mut value = 0;
    let mut mem = HashMap::new();

    input.iter().for_each(|ins| match ins {
        Instruction::Mask(m, v) => {
            mask = *m;
            value = *v;
        }
        Instruction::Write(addr, what) => {
            let v = value | (what & mask);
            mem.insert(addr, v);
        }
    });

    let sum = mem.iter().fold(0, |acc, (k, v)| acc + v);
    println!("part1: {}", sum);
}

fn part2(input: &[Instruction]) {
    let mut mask = 0;
    let mut value = 0;
    let mut mem = HashMap::new();

    input.iter().for_each(|ins| match ins {
        Instruction::Mask(m, v) => {
            mask = *m & !*v;
            value = *v;
        }
        Instruction::Write(addr, what) => {
            let v = value | addr;
            let fixed = v & !mask;
            let floating = mask.count_ones() as u32;
            for b in 0..2usize.pow(floating) {
                let comb = extend_to_mask(b, mask);
                mem.insert(fixed | comb, *what);
            }
        }
    });

    let sum = mem.iter().fold(0, |acc, (k, v)| acc + v);
    println!("part2: {}", sum);
}

fn extend_to_mask(mut value: usize, mut mask: usize) -> usize {
    let mut ret = 0;
    while value > 0 {
        let shift_amount = mask.trailing_zeros();
        if shift_amount == 64 {
            println!("{}", value);
            unreachable!();
        }
        let mut cur = (value & 1) << shift_amount;
        value >>= 1;
        ret |= cur;
        mask = ((mask >> shift_amount) & !1usize) << shift_amount;
    }

    ret
}
