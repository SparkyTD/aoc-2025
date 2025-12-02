use std::collections::HashMap;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use chrono::Datelike;
use colored::Colorize;
use terminal_size::Width;
use crate::utils::solution::{Solution, SolveTest};
use crate::utils::test_set::{TestRunResult, PRINT_RESULTS};

#[derive(Default)]
pub struct AdventOfCode {
    solutions: HashMap<u8, Box<dyn Solution>>,
}

impl AdventOfCode {
    pub fn add_solution(&mut self, day: u8, solution: Box<dyn Solution>) {
        if let Some(_) = self.solutions.insert(day, solution) {
            panic!("A solution has already been added for day {day}!")
        }
    }

    #[allow(dead_code)]
    pub fn solve_day(&self, day: u8, solve_test: SolveTest) {
        if let Some(solution) = self.solutions.get(&day) {
            solution.solve_test(day, solve_test);
        } else {
            panic!("No solution exists for day {day}!");
        }
    }

    #[allow(dead_code)]
    pub fn bench_day(&self, day: u8, solve_test: SolveTest) {
        if let Some(solution) = self.solutions.get(&day) {
            PRINT_RESULTS.store(false, Relaxed);

            let iterations = 100;
            let mut total_time = 0;

            for _ in 0..iterations {
                match solution.solve_test(day, solve_test) {
                    Some(result) => {
                        total_time += result.elapsed.as_micros();
                    }
                    None => {
                        println!("{}", "The program did not return a solution, aborting benchmark!".red().bold());
                        return;
                    }
                }
            }

            let duration = Duration::from_micros((total_time / iterations) as u64);
            println!("Average execution time: {}", format_elapsed(duration, true));

            PRINT_RESULTS.store(true, Relaxed);
        } else {
            panic!("No solution exists for day {day}!");
        }
    }

    #[allow(dead_code)]
    pub fn solve_all(&self) {
        let prev_print_results = PRINT_RESULTS.load(Relaxed);
        PRINT_RESULTS.store(false, Relaxed);
        let mut keys = self.solutions.keys().collect::<Vec<&u8>>();
        keys.sort();

        let mut results: HashMap<u8, Option<TestRunResult>> = HashMap::new();

        for day in &keys {
            let solution = self.solutions.get(day).unwrap();
            println!("Running solution for day {day}...");
            let result = solution.solve_test(**day, SolveTest::Last);
            results.insert(**day, result);
        }

        println!();

        let longest_duration = Self::get_longest_duration(&results);

        println!("All solutions have been executed, here are the results:");
        for day in &keys {
            let result = results.get(day).unwrap();
            let status_label = match result {
                None => "[Inconclusive]".white(),
                Some(result) => match (result.part1_success, result.part2_success) {
                    (Some(true), Some(true)) | (Some(true), None) | (None, Some(true)) => "[Success]".bright_green().bold(),
                    (Some(false), _) | (_, Some(false)) => " [Fail!] ".red().bold(),
                    (None, None) => "[Inconclusive]".white(),
                }
            };
            let duration_label = match result {
                None => "",
                Some(result) => &format_elapsed(result.elapsed, true),
            };
            let progress_label = match result {
                None => "",
                Some(result) => &format_progress_bar(&result.elapsed, &longest_duration, true),
            };
            println!("   Day {: >2}: {} {}  {}", format!("{}", day).purple().bold(), status_label, progress_label, duration_label);
        }
        PRINT_RESULTS.store(prev_print_results, Relaxed);

        self.check_date_and_print_link();
        Self::write_progress_report(results);
    }

    fn check_date_and_print_link(&self) {
        let local_date = chrono::Utc::now() - chrono::Duration::hours(7);
        if local_date.month() != 12 {
            return;
        }

        let mut keys = self.solutions.keys().collect::<Vec<&u8>>();
        keys.sort();

        let last_day_with_solution = **keys.last().unwrap() as u32;

        if local_date.day() > last_day_with_solution && last_day_with_solution < 25 {
            let next_day = last_day_with_solution + 1;
            let problem_url = format!("https://adventofcode.com/{}/day/{}", local_date.year(), next_day);
            println!();
            println!("{}: {}", "Your next AoC problem is ready! Grab it here".purple().bold().italic(), problem_url);
        }
    }

