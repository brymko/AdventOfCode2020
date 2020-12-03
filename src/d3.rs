use crate::Perf;

const INPUT: &str = include_str!("d3input");

pub fn main() {
    let perf = Perf::default();
    let input = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    let fast = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
    part2_fast(fast.as_slice());
    perf.print("part2_fast");
}

fn part1(input: &[&str]) {
    let encounters = check_slope(input, 3, 1);
    println!("part1: {}", encounters);
}

fn part2(input: &[&str]) {
    let a = check_slope(input, 1, 1);
    let b = check_slope(input, 3, 1);
    let c = check_slope(input, 5, 1);
    let d = check_slope(input, 7, 1);
    let e = check_slope(input, 1, 2);

    println!("part2: {}", a * b * c * d * e);
}

fn check_slope(input: &[&str], right: usize, down: usize) -> usize {
    input
        .iter()
        .skip(down)
        .step_by(down)
        .enumerate()
        .fold(0, |acc, (i, line)| {
            let to_check = (i + 1) * right;

            let field = line.chars().cycle().nth(to_check).unwrap();

            if field == '#' {
                acc + 1
            } else {
                acc
            }
        })
}

fn part2_fast(input: &[Vec<char>]) {
    let mut slopes_to_check = [(1, 1, 0), (3, 1, 0), (5, 1, 0), (7, 1, 0), (1, 2, 0)];

    input.iter().enumerate().for_each(|(i, k)| {
        if i == 0 {
            return;
        }
        for slope in &mut slopes_to_check {
            if i % slope.1 == 0 {
                let field = *k.get((slope.0 * (i / slope.1)) % k.len()).unwrap();
                if field == '#' {
                    slope.2 += 1;
                }
            }
        }
    });

    let ammount = slopes_to_check.iter().fold(1, |acc, slope| slope.2 * acc);
    println!("part2_fast: {}", ammount);
}
