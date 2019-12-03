use aoc_runner_derive::{aoc, aoc_generator, aoc_lib};

// ======================================================
// DAY 1
// ======================================================

#[aoc_generator(day1)]
pub fn input_generator_day1(input: &str) -> Vec<i64> {
    input.lines().map(|x| x.trim().parse().unwrap()).collect()
}

#[inline]
fn fuel_cost(mass: i64) -> i64 {
    (mass / 3) - 2
}

#[aoc(day1, part1)]
pub fn solve_day1_part1(input: &[i64]) -> i64 {
    input.iter().map(|x| fuel_cost(*x)).sum()
}

#[aoc(day1, part2)]
pub fn solve_day1_part2(input: &[i64]) -> i64 {
    input
        .iter()
        .map(|x| {
            let mut fuel = fuel_cost(*x);
            let mut just_added = fuel;
            loop {
                let new_req = fuel_cost(just_added);
                if new_req <= 0 {
                    break;
                }
                fuel += new_req;
                just_added = new_req;
            }
            fuel
        })
        .sum()
}

#[test]
fn test_day1() {
    assert_eq!(solve_day1_part2(&[14]), 2);
    assert_eq!(solve_day1_part2(&[1969]), 966);
    assert_eq!(solve_day1_part2(&[100756]), 50346);
}

// ======================================================
// DAY 2
// ======================================================

#[aoc_generator(day2)]
pub fn input_generator_day2(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect()
}

pub fn intcode(v: &mut [usize]) {
    let mut current_instruction = 0usize;

    // Run the computer
    while current_instruction < v.len() {
        // Perform instruction
        match v[current_instruction] {
            1 => {
                // Addition opcode
                let in_pos1 = v[current_instruction + 1];
                let in_pos2 = v[current_instruction + 2];
                let out_pos = v[current_instruction + 3];
                v[out_pos] = v[in_pos1] + v[in_pos2];
            }
            2 => {
                // Multiplication opcode
                let in_pos1 = v[current_instruction + 1];
                let in_pos2 = v[current_instruction + 2];
                let out_pos = v[current_instruction + 3];
                v[out_pos] = v[in_pos1] * v[in_pos2];
            }
            99 => break,
            _ => panic!("Unexpected opcode!"),
        }

        // Move to the next instruction
        current_instruction += 4;
    }
}

#[test]
fn test_intcode_basic() {
    let mut program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    intcode(&mut program);

    assert_eq!(program, [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
}

#[aoc(day2, part1)]
pub fn solve_day2_part1(input: &[usize]) -> usize {
    let mut v = input.to_vec();

    // Restore the gravity assist program
    // Replace position 1 with the value 12
    v[1] = 12;
    v[2] = 2;

    // Run the computer
    intcode(&mut v);

    v[0]
}

#[aoc(day2, part2)]
pub fn solve_day2_part2(input: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut v = input.to_vec();
            v[1] = noun;
            v[2] = verb;
            intcode(&mut v);
            if v[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No noun/verb combination found!");
}

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position(i64, i64);

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
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (inp[0].clone(), inp[1].clone())
}

#[aoc(day3, part1)]
pub fn solve_day3_part1(input: &(Vec<PathComponent>, Vec<PathComponent>)) -> u64 {
    use std::collections::HashSet;

    let mut closest = std::u64::MAX;

    let path1 = &input.0;
    let path2 = &input.1;

    let mut positions: HashSet<Position> = HashSet::default();

    // First wire
    {
        let mut current_position = Position(0, 0);
        for movement in path1.iter() {
            for _ in 0..movement.1 {
                current_position = match movement.0 {
                    Direction::Left => Position(current_position.0 - 1, current_position.1),
                    Direction::Right => Position(current_position.0 + 1, current_position.1),
                    Direction::Up => Position(current_position.0, current_position.1 - 1),
                    Direction::Down => Position(current_position.0, current_position.1 + 1),
                };
                positions.insert(current_position);
            }
        }
    }
    // Second wire - check for intersections
    {
        let mut current_position = Position(0, 0);
        for movement in path2.iter() {
            for _ in 0..movement.1 {
                current_position = match movement.0 {
                    Direction::Left => Position(current_position.0 - 1, current_position.1),
                    Direction::Right => Position(current_position.0 + 1, current_position.1),
                    Direction::Up => Position(current_position.0, current_position.1 - 1),
                    Direction::Down => Position(current_position.0, current_position.1 + 1),
                };
                if positions.contains(&current_position) {
                    // We have an intersection; calculate the manhattan distance and
                    // store if this is our closest intersection yet
                    let md = (current_position.0.abs() + current_position.1.abs()) as u64;
                    closest = std::cmp::min(md, closest);
                }
            }
        }
    }

    closest
}

#[aoc(day3, part2)]
pub fn solve_day3_part2(input: &(Vec<PathComponent>, Vec<PathComponent>)) -> u64 {
    use std::collections::HashMap;

    let mut closest = std::u64::MAX;

    let path1 = &input.0;
    let path2 = &input.1;

    let mut positions: HashMap<Position, u64> = HashMap::default();

    // First wire
    {
        let mut current_position = Position(0, 0);
        let mut total_movement = 0;
        for movement in path1.iter() {
            for _ in 0..movement.1 {
                current_position = match movement.0 {
                    Direction::Left => Position(current_position.0 - 1, current_position.1),
                    Direction::Right => Position(current_position.0 + 1, current_position.1),
                    Direction::Up => Position(current_position.0, current_position.1 - 1),
                    Direction::Down => Position(current_position.0, current_position.1 + 1),
                };
                total_movement += 1;
                positions.insert(current_position, total_movement);
            }
        }
    }
    // Second wire - check for intersections
    {
        let mut current_position = Position(0, 0);
        let mut total_movement = 0;
        for movement in path2.iter() {
            for _ in 0..movement.1 {
                current_position = match movement.0 {
                    Direction::Left => Position(current_position.0 - 1, current_position.1),
                    Direction::Right => Position(current_position.0 + 1, current_position.1),
                    Direction::Up => Position(current_position.0, current_position.1 - 1),
                    Direction::Down => Position(current_position.0, current_position.1 + 1),
                };
                total_movement += 1;

                if positions.contains_key(&current_position) {
                    // We have an intersection; calculate the manhattan distance and
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

aoc_lib! { year = 2019 }
