use crate::Perf;
const INPUT: &str = include_str!("d8input");

#[derive(Debug, Clone, Copy)]
enum Mnemonic {
    Nop,
    Jmp,
    Acc,
}

impl Mnemonic {
    fn from(s: &str) -> Option<Self> {
        Some(match s {
            "nop" => Self::Nop,
            "acc" => Self::Acc,
            "jmp" => Self::Jmp,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone)]
struct Ins {
    value: isize,
    ins: Mnemonic,
    visited: bool,
}

impl Ins {
    fn from_line(s: &str) -> Option<Self> {
        let mut fields = s.split(' ');

        let ins = Mnemonic::from(fields.next()?)?;
        let value = fields.next()?.parse().ok()?;

        Some( Self {
            value, ins, visited: false,
        })
    }
}

pub fn main() {
    let perf = Perf::default();

    let ins = INPUT.lines().filter_map(Ins::from_line).collect::<Vec<Ins>>();
    println!("{:#?}", ins);

    perf.print("setup");
    part1(ins.as_slice());
    perf.print("part1");
    part2(ins.as_slice());
    perf.print("part2");
}

fn part1(ins: &[Ins]) {
    let mut acc = 0;
    let mut pc = 0isize;
    let mut ins = ins.iter().cloned().collect::<Vec<Ins>>();
    loop {
        // crash to better debug
        // if pc >= ins.len() {
        //     break;
        // }

        let instruct = &mut ins[pc as usize];

        println!("{:?}", instruct);
        if instruct.visited {
            break;
        }

        instruct.visited = true;

        match instruct.ins {
            Mnemonic::Jmp => pc += instruct.value,
            Mnemonic::Nop => pc += 1,
            Mnemonic::Acc => { pc += 1; acc += instruct.value; }
        }
    }

    println!("part1 {}", acc);
}

fn part2(ins: &[Ins]) {
    for (i, k) in ins.iter().enumerate() {
        match k.ins {
            Mnemonic::Jmp | Mnemonic::Nop => {
                if let Some(acc) = part2_try(i, ins) {
                    println!("part2 {}", acc);
                    return;
                }
            }
            _ => {}
        }
    }
}

fn part2_try(c: usize, ins: &[Ins]) -> Option<isize>{
    let mut acc = 0;
    let mut pc = 0isize;
    let mut ins = ins.iter().cloned().collect::<Vec<Ins>>();
    loop {
        if pc as usize >= ins.len() {
            return Some(acc);
        }

        let instruct = &mut ins[pc as usize];

        println!("{:?}", instruct);
        if instruct.visited {
            return None;
        }

        instruct.visited = true;

        let inst = if pc as usize == c {
            match instruct.ins {
                Mnemonic::Jmp => Mnemonic::Nop,
                Mnemonic::Nop => Mnemonic::Jmp,
                Mnemonic::Acc => Mnemonic::Acc,
            }
        } else {
            instruct.ins
        };

        match inst  {
            Mnemonic::Jmp => pc += instruct.value,
            Mnemonic::Nop => pc += 1,
            Mnemonic::Acc => { pc += 1; acc += instruct.value; }
        }
    }
}
