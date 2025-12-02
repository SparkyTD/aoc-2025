use std::fmt::Display;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::{Duration, Instant};
use colored::Colorize;
use crate::utils::aoc::format_elapsed;

pub static PRINT_RESULTS: AtomicBool = AtomicBool::new(true);

pub struct Test {
    name: String,
    input_text: String,
    output_text_1: Option<String>,
    output_text_2: Option<String>,
    part1_name: String,
    part2_name: String,
}

impl Test {
    pub fn check_result_1(&self, result: &str) -> Option<bool> {
        self.check_result(result, self.output_text_1.clone(), &self.part1_name)
    }

    pub fn check_result_2(&self, result: &str) -> Option<bool> {
        self.check_result(result, self.output_text_2.clone(), &self.part2_name)
    }

    fn check_result(&self, result: &str, correct_result: Option<String>, label: &str) -> Option<bool> {
        if PRINT_RESULTS.load(Relaxed) {
            print!("   {}: {} ", label.bold(), result.bright_blue());
        }
        if let Some(correct_result) = correct_result {
            let matches = correct_result.eq(result);
            if PRINT_RESULTS.load(Relaxed) {
                if matches {
                    println!("{}", "[Success]".bright_green().bold())
                } else {
                    println!("{} (should be {})", "[Fail]".red().bold(), correct_result.yellow())
                }
            }
            Some(matches)
        } else {
            if PRINT_RESULTS.load(Relaxed) {
                println!("{}", "[Unknown]".white());
            }
            None
        }
    }

    pub fn get_input(&self) -> String {
        self.input_text.clone()
    }
}

pub struct TestRunResult {
    pub elapsed: Duration,
    pub part1_success: Option<bool>,
    pub part2_success: Option<bool>,
}

pub struct TestSet {
    tests: Vec<Test>,
}

impl TestSet {
    pub fn from(raw_input: &str) -> Self {
        let mut tests = Vec::new();
        let mut current_test: Option<Test> = None;
        let mut line_number = 0;
        let mut part1_name = "Part 1".to_string();
        let mut part2_name = "Part 2".to_string();

        for line in raw_input.lines() {
            line_number += 1;

            if line.trim().to_lowercase().starts_with("@test") {
                if let Some(current_test) = current_test.take() {
                    tests.push(current_test);
                }
                current_test.replace(Test {
                    name: format!("Test {}", tests.len() + 1),
                    input_text: String::new(),
                    output_text_1: None,
                    output_text_2: None,
                    part1_name: part1_name.clone(),
                    part2_name: part2_name.clone(),
                });
            } else if line.trim().to_lowercase().starts_with("@part1") {
                if let Some(current_test) = current_test.as_mut() {
                    let mut split = line.split_whitespace();
                    _ = split.next();
                    current_test.output_text_1.replace(split.next().unwrap().to_owned());
                } else {
                    panic!("Invalid data on line {}. Possible missing @test directive.", line_number);
                }
            } else if line.trim().to_lowercase().starts_with("@part2") {
                if let Some(current_test) = current_test.as_mut() {
                    let mut split = line.split_whitespace();
                    _ = split.next();
                    current_test.output_text_2.replace(split.next().unwrap().to_owned());
                } else {
                    panic!("Invalid data on line {}. Possible missing @test directive.", line_number);
                }
            } else if line.trim().to_lowercase().starts_with("@label1") {
                let mut split = line.split_whitespace();
                _ = split.next();
                part1_name = split.remainder().unwrap().to_owned();
            } else if line.trim().to_lowercase().starts_with("@label2") {
                let mut split = line.split_whitespace();
                _ = split.next();
                part2_name = split.remainder().unwrap().to_owned();
            } else {
                if let Some(current_test) = current_test.as_mut() {
                    current_test.input_text.push_str(line);
                    current_test.input_text.push_str("\n");
                }
            }
        }

        if let Some(current_test) = current_test.take() {
            tests.push(current_test);
        }

        tests.iter_mut().for_each(|test| {
            test.input_text = test.input_text.trim().to_string();
        });

        Self { tests }
    }

    pub fn get_test(&self, index: usize) -> &Test {
        &self.tests[index]
    }

    pub fn len(&self) -> usize {
        self.tests.len()
    }

    pub fn test_all<F, R>(&self, f: F) -> bool
    where
        F: Fn(String) -> (R, R),
        R: Display,
    {
        let mut all_successful = true;
        for i in 0..self.tests.len() {
            let result = self.test_one(i, &f);
            all_successful &= result.part1_success.unwrap_or(true) && result.part2_success.unwrap_or(true);
        }

        println!();
        if all_successful {
            println!("All tests {}!", "succeeded".bright_green().bold());
        } else {
            println!("Some test have {}!", "failed".red().bold())
        }

        all_successful
    }

    pub fn test_one<F, R>(&self, index: usize, f: F) -> TestRunResult
    where
        F: Fn(String) -> (R, R),
        R: Display,
    {
        let test = self.get_test(index);
        let start_time = Instant::now();
        let (part1, part2) = f(test.get_input());
        let elapsed = start_time.elapsed();

        if PRINT_RESULTS.load(Relaxed) {
            println!();
            println!("{} Results:", test.name.bold());
        }
        let test1_result = test.check_result_1(part1.to_string().as_str());
        let test2_result = test.check_result_2(part2.to_string().as_str());
        if PRINT_RESULTS.load(Relaxed) {
            println!("{}: {}", "Elapsed time".bold(), format_elapsed(elapsed, true).purple());
        }

        TestRunResult {
            part1_success: test1_result,
            part2_success: test2_result,
            elapsed,
        }
    }
}