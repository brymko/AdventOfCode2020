use crate::Perf;
use std::iter::FromIterator;
use std::collections::VecDeque;

const INPUT: &str = include_str!("d9input");
const PREAMBLE_COUNT: usize = 25;

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
    let mut preamble = input.iter().take(PREAMBLE_COUNT).copied().collect::<VecDeque<usize>>();

    let num = input.iter().skip(PREAMBLE_COUNT).find(|&&num| {
        let mut correct = false;
        'out: for a in &preamble {
            for b in &preamble {
                if a + b == num {
                    correct = true;
                    break 'out;
                }
            }
        }

        if correct {
            preamble.pop_front();
            preamble.push_back(num);
            false
        } else {
            true
        }
    }).unwrap();

    println!("part1 {}", num);
}

fn part2(input: &[usize]) {
    let mut preamble = input.iter().take(PREAMBLE_COUNT).copied().collect::<VecDeque<usize>>();

    let num = input.iter().skip(PREAMBLE_COUNT).find_map(|&num| {
        let mut correct = false;
        'out: for a in &preamble {
            for b in &preamble {
                if a + b == num {
                    correct = true;
                    break 'out;
                }
            }
        }

        if correct {
            preamble.pop_front();
            preamble.push_back(num);
            None
        } else {
            for i in 0..input.len() {
                for j in 1..input.len() - i {
                    let contin_try: usize = input.iter().skip(i).take(j).sum();
                    if contin_try == num {
                        let min = input.iter().skip(i).take(j).min().unwrap();
                        let max = input.iter().skip(i).take(j).max().unwrap();
                        return Some(min + max);

                    }
                }
            }
            panic!("contin nums not found");
        }
    }).unwrap();

    println!("part2 {}", num);
    
}
