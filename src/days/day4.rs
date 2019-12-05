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

fn is_valid(digits: &[usize], part2: bool) -> bool {
    // Given our number construction, we're guaranteed a monotonically increasing
    // digit sequence. So all we have to check is the duplicate question.
    digits
        .iter()
        .group_by(|&d| d)
        .into_iter()
        .map(|(_key, group)| group.count())
        .any(|f| if part2 { f == 2 } else { f >= 2 })
}

#[test]
fn test_day4_valid() {
    assert_eq!(is_valid(&[1, 1, 1, 1, 1, 1], true), false);
    assert_eq!(is_valid(&[1, 1, 1, 1, 1, 1], false), true);
    assert_eq!(is_valid(&[1, 1, 2, 2, 2, 2], true), true);
    assert_eq!(is_valid(&[1, 1, 2, 2, 2, 2], false), true);
    assert_eq!(is_valid(&[1, 2, 3, 4, 5, 6], true), false);
    assert_eq!(is_valid(&[1, 2, 3, 4, 5, 6], false), false);
    assert_eq!(is_valid(&[1, 2, 3, 4, 5, 5], true), true);
}

fn get_num(digits: &[usize]) -> usize {
    let mut acc = 0;
    for d in digits {
        acc *= 10;
        acc += d;
    }
    acc
}

fn day4((start, end): &(usize, usize), part2: bool) -> usize {
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
        if is_valid(&digits, part2) {
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

#[aoc(day4, part1)]
pub fn solve_day4_part1(range: &(usize, usize)) -> usize {
    day4(range, false)
}

#[aoc(day4, part2)]
pub fn solve_day4_part2(range: &(usize, usize)) -> usize {
    day4(range, true)
}
