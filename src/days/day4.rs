use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;

// ======================================================
// DAY 4
// ======================================================

#[aoc_generator(day4)]
pub fn input_generator_day4(input: &str) -> (usize, usize) {
    let vals = input
        .split('-')
        .map(|x| x.trim().parse().unwrap())
        .collect_vec();
    (vals[0], vals[1])
}

fn is_valid(digits: &[usize]) -> bool {
    // Must have a duplicate somewhere
    for n in 1..6 {
        if digits[n] == digits[n - 1] {
            return true;
        }
    }
    false
}

fn get_num(digits: &[usize]) -> usize {
    let mut acc = 0;
    for d in digits {
        acc *= 10;
        acc += d;
    }
    acc
}

#[aoc(day4, part1)]
pub fn solve_day4_part1((start, end): &(usize, usize)) -> usize {
    let mut digits = [0; 6];
    let mut val = *start;
    let mut index = 5;
    while val > 0 {
        digits[index] = val % 10;
        val /= 10;
        index -= 1;
    }

    // Start with the smallest strictly-increasing number in range
    for n in 1..6 {
        if digits[n] < digits[n - 1] {
            digits[n] = digits[n - 1];
        }
    }

    let mut count = 0;

    while get_num(&digits) < *end {
        if is_valid(&digits) {
            count += 1;
        }

        let mut change_idx = 5;
        while digits[change_idx] == 9 {
            change_idx -= 1;
        }
        digits[change_idx] += 1;
        let selected_num = digits[change_idx];
        for digit in digits.iter_mut().skip(change_idx) {
            *digit = selected_num;
        }
    }

    count
}

fn is_valid_2(digits: &[usize]) -> bool {
    // Must have an exact duplicate somewhere
    for n in 1..6 {
        if digits[n] == digits[n - 1]
            && (n < 2 || digits[n - 2] != digits[n])
            && (n > 4 || digits[n + 1] != digits[n])
        {
            return true;
        }
    }
    false
}

#[aoc(day4, part2)]
pub fn solve_day4_part2((start, end): &(usize, usize)) -> usize {
    let mut digits = [0; 6];
    let mut val = *start;
    let mut index = 5;
    while val > 0 {
        digits[index] = val % 10;
        val /= 10;
        index -= 1;
    }

    // Start with the smallest strictly-increasing number in range
    for n in 1..6 {
        if digits[n] < digits[n - 1] {
            digits[n] = digits[n - 1];
        }
    }

    let mut count = 0;

    while get_num(&digits) < *end {
        if is_valid_2(&digits) {
            count += 1;
        }

        let mut change_idx = 5;
        while digits[change_idx] == 9 {
            change_idx -= 1;
        }
        digits[change_idx] += 1;
        let selected_num = digits[change_idx];
        for digit in digits.iter_mut().skip(change_idx) {
            *digit = selected_num;
        }
    }

    count
}
