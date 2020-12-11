use crate::Perf;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
const INPUT: &str = include_str!("d11input");
// const INPUT: &str = "
// L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL";

struct Row {}

pub fn main() {
    let perf = Perf::default();

    let input = INPUT
        .lines()
        .filter_map(|l| {
            if !l.is_empty() {
                Some(l.chars().collect())
            } else {
                None
            }
        })
        .collect::<Vec<Vec<char>>>();

    let length = input[0].len();
    assert!(input.iter().all(|l| l.len() == length));

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

fn part1(input: &[Vec<char>]) {
    let mut has_changed = false;
    let mut seating = input.to_vec();

    let get_adjacent_count = |seating: &[Vec<char>], row: usize, col: usize| {
        let mut count = 0;

        let check_row = |seating: &[Vec<char>], row: usize, col: usize| {
            let mut count = 0;
            let ref_row = &seating[row];

            let end = if (col + 2) > ref_row.len() {
                ref_row.len()
            } else {
                col + 2
            };

            let col = col.saturating_sub(1);

            for i in col..end {
                if ref_row[i] == '#' {
                    count += 1;
                }
            }

            count
        };

        // check above
        if row > 0 {
            count += check_row(seating, row - 1, col);
        }

        // check left & right
        let ref_row = &seating[row];
        if col >= 1 && ref_row[col - 1] == '#' {
            count += 1;
        }

        if col + 1 < ref_row.len() && ref_row[col + 1] == '#' {
            count += 1;
        }

        // check below
        if row + 1 < seating.len() {
            count += check_row(seating, row + 1, col);
        }
        count
    };

    let mut ctr = 0;

    loop {
        let mut changed: Vec<Vec<char>> = Vec::with_capacity(seating.len());
        for i in 0..seating.len() {
            changed.push(Vec::with_capacity(seating[i].len()));
            let mut cur = changed.last_mut().unwrap();
            for k in 0..seating[i].len() {
                if seating[i][k] == 'L' && get_adjacent_count(seating.as_slice(), i, k) == 0 {
                    has_changed = true;
                    cur.push('#');
                } else if seating[i][k] == '#' && get_adjacent_count(seating.as_slice(), i, k) >= 4
                {
                    has_changed = true;
                    cur.push('L');
                } else {
                    cur.push(seating[i][k]);
                }
            }
        }

        if !has_changed {
            break;
        }
        has_changed = false;
        seating = changed;
        ctr += 1;
    }

    let count = seating
        .iter()
        .fold(0, |acc, s| acc + s.iter().filter(|&&c| c == '#').count());
    println!("part1: {}", count);
}

fn part2(input: &[Vec<char>]) {
    let mut has_changed = false;
    let mut seating = input.to_vec();

    let get_seeing_occupied = |seating: &[Vec<char>], row: usize, col: usize| {
        let mut count = 0;

        let trace_view =
            |seating: &[Vec<char>], row: usize, col: usize, step_down: isize, step_right: isize| {
                let mut row = row as isize;
                let mut col = col as isize;
                if step_down == 0 && step_right == 0 {
                    return 0;
                }

                loop {
                    if row == 0 && step_down == -1
                        || row + 1 == seating.len() as isize && step_down == 1
                    {
                        return 0;
                    }
                    if col == 0 && step_right == -1
                        || col + 1 == seating[0].len() as isize && step_right == 1
                    {
                        return 0;
                    }

                    row += step_down;
                    col += step_right;

                    if seating[row as usize][col as usize] == '#' {
                        return 1;
                    } else if seating[row as usize][col as usize] == 'L' {
                        return 0;
                    }
                }
            };

        count += trace_view(seating, row, col, -1, -1);
        count += trace_view(seating, row, col, 0, -1);
        count += trace_view(seating, row, col, -1, 0);
        count += trace_view(seating, row, col, 1, 1);
        count += trace_view(seating, row, col, 0, 1);
        count += trace_view(seating, row, col, 1, 0);
        count += trace_view(seating, row, col, -1, 1);
        count += trace_view(seating, row, col, 1, -1);

        count
    };

    let mut ctr = 0;

    loop {
        let mut changed: Vec<Vec<char>> = Vec::with_capacity(seating.len());
        for i in 0..seating.len() {
            changed.push(Vec::with_capacity(seating[i].len()));
            let mut cur = changed.last_mut().unwrap();
            for k in 0..seating[i].len() {
                if seating[i][k] == 'L' && get_seeing_occupied(seating.as_slice(), i, k) == 0 {
                    has_changed = true;
                    cur.push('#');
                } else if seating[i][k] == '#' && get_seeing_occupied(seating.as_slice(), i, k) >= 5
                {
                    has_changed = true;
                    cur.push('L');
                } else {
                    cur.push(seating[i][k]);
                }
            }
        }

        if !has_changed {
            break;
        }
        has_changed = false;
        seating = changed;
        ctr += 1;
    }

    let count = seating
        .iter()
        .fold(0, |acc, s| acc + s.iter().filter(|&&c| c == '#').count());
    println!("part1: {}", count);
}
