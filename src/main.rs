#![feature(str_split_whitespace_remainder)]

use crate::days::day1::SecretSafe;
use crate::days::day2::ProductCodes;
use crate::utils::aoc::AdventOfCode;
use crate::utils::solution::SolveTest;

mod utils;
mod days;

fn main() {
    let mut aoc = AdventOfCode::default();
    aoc.add_solution(1, Box::new(SecretSafe::default()));
    aoc.add_solution(2, Box::new(ProductCodes::default()));

    aoc.solve_day(2, SolveTest::Last);
    aoc.solve_all();
}
