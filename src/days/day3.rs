use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::helper::*;

// ======================================================
// DAY 3
// ======================================================

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PathComponent(Direction, usize);

#[aoc_generator(day3)]
pub fn input_generator_day3(input: &str) -> (Vec<PathComponent>, Vec<PathComponent>) {
    let inp: Vec<Vec<PathComponent>> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|rec| {
                    let mut dir = rec.to_owned();
                    let amount = dir.split_off(1).parse().unwrap();
                    let d = match dir.as_str() {
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        _ => panic!("Unexpected direction"),
                    };
                    PathComponent(d, amount)
                })
                .collect_vec()
        })
        .collect_vec();
    (inp[0].clone(), inp[1].clone())
}

fn delta_for_direction(dir: Direction) -> Point {
    match dir {
        Direction::Left => Point::new(-1, 0),
        Direction::Right => Point::new(1, 0),
        Direction::Up => Point::new(0, -1),
        Direction::Down => Point::new(0, 1),
    }
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &(Vec<PathComponent>, Vec<PathComponent>)) -> u64 {
    let mut closest = std::u64::MAX;

    let path1 = &input.0;
    let path2 = &input.1;

    let mut positions: HashSet<Point> = HashSet::default();

    // First wire
    {
        let mut current_position = Point::new(0, 0);
        for movement in path1.iter() {
            for _ in 0..movement.1 {
                current_position += delta_for_direction(movement.0);
                positions.insert(current_position);
            }
        }
    }
    // Second wire - check for intersections
    {
        let mut current_position = Point::new(0, 0);
        for movement in path2.iter() {
            for _ in 0..movement.1 {
                current_position += delta_for_direction(movement.0);
                if positions.contains(&current_position) {
                    // We have an intersection; calculate the manhattan distance and
                    // store if this is our closest intersection yet
                    let md = (current_position.manhattan()) as u64;
                    closest = std::cmp::min(md, closest);
                }
            }
        }
    }

    closest
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &(Vec<PathComponent>, Vec<PathComponent>)) -> u64 {
    let mut closest = std::u64::MAX;

    let path1 = &input.0;
    let path2 = &input.1;

    let mut positions: HashMap<Point, u64> = HashMap::default();

    // First wire
    {
        let mut current_position = Point::new(0, 0);
        let mut total_movement = 0;
        for movement in path1.iter() {
            for _ in 0..movement.1 {
                current_position += delta_for_direction(movement.0);
                total_movement += 1;
                positions.insert(current_position, total_movement);
            }
        }
    }
    // Second wire - check for intersections
    {
        let mut current_position = Point::new(0, 0);
        let mut total_movement = 0;
        for movement in path2.iter() {
            for _ in 0..movement.1 {
                current_position += delta_for_direction(movement.0);
                total_movement += 1;

                if positions.contains_key(&current_position) {
                    // We have an intersection; calculate the total movement and
                    // store if this is our closest intersection yet
                    let both_movement = total_movement + positions[&current_position];
                    closest = std::cmp::min(both_movement, closest);
                }
            }
        }
    }

    closest
}

#[test]
fn test_day3_p1_ex1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let gen = input_generator_day3(input);
    let res = solve_day3_part1(&gen);

    assert_eq!(res, 159);
}

#[test]
fn test_day3_p1_ex2() {
    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let gen = input_generator_day3(input);
    let res = solve_day3_part1(&gen);

    assert_eq!(res, 135);
}

#[test]
fn test_day3_p2_ex1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    let gen = input_generator_day3(input);
    let res = solve_day3_part2(&gen);

    assert_eq!(res, 610);
}

#[test]
fn test_day3_p2_ex2() {
    let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let gen = input_generator_day3(input);
    let res = solve_day3_part2(&gen);

    assert_eq!(res, 410);
}
