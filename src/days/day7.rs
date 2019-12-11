use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::shared::*;

// ======================================================
// DAY 7
// ======================================================

#[aoc_generator(day7)]
pub fn input_generator_day7(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_day7_part1(input: &[i64]) -> i64 {
    let phase_settings = 0..5;
    let mut max_result = std::i64::MIN;
    for phases in phase_settings.permutations(5) {
        let mut result = 0;

        for phase in phases {
            let mut program = Program::new(input, &[phase, result]);

            program.run();

            result = *program.outputs.last().unwrap();
        }

        max_result = std::cmp::max(max_result, result);
    }

    max_result
}

#[aoc(day7, part2)]
pub fn solve_day7_part2(input: &[i64]) -> i64 {
    let phase_settings = 5..10;
    let mut max_result = std::i64::MIN;
    for phases in phase_settings.permutations(5) {
        let mut result = 0;

        let mut programs = phases
            .iter()
            .map(|&phase| Program::new(input, &[phase]))
            .collect_vec();

        loop {
            for program in programs.iter_mut() {
                program.add_input(result);
                program.run();
                result = *program.outputs.last().unwrap();
            }

            // Check if the final amplifier has finished
            if programs.iter().last().unwrap().get_status() == IntcodeStepResult::Halt {
                break;
            }
        }

        max_result = std::cmp::max(max_result, result);
    }

    max_result
}
