use crate::utils::solution::{Solution, solution};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Default)]
pub struct SecretSafe;

impl Solution for SecretSafe {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let mut dial: i32 = 50;
        let mut zero_end_count = 0;
        let mut zero_total_count = 0;

        for line in input.lines() {
            let direction = match line.chars().nth(0).unwrap() {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            };

            let count = line[1..].parse::<i32>().unwrap();
            let count = match direction {
                Direction::Left => -count,
                Direction::Right => count,
            };

            zero_total_count += match direction {
                Direction::Right => (dial + count) / 100,
                Direction::Left => (99 - (dial + 99) % 100 - count) / 100,
            };

            dial = abs_mod(dial + count, 100);
            if dial == 0 {
                zero_end_count += 1;
            }
        }

        solution!(zero_end_count, zero_total_count)
    }
}

fn abs_mod(x: i32, m: i32) -> i32 {
    (x % m + m) % m
}
