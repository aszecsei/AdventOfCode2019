#![warn(clippy::all)]

mod days;
mod helper;
mod shared;

use aoc_runner_derive::aoc_lib;

pub use days::*;

aoc_lib! { year = 2019 }
