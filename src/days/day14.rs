use aoc_runner_derive::{aoc, aoc_generator};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use string_interner::{StringInterner, Sym};

// ======================================================
// DAY 14
// ======================================================

pub struct Quantity {
    pub amount: usize,
    pub symbol: Sym,
}
pub struct Recipe {
    pub inputs: Vec<Quantity>,
    pub output: Quantity,
}

#[aoc_generator(day14)]
pub fn input_generator_day14(input: &str) -> (Vec<Recipe>, Sym, Sym) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<inputs>\d+ [a-zA-Z]+(?:, \d+ [a-zA-Z]+)*) => (?P<output>\d+ [a-zA-Z]+)"
        )
        .unwrap();
    }
    let mut interner = StringInterner::default();
    let recipes = input
        .lines()
        .map(|x| {
            let caps = RE.captures(x.trim()).unwrap();
            let inputs = caps["inputs"].to_owned();
            let output = caps["output"].to_owned();

            // Parse the inputs
            let inputs = inputs
                .split(',')
                .map(|s| {
                    let mut parts = s.trim().split(' ');
                    let part1 = parts.next().unwrap();
                    let part2 = parts.next().unwrap();
                    Quantity {
                        amount: part1.parse().unwrap(),
                        symbol: interner.get_or_intern(part2),
                    }
                })
                .collect_vec();
            let mut output_parts = output.split(' ');
            let part1 = output_parts.next().unwrap();
            let part2 = output_parts.next().unwrap();
            let output = Quantity {
                amount: part1.parse().unwrap(),
                symbol: interner.get_or_intern(part2),
            };
            Recipe { inputs, output }
        })
        .collect_vec();
    (
        recipes,
        interner.get("ORE").unwrap(),
        interner.get("FUEL").unwrap(),
    )
}

fn solve_day14_part1_h(
    (recipes, ore_sym, fuel_sym): &(Vec<Recipe>, Sym, Sym),
    fuel_to_make: i64,
) -> i64 {
    let mut need: HashMap<Sym, i64> = HashMap::default();
    need.insert(*fuel_sym, fuel_to_make);

    let mut num_to_make = 1;

    while num_to_make > 0 {
        // Choose a needed product
        let (&to_make, &to_make_amt) = need.iter().find(|(&k, &v)| k != *ore_sym && v > 0).unwrap();

        // Find a recipe to make that product
        let recipe = recipes
            .iter()
            .find(|&r| r.output.symbol == to_make)
            .unwrap();

        let num_times_to_make = (to_make_amt as f64 / recipe.output.amount as f64).ceil() as i64;

        need.insert(
            to_make,
            to_make_amt - num_times_to_make * recipe.output.amount as i64,
        );
        for quantity in recipe.inputs.iter() {
            let prev = *need.get(&quantity.symbol).unwrap_or(&0);
            need.insert(
                quantity.symbol,
                prev + num_times_to_make * quantity.amount as i64,
            );
        }

        num_to_make = need
            .iter()
            .filter(|(&k, &v)| k != *ore_sym && v > 0)
            .count();
    }

    need[ore_sym]
}

#[aoc(day14, part1)]
pub fn solve_day14_part1(inp: &(Vec<Recipe>, Sym, Sym)) -> i64 {
    solve_day14_part1_h(inp, 1)
}

#[aoc(day14, part2)]
pub fn solve_day14_part2(inp: &(Vec<Recipe>, Sym, Sym)) -> i64 {
    // Binary search
    let mut lower = 0;
    let mut upper = 100_000_000;
    let mut fuel = upper / 2;
    loop {
        let ore_cost = solve_day14_part1_h(inp, fuel);
        if ore_cost < 1_000_000_000_000 {
            lower = fuel;
        } else if ore_cost > 1_000_000_000_000 {
            upper = fuel;
        }

        // Test one below upper limit
        if solve_day14_part1_h(inp, upper - 1) <= 1_000_000_000_000
            && solve_day14_part1_h(inp, upper) > 1_000_000_000_000
        {
            return upper - 1;
        }
        if solve_day14_part1_h(inp, upper) <= 1_000_000_000_000 {
            panic!("Upper bound too low!");
        }

        fuel = (upper - lower) / 2 + lower;
    }
}

#[test]
fn test_day14_p1_short_1() {
    let input = "10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part1(&parsed);
    assert_eq!(result, 31);
}

#[test]
fn test_day14_p1_short_2() {
    let input = "9 ORE => 2 A
    8 ORE => 3 B
    7 ORE => 5 C
    3 A, 4 B => 1 AB
    5 B, 7 C => 1 BC
    4 C, 1 A => 1 CA
    2 AB, 3 BC, 4 CA => 1 FUEL";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part1(&parsed);
    assert_eq!(result, 165);
}

#[test]
fn test_day14_p1_large_1() {
    let input = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part1(&parsed);
    assert_eq!(result, 13312);
}

#[test]
fn test_day14_p1_large_2() {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part1(&parsed);
    assert_eq!(result, 180697);
}

#[test]
fn test_day14_p1_large_3() {
    let input = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part1(&parsed);
    assert_eq!(result, 2210736);
}

#[test]
fn test_day14_p2_long_1() {
    let input = "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part2(&parsed);
    assert_eq!(result, 82892753);
}

#[test]
fn test_day14_p2_large_2() {
    let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
    17 NVRVD, 3 JNWZP => 8 VPVL
    53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
    22 VJHF, 37 MNCFX => 5 FWMGM
    139 ORE => 4 NVRVD
    144 ORE => 7 JNWZP
    5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
    5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
    145 ORE => 6 MNCFX
    1 NVRVD => 8 CXFTF
    1 VJHF, 6 MNCFX => 4 RFSQX
    176 ORE => 6 VJHF";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part2(&parsed);
    assert_eq!(result, 5586022);
}

#[test]
fn test_day14_p2_large_3() {
    let input = "171 ORE => 8 CNZTR
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
    114 ORE => 4 BHXH
    14 VRPVC => 6 BMBT
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
    5 BMBT => 4 WPTQ
    189 ORE => 9 KTJDG
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
    12 VRPVC, 27 CNZTR => 2 XDBXC
    15 KTJDG, 12 BHXH => 5 XCVML
    3 BHXH, 2 VRPVC => 7 MZWV
    121 ORE => 7 VRPVC
    7 XCVML => 6 RJRHP
    5 BHXH, 4 VRPVC => 5 LTCX";
    let parsed = input_generator_day14(input);
    let result = solve_day14_part2(&parsed);
    assert_eq!(result, 460664);
}
