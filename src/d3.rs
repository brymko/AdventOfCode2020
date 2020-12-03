use crate::Perf;

const INPUT: &str = include_str!("d3input");

pub fn main() {
    let perf = Perf::default();
    let input = INPUT
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
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
