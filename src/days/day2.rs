use aoc_runner_derive::{aoc, aoc_generator};

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
            if v[0] == 19_690_720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No noun/verb combination found!");
}
