use crate::Perf;

const INPUT: &str = include_str!("d4input");

pub fn main() {
    let perf = Perf::default();

    let input = INPUT
        .split("\n\n")
        .filter_map(|entry| Passport::from(entry))
        .collect::<Vec<Passport>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

fn part1(input: &[Passport]) {
    println!("part1 {}", input.len());
}

fn part2(input: &[Passport]) {
    let count = input
        .iter()
        .fold(0, |acc, entry| if entry.is_valid() { acc + 1 } else { acc });

    println!("part2 {}", count);
}

#[derive(Default, Debug)]
struct Passport {
    byr: u32,
    iyr: u32,
    eyr: u32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Passport {
    fn from(s: &str) -> Option<Self> {
        let mut ret = Self::default();

        for field in s.split_whitespace() {
            let mut sp = field.split(':');
            let field = sp.next()?;
            let content = sp.next()?;

            match field {
                "byr" => ret.byr = content.parse().ok()?,
                "iyr" => ret.iyr = content.parse().ok()?,
                "eyr" => ret.eyr = content.parse().ok()?,
                "hgt" => ret.hgt = content.to_string(),
                "hcl" => ret.hcl = content.to_string(),
                "ecl" => ret.ecl = content.to_string(),
                "pid" => ret.pid = content.to_string(),
                _ => {}
            }
        }

        if ret.byr == 0
            || ret.iyr == 0
            || ret.eyr == 0
            || ret.hgt.is_empty()
            || ret.hcl.is_empty()
            || ret.ecl.is_empty()
            || ret.pid.is_empty()
        {
            None
        } else {
            Some(ret)
        }
    }

    fn is_valid(&self) -> bool {
        if !(1920 <= self.byr && self.byr <= 2002) {
            return false;
        }
        if !(2010 <= self.iyr && self.iyr <= 2020) {
            return false;
        }
        if !(2020 <= self.eyr && self.eyr <= 2030) {
            return false;
        }

        if let Some(off) = self.hgt.find("in") {
            let content: u32 = self.hgt.split_at(off).0.parse().ok().unwrap_or(0);
            if !(59 <= content && content <= 76) {
                return false;
            }
        } else if let Some(off) = self.hgt.find("cm") {
            let content: u32 = self.hgt.split_at(off).0.parse().ok().unwrap_or(0);
            if !(150 <= content && content <= 193) {
                return false;
            }
        } else {
            return false;
        }

        if !self.hcl.starts_with('#') || self.hcl.len() != 7 {
            return false;
        }
        for ch in self.hcl.chars().skip(1) {
            if !ch.is_ascii_hexdigit() {
                return false;
            }
        }

        match self.ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => return false,
        }

        if self.pid.len() != 9 {
            return false;
        }
        for ch in self.pid.chars() {
            if !ch.is_ascii_digit() {
                return false;
            }
        }
        true
    }
}
