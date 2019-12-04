use aoc_runner_derive::{aoc, aoc_generator};

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
    assert_eq!(solve_day1_part2(&[100_756]), 50346);
}
