use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::helper::Point;
use crate::shared::*;

// ======================================================
// DAY 17
// ======================================================

#[aoc_generator(day17)]
pub fn input_generator_day17(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Scaffold,
    Robot(Dir),
}

#[aoc(day17, part1)]
pub fn solve_day15_part1(input: &[i64]) -> i64 {
    let mut program = Program::new(input, &[]);
    program.run();

    let mut map = vec![vec![]];
    let mut y = 0;

    // Build the map
    for &ch in program.outputs.iter() {
        if ch == 35 {
            // #
            map[y].push(Tile::Scaffold);
        } else if ch == 46 {
            // .
            map[y].push(Tile::Empty);
        } else if ch == 94 {
            // ^
            map[y].push(Tile::Robot(Dir::Up));
        } else if ch == 60 {
            // <
            map[y].push(Tile::Robot(Dir::Left));
        } else if ch == 62 {
            // >
            map[y].push(Tile::Robot(Dir::Right));
        } else if ch == 118 {
            // v
            map[y].push(Tile::Robot(Dir::Down));
        }
        if ch == 10 {
            y += 1;
            map.push(vec![]);
        }
    }
    if map.last().unwrap().is_empty() {
        map.pop(); // Remove trailing newline
    }

    println!("Map: {} x {}", map.len(), map[0].len());

    let mut intersections = vec![];

    for y in 1..map.len() - 1 {
        for x in 1..map[y].len() - 1 {
            let cell = map[y][x];
            if cell == Tile::Scaffold {
                // Check cardinal directions
                if map[y][x - 1] == Tile::Scaffold
                    && map[y][x + 1] == Tile::Scaffold
                    && map[y - 1][x] == Tile::Scaffold
                    && map[y + 1][x] == Tile::Scaffold
                {
                    // We have an intersection
                    intersections.push(Point::new(x as i64, y as i64));
                }
            }
        }
    }

    intersections.iter().map(|p| p.x * p.y).sum()
}

/*

A------------- B------- B------- A------------- C--------- B------- C--------- C--------- B------- A-------------
R10 R8 L10 L10 R8 L6 L6 R8 L6 L6 R10 R8 L10 L10 L10 R10 L6 R8 L6 L6 L10 R10 L6 L10 R10 L6 R8 L6 L6 R10 R8 L10 L10

A - R10 R8 L10 L10
B - R8 L6 L6
C - L10 R10 L6

*/

#[aoc(day17, part2)]
pub fn solve_day15_part2(input: &[i64]) -> i64 {
    let main_routine = "A,B,B,A,C,B,C,C,B,A\n";
    let routine_a = "R,10,R,8,L,10,L,10\n";
    let routine_b = "R,8,L,6,L,6\n";
    let routine_c = "L,10,R,10,L,6\n";
    let show_video = "n\n";
    let inputs = format!(
        "{}{}{}{}{}",
        main_routine, routine_a, routine_b, routine_c, show_video
    );
    let inputs_ascii = inputs.chars().map(|c| c as i64).collect_vec();
    let mut program = Program::new(input, &inputs_ascii);
    program[0] = 2;

    program.run();

    *program.outputs.last().unwrap()
}
