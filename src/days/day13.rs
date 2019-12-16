use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use crate::shared::*;
use std::collections::HashMap;

// ======================================================
// DAY 12
// ======================================================

#[aoc_generator(day13)]
pub fn input_generator_day13(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl From<u8> for Tile {
    fn from(val: u8) -> Self {
        match val {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Unexpected tile type {}", val),
        }
    }
}

#[aoc(day13, part1)]
pub fn solve_day13_part1(input: &[i64]) -> usize {
    let mut program = Program::new(input, &[]);
    let mut screen: HashMap<(i64, i64), Tile> = HashMap::new();

    let mut output_idx = 0;

    while program.get_status() != IntcodeStepResult::Halt {
        program.run();
        while output_idx < program.outputs.len() {
            let x = program.outputs[output_idx];
            let y = program.outputs[output_idx + 1];
            let t: Tile = (program.outputs[output_idx + 2] as u8).into();
            screen.insert((x, y), t);

            output_idx += 3;
        }
    }

    screen.values().filter(|&v| *v == Tile::Block).count()
}

#[aoc(day13, part2)]
pub fn solve_day13_part2(input: &[i64]) -> i64 {
    let mut program = Program::new(input, &[]);
    program[0] = 2;
    let mut screen: HashMap<(i64, i64), Tile> = HashMap::new();

    let mut output_idx = 0;
    let mut score = 0;

    let mut ball_pos = (0, 0);
    let mut paddle_pos = (0, 0);

    while program.get_status() != IntcodeStepResult::Halt {
        program.run();
        while output_idx < program.outputs.len() {
            let x = program.outputs[output_idx];
            let y = program.outputs[output_idx + 1];
            if x == -1 && y == 0 {
                score = program.outputs[output_idx + 2];
            } else {
                let t: Tile = (program.outputs[output_idx + 2] as u8).into();
                screen.insert((x, y), t);

                if t == Tile::Paddle {
                    paddle_pos = (x, y);
                } else if t == Tile::Ball {
                    ball_pos = (x, y);
                }
            }
            
            output_idx += 3;
        }

        if program.get_status() == IntcodeStepResult::WaitingForInput {
            let mv = (ball_pos.0 - paddle_pos.0).signum();
            program.add_input(mv);
        }
    }

    score
}