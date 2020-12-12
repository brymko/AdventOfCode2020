use crate::Perf;
use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("d12input");

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn left(&mut self, amount: usize) {
        for _ in 0..amount {
            match self {
                Direction::East => *self = Direction::North,
                Direction::North => *self = Direction::West,
                Direction::West => *self = Direction::South,
                Direction::South => *self = Direction::East,
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Debug)]
struct Compas {
    dir: Direction,
    degree: isize,
}

impl Compas {
    fn from(s: &str) -> Option<Self> {
        let dir = match &s[..1] {
            "N" => Direction::North,
            "E" => Direction::East,
            "S" => Direction::South,
            "W" => Direction::West,
            "L" => Direction::Left,
            "R" => Direction::Right,
            "F" => Direction::Forward,
            _ => return None,
        };

        let degree = s[1..].parse().ok()?;

        Some(Self { dir, degree })
    }
}

pub fn main() {
    let perf = Perf::default();

    let input = INPUT
        .lines()
        .filter_map(Compas::from)
        .collect::<Vec<Compas>>();

    perf.print("setup");
    part1(input.as_slice());
    perf.print("part1");
    part2(input.as_slice());
    perf.print("part2");
}

fn part1(input: &[Compas]) {
    let mut north = 0isize;
    let mut east = 0isize;
    let mut facing = Direction::East;

    input.iter().for_each(|cmd| match cmd.dir {
        Direction::Forward => match facing {
            Direction::East => east += cmd.degree,
            Direction::West => east -= cmd.degree,
            Direction::North => north += cmd.degree,
            Direction::South => north -= cmd.degree,
            _ => unreachable!(),
        },
        Direction::Left => match cmd.degree {
            90 => facing.left(1),
            180 => facing.left(2),
            270 => facing.left(3),
            _ => panic!(format!("Where are we moving ? {:?}", cmd)),
        },
        Direction::Right => match cmd.degree {
            90 => facing.left(3),
            180 => facing.left(2),
            270 => facing.left(1),
            _ => panic!(format!("Where are we moving ? {:?}", cmd)),
        },
        Direction::East => east += cmd.degree,
        Direction::West => east -= cmd.degree,
        Direction::North => north += cmd.degree,
        Direction::South => north -= cmd.degree,
    });

    println!("part1: {}", north.abs() + east.abs());
}

fn part2(input: &[Compas]) {
    let mut north = 0isize;
    let mut east = 0isize;
    let mut waypoint = (10isize, 1isize);

    let mut move_waypoint = |waypoint: &mut (isize, isize), left: usize| {
        for _ in 0..left {
            let tmp0 = waypoint.0;
            waypoint.0 = -waypoint.1;
            waypoint.1 = tmp0;
        }
    };

    input.iter().for_each(|cmd| match cmd.dir {
        Direction::Forward => {
            north += waypoint.1 * cmd.degree;
            east += waypoint.0 * cmd.degree;
        }
        Direction::Left => match cmd.degree {
            90 => move_waypoint(&mut waypoint, 1),
            180 => move_waypoint(&mut waypoint, 2),
            270 => move_waypoint(&mut waypoint, 3),
            _ => panic!(format!("Where are we moving ? {:?}", cmd)),
        },
        Direction::Right => match cmd.degree {
            90 => move_waypoint(&mut waypoint, 3),
            180 => move_waypoint(&mut waypoint, 2),
            270 => move_waypoint(&mut waypoint, 1),
            _ => panic!(format!("Where are we moving ? {:?}", cmd)),
        },
        Direction::East => waypoint.0 += cmd.degree,
        Direction::West => waypoint.0 -= cmd.degree,
        Direction::North => waypoint.1 += cmd.degree,
        Direction::South => waypoint.1 -= cmd.degree,
    });

    println!("part2: {}", north.abs() + east.abs());
}
