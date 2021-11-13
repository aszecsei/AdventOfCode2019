use aoc_runner_derive::{aoc, aoc_generator};

// ======================================================
// DAY 16
// ======================================================

#[aoc_generator(day16)]
pub fn input_generator_day16(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect()
}

#[test]
fn test_day16_process_1() {
    let mut input: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    process(&mut input);
    assert_eq!(input, [4, 8, 2, 2, 6, 1, 5, 8]);
    process(&mut input);
    assert_eq!(input, [3, 4, 0, 4, 0, 4, 3, 8]);
}

#[test]
fn test_day16_process_2() {
    let input = input_generator_day16("80871224585914546619083218645595");
    let res = solve_day16_part1(&input);
    assert_eq!(res, 24176176);
}

#[test]
fn test_day16_process_3() {
    let input = input_generator_day16("19617804207202209144916044189917");
    let res = solve_day16_part1(&input);
    assert_eq!(res, 73745418);
}

#[test]
fn test_day16_process_4() {
    let input = input_generator_day16("69317163492948606335995924319873");
    let res = solve_day16_part1(&input);
    assert_eq!(res, 52432133);
}

fn process(input: &mut [u8]) {
    const PATTERN: [i8; 4] = [0, 1, 0, -1];
    let mut new_input = vec![0u8; input.len()];
    for digit in 1..=input.len() {
        let pattern = PATTERN
            .iter()
            .flat_map(|&v| std::iter::repeat(v).take(digit))
            .cycle()
            .skip(1);
        let digit_sum: i64 = input
            .iter()
            .zip(pattern)
            .map(|(a, b)| (*a as i64) * (b as i64))
            .sum();
        let final_digit = i64::abs(digit_sum % 10);
        new_input[digit - 1] = final_digit as u8;
    }

    input.clone_from_slice(&new_input);
}

fn to_number(input: &[u8]) -> usize {
    input.iter().fold(0usize, |acc, x| acc * 10 + (*x as usize))
}

#[aoc(day16, part1)]
pub fn solve_day16_part1(input: &[u8]) -> usize {
    let mut v = input.to_vec();
    for _i in 0..100 {
        process(&mut v);
    }
    to_number(&v[..8])
}
