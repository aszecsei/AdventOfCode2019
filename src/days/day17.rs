use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::helper::{replace_all_with, Point};
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

impl Tile {
    fn get_dir(self) -> Option<Dir> {
        if let Tile::Robot(d) = self {
            Some(d)
        } else {
            None
        }
    }
}

fn get_map(input: &[i64]) -> Vec<Vec<Tile>> {
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
    map
}

#[aoc(day17, part1)]
pub fn solve_day15_part1(input: &[i64]) -> i64 {
    let map = get_map(input);
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Rotation {
    Right,
    Left,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Rotation(Rotation),
    Movement(usize),
    Subroutine(usize),
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Rotation(Rotation::Left) => write!(f, "L"),
            Direction::Rotation(Rotation::Right) => write!(f, "R"),
            Direction::Movement(len) => write!(f, "{}", len),
            Direction::Subroutine(0) => write!(f, "A"),
            Direction::Subroutine(1) => write!(f, "B"),
            Direction::Subroutine(2) => write!(f, "C"),
            Direction::Subroutine(x) => panic!("Unexpected subroutine {}", x),
        }
    }
}

fn get_delta_for_direction(dir: Dir) -> Point {
    match dir {
        Dir::Up => Point::new(0, -1),
        Dir::Right => Point::new(1, 0),
        Dir::Down => Point::new(0, 1),
        Dir::Left => Point::new(-1, 0),
    }
}

fn rotate(dir: Dir, rot: Rotation) -> Dir {
    match (rot, dir) {
        (Rotation::Right, Dir::Up) => Dir::Right,
        (Rotation::Right, Dir::Right) => Dir::Down,
        (Rotation::Right, Dir::Down) => Dir::Left,
        (Rotation::Right, Dir::Left) => Dir::Up,
        (Rotation::Left, Dir::Up) => Dir::Left,
        (Rotation::Left, Dir::Left) => Dir::Down,
        (Rotation::Left, Dir::Down) => Dir::Right,
        (Rotation::Left, Dir::Right) => Dir::Up,
    }
}

fn is_valid_position(pos: Point, map: &[Vec<Tile>]) -> bool {
    if pos.y < 0 || pos.y >= map.len() as i64 {
        return false;
    }
    if pos.x < 0 || pos.x >= map[pos.y as usize].len() as i64 {
        return false;
    }
    map[pos.y as usize][pos.x as usize] != Tile::Empty
}

#[aoc(day17, part2)]
pub fn solve_day15_part2(input: &[i64]) -> i64 {
    let map = get_map(input);

    // Find the robot's current location
    let tiles = map
        .iter()
        .enumerate()
        .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, t)| (x, y, t)))
        .collect_vec();

    let (mut robot_pos, mut robot_dir) = {
        let rp = tiles
            .iter()
            .find(|(_, _, &t)| match t {
                Tile::Robot(_) => true,
                _ => false,
            })
            .unwrap();
        (
            Point::new(rp.0 as i64, rp.1 as i64),
            rp.2.get_dir().unwrap(),
        )
    };

    let mut directions = vec![];
    let mut movement = 0;
    loop {
        // Check if we can keep going
        let delta = get_delta_for_direction(robot_dir);
        let new_pos = robot_pos + delta;
        if is_valid_position(new_pos, &map) {
            movement += 1;
            robot_pos = new_pos;
            continue;
        }

        if movement > 0 {
            directions.push(Direction::Movement(movement));
            movement = 0;
        }

        // Check to see if we can rotate to a valid direction
        let delta_right = get_delta_for_direction(rotate(robot_dir, Rotation::Right));
        if is_valid_position(robot_pos + delta_right, &map) {
            directions.push(Direction::Rotation(Rotation::Right));
            robot_dir = rotate(robot_dir, Rotation::Right);
            continue;
        }

        let delta_left = get_delta_for_direction(rotate(robot_dir, Rotation::Left));
        if is_valid_position(robot_pos + delta_left, &map) {
            directions.push(Direction::Rotation(Rotation::Left));
            robot_dir = rotate(robot_dir, Rotation::Left);
            continue;
        }

        // We have reached a dead end :)
        break;
    }

    // Now that we have the movement code, we need to compress it into 3 subroutines.
    let mut subroutines: [Vec<Direction>; 3] = [vec![], vec![], vec![]];
    let mut master = directions.clone();

    for (s_idx, s) in subroutines.iter_mut().enumerate() {
        let mut idx = 0;
        while idx < master.len() {
            // Skip subroutine calls
            if let Direction::Subroutine(_) = master[idx] {
                if !s.is_empty() {
                    // Replace all instances of the subroutine in `master` with this particular subroutine call
                    replace_all_with(&mut master, &s, &[Direction::Subroutine(s_idx)]);

                    // We're 1 + s.len() beyond the start of s. We remove s.len() - 1 elements from the array,
                    // and we want to progress 1 further.
                    // To stay at the same point, we subtract s.len() - 1 from idx.
                    // Then we progress forward by 1, for a total subtraction of s.len() - 2.
                    idx -= s.len() - 2;
                } else {
                    idx += 1;
                }
                continue;
            }

            s.push(master[idx]);

            // Check if there's more than 1 of `s` in master
            if master
                .as_slice()
                .windows(s.len())
                .filter(|&p| p == s.as_slice())
                .count()
                <= 1
            {
                // If not, we go back to the last valid configuration
                s.remove(s.len() - 1);
                // Replace all instances of the subroutine in `master` with this particular subroutine call
                replace_all_with(&mut master, &s, &[Direction::Subroutine(s_idx)]);
                break;
            } else {
                idx += 1;
            }
        }
    }

    let main_routine = format!("{}\n", master.iter().format(","));
    let routine_a = format!("{}\n", subroutines[0].iter().format(","));
    let routine_b = format!("{}\n", subroutines[1].iter().format(","));
    let routine_c = format!("{}\n", subroutines[2].iter().format(","));
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
