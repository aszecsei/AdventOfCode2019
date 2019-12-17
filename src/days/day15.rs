use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::helper::Point;
use crate::shared::*;

// ======================================================
// DAY 15
// ======================================================

#[aoc_generator(day15)]
pub fn input_generator_day15(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
    Oxygen,
}

#[aoc(day15, part1)]
pub fn solve_day15_part1(input: &[i64]) -> usize {
    let mut program = Program::new(input, &[]);

    let mut output_idx = 0;
    let mut nodes: HashMap<Point, Tile> = HashMap::default();
    let mut curr_pos = Point::new(0, 0);
    nodes.insert(curr_pos, Tile::Empty);
    let mut positions: Vec<Point> = vec![];

    'outer: while program.get_status() != IntcodeStepResult::Halt {
        // Decide where to move
        let deltas = vec![
            Point::new(0, 1),
            Point::new(0, -1),
            Point::new(-1, 0),
            Point::new(1, 0),
        ];

        // Look for unexplored deltas
        let movement = deltas
            .iter()
            .find(|d| !nodes.contains_key(&(curr_pos + **d)))
            .copied()
            .unwrap_or_else(|| positions.pop().expect("No previous locations") - curr_pos);

        // Move in that direction
        program.add_input(match movement {
            Point { x: 0, y: -1 } => 1, // North
            Point { x: 0, y: 1 } => 2,  // South
            Point { x: -1, y: 0 } => 3, // West
            Point { x: 1, y: 0 } => 4,  // East
            _ => panic!("Invalid movement {:?}", movement),
        });
        program.run();

        // Check robot outputs
        while output_idx < program.outputs.len() {
            match program.outputs[output_idx] {
                0 => {
                    // Hit a wall. Location unchanged.
                    nodes.insert(curr_pos + movement, Tile::Wall);
                }
                1 => {
                    // Moved one step in the requested direction.
                    if !nodes.contains_key(&(curr_pos + movement)) {
                        // We are not backtracking
                        positions.push(curr_pos);
                    }
                    curr_pos += movement;

                    nodes.insert(curr_pos, Tile::Empty);
                }
                2 => {
                    // Moved one step in the requested direction. New position is the oxygen
                    // subsystem.
                    // We are guaranteed to not be backtracking
                    positions.push(curr_pos);
                    curr_pos += movement;
                    nodes.insert(curr_pos, Tile::Oxygen);
                    break 'outer;
                }
                _ => panic!("Unexpected output {}", program.outputs[output_idx]),
            }
            output_idx += 1;
        }
    }

    positions.len()
}

#[aoc(day15, part2)]
pub fn solve_day15_part2(input: &[i64]) -> usize {
    let mut program = Program::new(input, &[]);

    let mut output_idx = 0;
    let mut nodes: HashMap<Point, Tile> = HashMap::default();
    let mut curr_pos = Point::new(0, 0);
    nodes.insert(curr_pos, Tile::Empty);
    let mut positions: Vec<Point> = vec![];

    let mut oxygen = Point::new(0, 0);

    let deltas = vec![
        Point::new(0, 1),
        Point::new(0, -1),
        Point::new(-1, 0),
        Point::new(1, 0),
    ];

    'outer: while program.get_status() != IntcodeStepResult::Halt {
        // Decide where to move
        // Look for unexplored deltas
        let movement = deltas
            .iter()
            .find(|d| !nodes.contains_key(&(curr_pos + **d)))
            .copied()
            .unwrap_or_else(|| positions.pop().unwrap_or(curr_pos + Point::new(-1, -1)) - curr_pos);
        if movement == Point::new(-1, -1) {
            break 'outer; // We've fully explored the map
        }

        // Move in that direction
        program.add_input(match movement {
            Point { x: 0, y: -1 } => 1, // North
            Point { x: 0, y: 1 } => 2,  // South
            Point { x: -1, y: 0 } => 3, // West
            Point { x: 1, y: 0 } => 4,  // East
            _ => panic!("Invalid movement {:?}", movement),
        });
        program.run();

        // Check robot outputs
        while output_idx < program.outputs.len() {
            match program.outputs[output_idx] {
                0 => {
                    // Hit a wall. Location unchanged.
                    nodes.insert(curr_pos + movement, Tile::Wall);
                }
                1 => {
                    // Moved one step in the requested direction.
                    if !nodes.contains_key(&(curr_pos + movement)) {
                        // We are not backtracking
                        positions.push(curr_pos);
                    }
                    curr_pos += movement;

                    nodes.insert(curr_pos, Tile::Empty);
                }
                2 => {
                    // Moved one step in the requested direction. New position is the oxygen
                    // subsystem.
                    if !nodes.contains_key(&(curr_pos + movement)) {
                        // We are not backtracking
                        positions.push(curr_pos);
                    }
                    curr_pos += movement;
                    nodes.insert(curr_pos, Tile::Oxygen);
                    oxygen = curr_pos;
                }
                _ => panic!("Unexpected output {}", program.outputs[output_idx]),
            }
            output_idx += 1;
        }
    }

    // Calculate how long the maximal path away from the oxygen location is
    let mut checked: HashSet<Point> = HashSet::default();
    let mut boundary: VecDeque<Point> = VecDeque::new();
    let mut lengths: HashMap<Point, usize> = HashMap::default();

    lengths.insert(oxygen, 0);
    boundary.push_back(oxygen);

    while !boundary.is_empty() {
        let to_check = boundary.pop_front().unwrap();
        checked.insert(to_check);

        for &d in deltas.iter() {
            let pos = d + to_check;
            if !checked.contains(&pos) && nodes[&pos] != Tile::Wall {
                lengths.insert(pos, lengths[&to_check] + 1);
                boundary.push_back(pos);
            }
        }
    }

    *lengths.values().max().unwrap()
}
