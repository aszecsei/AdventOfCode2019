use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

// ======================================================
// DAY 8
// ======================================================

#[aoc_generator(day8)]
pub fn input_generator_day7(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect_vec()
}

#[aoc(day8, part1)]
pub fn solve_day8_part1(input: &[u8]) -> usize {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;
    let img = {
        let mut layers = vec![];
        
        let mut idx = 0;
        let mut layer = [0u8; WIDTH * HEIGHT];
        for &pix in input.iter() {
            layer[idx] = pix;
            idx += 1;
            if idx >= WIDTH * HEIGHT {
                idx = 0;
                layers.push(layer);
            }
        }
        layers
    };

    let min_zero_layer = img.iter().min_by(|&x, &y| {
        let xzc = bytecount::naive_count_32(x, 0);
        let yzc = bytecount::naive_count_32(y, 0);
        xzc.cmp(&yzc)
    }).unwrap();

    let noc = bytecount::naive_count_32(min_zero_layer, 1);
    let ntc = bytecount::naive_count_32(min_zero_layer, 2);

    noc * ntc
}

#[aoc(day8, part2)]
pub fn solve_day8_part2(input: &[u8]) -> String {
    const WIDTH: usize = 25;
    const HEIGHT: usize = 6;

    let mut res = [2u8; WIDTH * HEIGHT];

    let img = {
        let mut layers = vec![];
        
        let mut idx = 0;
        let mut layer = [0u8; WIDTH * HEIGHT];
        for &pix in input.iter() {
            layer[idx] = pix;
            idx += 1;
            if idx >= WIDTH * HEIGHT {
                idx = 0;
                layers.push(layer);
            }
        }
        layers
    };

    for layer in img.iter() {
        for (idx, &pix) in layer.iter().enumerate() {
            if pix != 2 && res[idx] == 2 {
                res[idx] = pix;
            }
        }
    }

    let mut printable = String::from("\n");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = y * WIDTH + x;
            if res[idx] == 0 {
                printable.push(' ');
            } else {
                printable.push('O');
            }
        }
        printable.push('\n');
    }

    printable
}