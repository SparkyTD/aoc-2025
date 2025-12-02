use std::fmt::Display;
use rayon::prelude::*;
use crate::utils::solution::{solution, Solution};

#[derive(Default)]
pub struct ProductCodes;

impl Solution for ProductCodes {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>) {
        let ranges = input
            .split(',')
            .map(|s| s.split('-'))
            .map(|mut r| (r.next(), r.next()))
            .map(|(r1, r2)| {
                let start = r1.unwrap().parse::<u64>().unwrap();
                let end = r2.unwrap().parse::<u64>().unwrap();
                Range { start, end }
            })
            .collect::<Vec<_>>();

        let mut sum_1: u64 = 0;
        let mut sum_2: u64 = 0;
        for range in ranges {
            let sums = (range.start..=range.end).into_par_iter().map(|i| {
                let mut sum_1: u64 = 0;
                let mut sum_2: u64 = 0;

                let id = i.to_string();
                let mid = id.len() / 2;
                let start = &id[..mid];
                let end = &id[mid..];
                if start == end {
                    sum_1 = i;
                }

                let mut has_valid = false;
                for len in 1..=id.len() / 2 {
                    if id.len() % len != 0 {
                        continue;
                    }

                    let seg0 = &id[..len];
                    let reassembled = seg0.repeat(id.len() / len);
                    if reassembled == id {
                        has_valid = true;
                    }
                }

                if has_valid {
                    sum_2 = i;
                }

                (sum_1, sum_2)
            });

            sum_1 += sums.clone().map(|(sum_1, _)| sum_1).sum::<u64>();
            sum_2 += sums.map(|(_, sum_2)| sum_2).sum::<u64>();
        }

        solution!(sum_1, sum_2)
    }
}

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}
