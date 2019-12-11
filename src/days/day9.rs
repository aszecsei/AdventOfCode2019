use aoc_runner_derive::{aoc, aoc_generator};

use crate::shared::*;

// ======================================================
// DAY 9
// ======================================================

#[aoc_generator(day9)]
pub fn input_generator_day9(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_day9_part1(input: &[i64]) -> String {
    let mut program = Program::new(input, &[1]);

    program.run();

    format!("{:?}", program.outputs)
}

#[aoc(day9, part2)]
pub fn solve_day9_part2(input: &[i64]) -> String {
    let mut program = Program::new(input, &[2]);

    program.run();

    format!("{:?}", program.outputs)
}
