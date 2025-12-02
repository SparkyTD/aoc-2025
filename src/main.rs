#![feature(str_split_whitespace_remainder)]

use crate::days::day1::SecretSafe;
use crate::utils::aoc::AdventOfCode;
use crate::utils::solution::SolveTest;

mod utils;
mod days;

fn main() {
    let mut aoc = AdventOfCode::default();
    aoc.add_solution(1, Box::new(SecretSafe::default()));

    //aoc.solve_day(1, SolveTest::All);
    aoc.solve_all();
}
