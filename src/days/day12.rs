use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use lazy_static::lazy_static;
use num::integer::lcm;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashSet;

use crate::helper::Point3;

// ======================================================
// DAY 12
// ======================================================

#[aoc_generator(day12)]
pub fn input_generator_day12(input: &str) -> Vec<Point3> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>").unwrap();
    }
    input
        .lines()
        .map(|x| {
            let caps = RE.captures(x.trim()).unwrap();
            Point3::new(
                caps["x"].parse().unwrap(),
                caps["y"].parse().unwrap(),
                caps["z"].parse().unwrap(),
            )
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Moon {
    pub pos: Point3,
    pub vel: Point3,
}

impl Moon {
    pub fn new(pos: Point3, vel: Point3) -> Self {
        Moon { pos, vel }
    }
    pub fn from_pos(pos: Point3) -> Self {
        Moon {
            pos,
            vel: Point3::new(0, 0, 0),
        }
    }

    pub fn apply_velocity(&self) -> Self {
        Moon::new(self.pos + self.vel, self.vel)
    }
}

pub fn solve_day12_part1_h(input: &[Point3], steps: i64) -> i64 {
    let mut moons = input.iter().map(|&x| Moon::from_pos(x)).collect_vec();

    for _ in 0..steps {
        moons = moons
            .iter()
            .map(|x| {
                // Apply gravity
                let mut new_vel = x.vel;
                for y in moons.iter() {
                    // Get x-vel change
                    let delta_x = match x.pos.x().cmp(&y.pos.x()) {
                        std::cmp::Ordering::Less => 1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => -1,
                    };
                    let delta_y = match x.pos.y().cmp(&y.pos.y()) {
                        std::cmp::Ordering::Less => 1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => -1,
                    };
                    let delta_z = match x.pos.z().cmp(&y.pos.z()) {
                        std::cmp::Ordering::Less => 1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => -1,
                    };
                    new_vel += Point3::new(delta_x, delta_y, delta_z);
                }
                Moon::new(x.pos, new_vel).apply_velocity()
            })
            .collect_vec();
    }

    // Calculate energy
    moons
        .iter()
        .map(|m| {
            // Potential energy
            let pot = m.pos.manhattan();
            let kin = m.vel.manhattan();

            pot * kin
        })
        .sum()
}

#[aoc(day12, part1)]
pub fn solve_day12_part1(input: &[Point3]) -> i64 {
    solve_day12_part1_h(input, 1000)
}

fn day12_p2_helper(input: &[Point3], idx: usize) -> i64 {
    let mut moons = input.iter().map(|&x| Moon::from_pos(x)).collect_vec();
    let mut prev: HashSet<Vec<Moon>> = HashSet::new();
    let mut steps = 0;

    loop {
        moons = moons
            .iter()
            .map(|x| {
                // Apply gravity
                let mut new_vel = x.vel;
                for y in moons.iter() {
                    let delta = match x.pos[idx].cmp(&y.pos[idx]) {
                        std::cmp::Ordering::Less => 1,
                        std::cmp::Ordering::Equal => 0,
                        std::cmp::Ordering::Greater => -1,
                    };
                    new_vel[idx] += delta;
                }
                Moon::new(x.pos, new_vel).apply_velocity()
            })
            .collect_vec();

        if prev.contains(&moons) {
            break;
        }
        prev.insert(moons.clone());
        steps += 1;
    }
    steps
}

#[aoc(day12, part2)]
pub fn solve_day12_part2(input: &[Point3]) -> i64 {
    [0, 1, 2]
        .par_iter()
        .map(|&i| day12_p2_helper(input, i))
        .reduce(|| 1, lcm)
}

#[test]
fn test_day12_p1_short() {
    let input = vec![
        Point3::new(-1, 0, 2),
        Point3::new(2, -10, -7),
        Point3::new(4, -8, 8),
        Point3::new(3, 5, -1),
    ];
    let res = solve_day12_part1_h(&input, 10);
    assert_eq!(res, 179);
}

#[test]

fn test_day12_p1_long() {
    let input = vec![
        Point3::new(-8, -10, 0),
        Point3::new(5, 5, 10),
        Point3::new(2, -7, 3),
        Point3::new(9, -8, -3),
    ];
    let res = solve_day12_part1_h(&input, 100);
    assert_eq!(res, 1940);
}

#[test]
fn test_day12_p2_short() {
    let input = vec![
        Point3::new(-1, 0, 2),
        Point3::new(2, -10, -7),
        Point3::new(4, -8, 8),
        Point3::new(3, 5, -1),
    ];
    let res = solve_day12_part2(&input);
    assert_eq!(res, 2772);
}

#[test]

fn test_day12_p2_long() {
    let input = vec![
        Point3::new(-8, -10, 0),
        Point3::new(5, 5, 10),
        Point3::new(2, -7, 3),
        Point3::new(9, -8, -3),
    ];
    let res = solve_day12_part2(&input);
    assert_eq!(res, 4_686_774_924);
}
