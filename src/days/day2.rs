use aoc_runner_derive::{aoc, aoc_generator};

use crate::shared::*;

// ======================================================
// DAY 2
// ======================================================

#[aoc_generator(day2)]
pub fn input_generator_day2(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &[i64]) -> i64 {
    let mut program = Program::new(input, &[]);
    
    // Restore the gravity assist program
    // Replace position 1 with the value 12
    program.data[1] = 12;
    program.data[2] = 2;

    // Run the computer
    program.run();

    program.data[0]
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &[i64]) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = Program::new(input, &[]);
            program.data[1] = noun;
            program.data[2] = verb;
            program.run();
            if program.data[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No noun/verb combination found!");
}
