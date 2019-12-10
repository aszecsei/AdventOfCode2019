use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

// ======================================================
// DAY 10
// ======================================================

#[aoc_generator(day10)]
pub fn input_generator_day10(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| *ch == '#')
                .map(move |(x, _ch)| (x as i64, y as i64))
        })
        .flatten()
        .collect_vec()
}

fn dist_vec(a: &(i64, i64), b: &(i64, i64)) -> (i64, i64) {
    ((b.0 - a.0), (b.1 - a.1))
}

/// Returns true if `c` blocks LOS of `a` to `b`.
fn is_blocking(a: &(i64, i64), b: &(i64, i64), c: &(i64, i64)) -> bool {
    let dist_ba = dist_vec(b, a);
    let dist_ca = dist_vec(c, a);
    if dist_ba.0.signum() != dist_ca.0.signum() || dist_ba.1.signum() != dist_ca.1.signum() {
        return false;
    }
    if dist_ca.0.abs() + dist_ca.1.abs() >= dist_ba.0.abs() + dist_ba.1.abs() {
        // C is further from `a` than `b`; it cannot block LOS
        return false;
    }

    dist_ba.0 * dist_ca.1 == dist_ca.0 * dist_ba.1
}

fn visible_asteroids(base: &(i64, i64), asteroids: &[(i64, i64)]) -> usize {
    asteroids
        .iter()
        .filter(|&a| {
            // Determine if there's another astroid blocking LOS
            a != base && asteroids.iter().all(|b| !is_blocking(base, a, b))
        })
        .count()
}

#[aoc(day10, part1)]
pub fn solve_day10_part1(input: &[(i64, i64)]) -> usize {
    let mut max_visible = 0;

    for base in input.iter() {
        let va = visible_asteroids(base, input);
        max_visible = std::cmp::max(max_visible, va);
    }

    max_visible
}

#[aoc(day10, part2)]
pub fn solve_day10_part2(input: &[(i64, i64)]) -> usize {
    let base = {
        let mut max_visible = 0;
        let mut best_base = input[0];

        for base in input.iter() {
            let va = visible_asteroids(base, input);
            if va > max_visible {
                max_visible = va;
                best_base = *base;
            }
        }
        best_base
    };

    println!("Base: {:?}", base);

    let mut asteroids = input.to_vec();
    let mut counter = 199;
    loop {
        let va = visible_asteroids(&base, &asteroids) as i64;
        if counter - va >= 0 {
            let ast2 = asteroids.clone();
            asteroids.retain(|a| a == &base || ast2.iter().any(|b| is_blocking(&base, a, b)));
            counter -= va;
        } else {
            break;
        }
    }

    // We now have `counter` asteroids to destroy, in order.
    // However, we know all asteroids to be destroyed *must* be unblocked (otherwise, the loop would not have exited!)
    {
        let ast2 = asteroids.clone();
        asteroids.retain(|a| a != &base && ast2.iter().all(|b| !is_blocking(&base, a, b)));
    }

    println!("Deleted all but {} asteroids...", asteroids.len());

    const PI_OVER_TWO: f64 = std::f64::consts::PI / 2f64;
    const TWO_PI: f64 = std::f64::consts::PI * 2f64;

    // Calculate the angles of each asteroid
    let mut asteroid_angles = asteroids
        .iter()
        .map(|a| {
            // We eventually want to sort by the angle between the asteroid and the base. To get the angle, we can use atan2()!
            // In Rust, atan2 has self = y, and other = x -- that is, we call y.atan2(x)
            // atan2 gives us the angle starting pointing right - we want to start pointing vertically! Thus, we add 90 degrees (or pi/2) to the resulting angle.
            let mut angle = ((a.1 - base.1) as f64).atan2((a.0 - base.0) as f64) + PI_OVER_TWO;
            // Re-constrain ourselves to the range [0, 2pi)
            while angle >= TWO_PI {
                angle -= TWO_PI;
            }
            while angle < 0f64 {
                angle += TWO_PI;
            }
            (a, angle)
        })
        .collect_vec();

    asteroid_angles.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let selected_asteroid = asteroid_angles[counter as usize].0;

    selected_asteroid.0 as usize * 100 + selected_asteroid.1 as usize
}

#[test]
fn test_is_blocking() {
    assert_eq!(is_blocking(&(3, 4), &(1, 0), &(2, 2)), true);
    assert_eq!(is_blocking(&(3, 4), &(0, 0), &(2, 2)), false);

    assert_eq!(is_blocking(&(0, 0), &(1, 0), &(2, 0)), false);
    assert_eq!(is_blocking(&(0, 0), &(2, 0), &(1, 0)), true);
    assert_eq!(is_blocking(&(0, 0), &(2, 0), &(2, 0)), false);

    assert_eq!(is_blocking(&(0, 0), &(4, 6), &(2, 3)), true);
    assert_eq!(is_blocking(&(0, 0), &(3, 9), &(2, 6)), true);

    assert_eq!(is_blocking(&(1, 0), &(4, 3), &(3, 2)), true);
    assert_eq!(is_blocking(&(1, 0), &(3, 4), &(2, 2)), true);
}

#[test]
fn test_visible_asteroids() {
    let ex = ".#..#\n.....\n#####\n....#\n...##";
    let inp = input_generator_day10(ex);

    let base = inp[0];
    let va = visible_asteroids(&base, &inp);
    assert_eq!(va, 7);
}

#[test]
fn test_pt1_1() {
    let ex = ".#..#\n.....\n#####\n....#\n...##";
    let inp = input_generator_day10(ex);

    let res = solve_day10_part1(&inp);
    assert_eq!(res, 8);
}

#[test]
fn test_pt1_2() {
    let ex = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####";
    let inp = input_generator_day10(ex);
    let res = solve_day10_part1(&inp);
    assert_eq!(res, 33);
}

#[test]
fn test_pt1_3() {
    let ex = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
    let inp = input_generator_day10(ex);
    let res = solve_day10_part1(&inp);
    assert_eq!(res, 35);
}

#[test]
fn test_pt1_4() {
    let ex = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
    let inp = input_generator_day10(ex);
    let res = solve_day10_part1(&inp);
    assert_eq!(res, 41);
}

#[test]
fn test_pt1_5() {
    let ex = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    let inp = input_generator_day10(ex);
    let res = solve_day10_part1(&inp);
    assert_eq!(res, 210);
}

#[test]
fn test_pt2() {
    let ex = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    let inp = input_generator_day10(ex);
    let res = solve_day10_part2(&inp);
    assert_eq!(res, 802);
}
