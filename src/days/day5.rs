use aoc_runner_derive::{aoc, aoc_generator};

use crate::shared::*;

// ======================================================
// DAY 5
// ======================================================

#[aoc_generator(day5)]
pub fn input_generator_day5(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_day5_part1(input: &[i64]) -> i64 {
    let mut program = Program::new(input, &[1]);

    program.run();

    *program.outputs.last().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_day5_part2(input: &[i64]) -> i64 {
    let mut program = Program::new(input, &[5]);

    program.run();

    *program.outputs.last().unwrap()
}
