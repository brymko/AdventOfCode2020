use crate::Perf;

const INPUT: &str = include_str!("d2input");

#[derive(Debug)]
struct PolicyPw {
    min: u8,
    max: u8,
    ch: char,
    pw: Vec<char>,
}

impl PolicyPw {
    fn from(input: &str) -> Option<Self> {
        let mut parts = input.split(' ');
        let min_max = parts
            .next()?
            .split('-')
            .filter_map(|elem| elem.parse().ok())
            .collect::<Vec<u8>>();

        let ch = parts.next()?.chars().next()?;

        let (min, max) = (*min_max.get(0)?, *min_max.get(1)?);

        let pw = parts.next()?.chars().collect::<Vec<char>>();

        Some(PolicyPw { min, max, ch, pw })
    }

    fn is_part1_valid(&self) -> bool {
        let amount = self
            .pw
            .iter()
            .fold(0, |acc, &c| if c == self.ch { acc + 1 } else { acc });
        self.max >= amount && amount >= self.min
    }

    fn is_part2_valid(&self) -> bool {
        let validate = || -> Option<bool> {
            let first = *self.pw.get(self.min as usize - 1)?;
            let second = *self.pw.get(self.max as usize - 1)?;

            if first == self.ch && second != self.ch || first != self.ch && second == self.ch {
                Some(true)
            } else {
                Some(false)
            }
        };

        validate().unwrap()
    }
}

pub fn main() {
    let perf = Perf::default();
    let input = INPUT
        .split('\n')
        .filter_map(|line| PolicyPw::from(line))
        .collect::<Vec<PolicyPw>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

fn part1(input: &[PolicyPw]) {
    let correct = input
        .iter()
        .fold(0, |acc, el| if el.is_part1_valid() { acc + 1 } else { acc });

    println!("part1: {} correct pws", correct);
}

fn part2(input: &[PolicyPw]) {
    let correct = input
        .iter()
        .fold(0, |acc, el| if el.is_part2_valid() { acc + 1 } else { acc });
    println!("part2: {} correct pws", correct);
}
