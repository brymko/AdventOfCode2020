use crate::Perf;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const INPUT: &str = include_str!("d16input");
// const INPUT: &str = "class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50

// your ticket:
// 7,1,14

// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12";

const MY_TICKET: &[usize] = &[
    73, 59, 83, 127, 137, 151, 71, 139, 67, 53, 89, 79, 61, 109, 131, 103, 149, 97, 107, 101,
];

pub fn main() {
    let perf = Perf::default();

    let meta: Vec<MetaField> = INPUT
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .filter_map(|l| {
            let mut fields = l.split(": ");
            let name = fields.next()?.to_string();
            let nums = fields.next()?;
            let mut ranges = nums.split(" or ");

            Some(MetaField {
                name,
                first: NumRange::from_str(ranges.next()?)?,
                last: NumRange::from_str(ranges.next()?)?,
            })
        })
        .collect();

    let nearby_tickets: Vec<Vec<usize>> = INPUT
        .split("\n\n")
        .nth(2)
        .unwrap()
        .lines()
        .map(|l| {
            l.split(',')
                .filter_map(|num| Some(num.parse().ok()?))
                .collect()
        })
        .collect();

    perf.print("setup");
    part1(meta.as_slice(), nearby_tickets.as_slice());
    perf.print("part1");
    part2(meta.as_slice(), nearby_tickets.as_slice());
    perf.print("part2");
}

#[derive(Debug)]
struct NumRange {
    start: usize,
    end: usize,
}

impl NumRange {
    fn from_str(s: &str) -> Option<Self> {
        let mut s = s.split('-');
        Some(Self {
            start: s.next()?.parse().ok()?,
            end: s.next()?.parse().ok()?,
        })
    }
}

#[derive(Debug)]
struct MetaField {
    name: String,
    first: NumRange,
    last: NumRange,
}

fn part1(meta: &[MetaField], tickets: &[Vec<usize>]) {
    let count = tickets.iter().fold(0, |mut acc, t| {
        t.iter().for_each(|&f| {
            let no_field = meta.iter().all(|mf| {
                !(mf.first.start <= f && f <= mf.first.end
                    || mf.last.start <= f && f <= mf.last.end)
            });

            if no_field {
                acc += f;
            }
        });

        acc
    });

    println!("part1: {}", count);
}

fn part2(meta: &[MetaField], tickets: &[Vec<usize>]) {
    let valid_tickets: Vec<Vec<usize>> = tickets
        .iter()
        .cloned()
        .filter(|t| {
            !t.is_empty()
                && !t.iter().any(|&f| {
                    meta.iter().all(|mf| {
                        !(mf.first.start <= f && f <= mf.first.end
                            || mf.last.start <= f && f <= mf.last.end)
                    })
                })
        })
        .chain((&[MY_TICKET.to_vec()][..]).iter().cloned())
        .collect();

    let mut filed_index = Vec::new();

    for pos in 0..MY_TICKET.len() {
        // check if there is only one possible col
        let pos_fields: Vec<Vec<&str>> = valid_tickets
            .iter()
            .map(|ticket| {
                let f = ticket[pos];
                let mut pos_fields = Vec::new();
                meta.iter().for_each(|mf| {
                    let in_meta = (mf.first.start <= f && f <= mf.first.end
                        || mf.last.start <= f && f <= mf.last.end);
                    if in_meta {
                        pos_fields.push(mf.name.as_str());
                    }
                });
                pos_fields
            })
            .collect();

        let mut deduped = pos_fields.get(0).unwrap().clone();
        pos_fields.iter().skip(1).for_each(|names| {
            deduped = deduped
                .iter()
                .cloned()
                .filter(|name| names.iter().any(|n| n == name))
                .collect();
        });

        filed_index.push((deduped, pos));
    }

    filed_index.sort_by(|a, b| a.0.len().cmp(&b.0.len()));

    let mut assigned = HashMap::new();
    for fidx in filed_index {
        for name in fidx.0 {
            match assigned.insert(name, fidx.1) {
                Some(v) => {
                    assigned.insert(name, v);
                }
                None => break,
            }
        }
    }

    let aw = assigned.iter().fold(1, |acc, (key, val)| {
        if key.contains("departure") {
            acc * MY_TICKET[*val]
        } else {
            acc
        }
    });

    println!("part2: {}", aw);
}