    fn write_progress_report(results: HashMap<u8, Option<TestRunResult>>) {
        if cfg!(debug_assertions) {
            return;
        }
        let readme_file_name = "README.md";
        let mut git_root = std::env::current_dir().unwrap();
        while !git_root.join(readme_file_name).exists() {
            if let Some(parent_path) = git_root.parent() {
                git_root = parent_path.to_path_buf();
            } else {
                return;
            }
        }

        let mut keys = results.keys().collect::<Vec<&u8>>();
        keys.sort();

        let readme_content = std::fs::read_to_string(git_root.join(readme_file_name)).unwrap();
        let readme_lines = readme_content.split('\n').collect::<Vec<_>>();

        let mut should_rewrite_readme = false;
        let mut lines = Vec::new();
        let longest_duration = Self::get_longest_duration(&results);

        let mut reading_table = false;
        let mut table_written = false;
        for line in readme_lines {
            if line.contains("{RESULTS_START}") {
                reading_table = true;
                lines.push(line.to_string());
            } else if line.contains("{RESULTS_END}") {
                reading_table = false;
                lines.push(line.to_string());
                continue;
            } else if line.starts_with("<!-- {DAY ") {
                let parts = line.split([' ', '}', '=']).collect::<Vec<_>>();
                let day = parts[2].parse::<u8>().unwrap();
                let mut stored_micros = parts[3].parse::<u128>().unwrap() as f64;

                if let Some(result) = results.get(&day).and_then(|res| res.as_ref()) {
                    let mut result_micros = result.elapsed.as_micros() as f64;
                    if stored_micros > result_micros {
                        (stored_micros, result_micros) = (result_micros, stored_micros);
                    }
                    if 1.0 - stored_micros / result_micros >= 0.1 {
                        should_rewrite_readme = true;
                    }
                }
            }

            if reading_table && !table_written {
                lines.push(format_args!("| Day | Status | Execution Time Comparison | Execution Time |\n").to_string());
                lines.push(format_args!("|-----|--------|---------------------------|----------------|\n").to_string());
                for day in &keys {
                    let result = results.get(day).unwrap();
                    let duration_label = match result {
                        None => "",
                        Some(result) => &format_elapsed(result.elapsed, false),
                    };
                    let status_badge = match result {
                        None => "![Static Badge](https://img.shields.io/badge/Inconclusive-grey?style=flat)",
                        Some(result) => match (result.part1_success, result.part2_success) {
                            (Some(true), Some(true)) => "![Static Badge](https://img.shields.io/badge/Success-green?style=flat)",
                            (Some(true), Some(false)) | (Some(false), Some(true)) => "![Static Badge](https://img.shields.io/badge/Failed-red?style=flat)",
                            (None, Some(_)) | (Some(_), None) | _ => "![Static Badge](https://img.shields.io/badge/Inconclusive-grey?style=flat)"
                        }
                    };
                    let progress_label = {
                        if let Some(result) = result {
                            let max_micros = longest_duration.as_micros();
                            let result_micros = result.elapsed.as_micros();
                            let percentage = 100 * result_micros / max_micros;
                            format!("![Static Badge](https://progress-bar.xyz/{}/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)", percentage)
                        } else {
                            "".to_string()
                        }
                    };
                    let day_label = format!("[{}](src/days/day{}.rs)", day, day);
                    lines.push(format_args!("|{}|{}|{}|{}|\n", day_label, status_badge, progress_label, duration_label.clear()).to_string());
                }

                for day in &keys {
                    let result = results.get(day).unwrap();
                    let duration_micros = {
                        if let Some(result) = result {
                            result.elapsed.as_micros()
                        } else {
                            u128::MAX
                        }
                    };
                    lines.push(format_args!("<!-- {{DAY {}={}}} -->\n", day, duration_micros).to_string());
                }
                table_written = true;
            } else if !reading_table {
                lines.push(line.to_string());
            }
        }

        if should_rewrite_readme {
            std::fs::write(git_root.join(readme_file_name), lines.iter().map(|s| s.trim()).collect::<Vec<_>>().join("\n")).unwrap();
        }
    }

    fn get_longest_duration(results: &HashMap<u8, Option<TestRunResult>>) -> Duration {
        let mut longest_duration = results.values()
            .filter_map(|value| value.as_ref())
            .map(|result| result.elapsed)
            .max().unwrap_or(Duration::from_secs(0));

        if longest_duration.as_secs() < 5 {
            longest_duration *= 6;
            longest_duration /= 5;
        }

        longest_duration
    }
}

fn format_progress_bar(current_duration: &Duration, max_duration: &Duration, colorize: bool) -> String {
    let terminal_width = terminal_size::terminal_size()
        .and_then(|(Width(w), terminal_size::Height(_))| Some(w as usize))
        .unwrap_or(200)
        .min(200);

    let bar_width = terminal_width / 4;
    let progress = current_duration.as_secs_f64() / max_duration.as_secs_f64();
    let filled_width = (progress * bar_width as f64).round() as usize;

    let filled_bar = "=".repeat(filled_width);
    let empty_bar = " ".repeat(bar_width.saturating_sub(filled_width));

    if colorize {
        format!("[{}{}{}{}]", "=".purple(), filled_bar.purple(), ">".bright_purple(), empty_bar)
    } else {
        format!("[{}{}{}{}]", "=", filled_bar, ">", empty_bar)

    }
}

pub fn format_elapsed(duration: Duration, colorize: bool) -> String {
    let millis = duration.as_millis();
    let micros = duration.as_micros();
    let seconds = duration.as_secs();
    let result = if micros < 1000 {
        format!("{}µs", micros)
    } else if millis < 1000 {
        format!("{}ms", millis)
    } else if seconds <= 5 {
        format!("{}s {}ms", duration.as_secs(), millis % 1000)
    } else {
        format!("{}s {}ms", duration.as_secs(), millis % 1000)
    };

    if !colorize {
        result
    } else {
        if micros < 1000 {
            result.bright_cyan().to_string()
        } else if millis < 1000 {
            result.bright_green().to_string()
        } else if seconds <= 5 {
            result.yellow().bold().to_string()
        } else {
            result.red().bold().to_string()
        }
    }
}