
use aoc_runner_derive::{aoc, aoc_generator};
use crate::shared::*;
use std::collections::HashMap;

// ======================================================
// DAY 11
// ======================================================

#[aoc_generator(day11)]
pub fn input_generator_day11(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Black,
    White
}

#[aoc(day11, part1)]
pub fn solve_day11_part1(input: &[i64]) -> String {
    let mut painting: HashMap<(i64, i64), Color> = HashMap::new();

    let mut program = Program::new(input, &[]);

    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;

    let mut output_idx = 0;

    while program.get_status() != IntcodeStepResult::Halt {
        program.run();

        while output_idx < program.outputs.len() {
            let output_color = match program.outputs[output_idx] {
                0 => Color::Black,
                1 => Color::White,
                _ => panic!("Unexpected output {}", program.outputs[output_idx]),
            };

            painting.insert((x, y), output_color);

            match program.outputs[output_idx + 1] {
                0 => {
                    // Turn left 90 degrees
                    dir -= 1;
                }
                1 => {
                    // Turn right 90 degrees
                    dir += 1;
                }
                _ => (),
            }

            if dir < 0 {
                dir += 4;
            }
            if dir >= 4 {
                dir -= 4;
            }
            
            // Move forward one panel
            match dir {
                0 => y -= 1,
                1 => x += 1,
                2 => y += 1,
                3 => x -= 1,
                _ => panic!("Unexpected direction: {}", dir),
            }

            output_idx += 2;
        }

        if program.get_status() == IntcodeStepResult::WaitingForInput {
            // Provide current color if requested
            let curr_color = painting.get(&(x, y)).unwrap_or(&Color::Black);
            program.add_input(match curr_color {
                Color::Black => 0,
                Color::White => 1,
            });
        }
    }

    format!("{}", painting.len())
}

#[aoc(day11, part2)]
pub fn solve_day11_part2(input: &[i64]) -> String {
    let mut painting: HashMap<(i64, i64), Color> = HashMap::new();
    painting.insert((0, 0), Color::White);

    let mut program = Program::new(input, &[]);

    let mut x = 0;
    let mut y = 0;
    let mut dir = 0;

    let mut output_idx = 0;

    while program.get_status() != IntcodeStepResult::Halt {
        program.run();

        while output_idx < program.outputs.len() {
            let output_color = match program.outputs[output_idx] {
                0 => Color::Black,
                1 => Color::White,
                _ => panic!("Unexpected output {}", program.outputs[output_idx]),
            };

            painting.insert((x, y), output_color);

            match program.outputs[output_idx + 1] {
                0 => {
                    // Turn left 90 degrees
                    dir -= 1;
                }
                1 => {
                    // Turn right 90 degrees
                    dir += 1;
                }
                _ => (),
            }

            if dir < 0 {
                dir += 4;
            }
            if dir >= 4 {
                dir -= 4;
            }
            
            // Move forward one panel
            match dir {
                0 => y -= 1,
                1 => x += 1,
                2 => y += 1,
                3 => x -= 1,
                _ => panic!("Unexpected direction: {}", dir),
            }

            output_idx += 2;
        }

        if program.get_status() == IntcodeStepResult::WaitingForInput {
            // Provide current color if requested
            let curr_color = painting.get(&(x, y)).unwrap_or(&Color::Black);
            program.add_input(match curr_color {
                Color::Black => 0,
                Color::White => 1,
            });
        }
    }

    // format!("{}", painting.len())
    let min_x = painting.keys().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let min_y = painting.keys().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let max_x = painting.keys().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let max_y = painting.keys().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let mut res = String::from("\n");

    for m_y in min_y..=max_y {
        for m_x in min_x..=max_x {
            res.push(match *painting.get(&(m_x, m_y)).unwrap_or(&Color::Black) {
                Color::Black => ' ',
                Color::White => '#',
            });
        }
        res.push('\n');
    }

    res
}