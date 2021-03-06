use crate::Perf;
const INPUT: &str = include_str!("d1input");

fn part1(numbers: &[usize]) {
    for a in numbers {
        for b in numbers {
            if a + b == 2020 {
                println!("Day1 Part1: {}", a * b);
                return;
            }
        }
    }
}

fn part2(numbers: &[usize]) {
    for a in numbers {
        for b in numbers {
            for c in numbers {
                if a + b + c == 2020 {
                    println!("Day1 Part2: {}", a * b * c);
                    return;
                }
            }
        }
    }
}

pub fn main() {
    let perf = Perf::default();
    let numbers = INPUT
        .split('\n')
        .filter_map(|str_num| str_num.parse().ok())
        .collect::<Vec<usize>>();

    perf.print("setup");
    part1(numbers.as_slice());
    perf.print("Part1");
    part2(numbers.as_slice());
    perf.print("Part2");
}
