use std::fmt::Display;
use crate::utils::test_set::{TestRunResult, TestSet};

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum SolveTest {
    All,
    Last,
    Index(usize),
}

macro_rules! solution {
    ($a:expr, $b:expr) => {
        (Box::new($a), Box::new($b))
    };
}

pub(crate) use solution;

pub trait Solution {
    fn solve(&self, input: String) -> (Box<dyn Display>, Box<dyn Display>);

    fn solve_test(&self, day: u8, solve_test: SolveTest) -> Option<TestRunResult> {
        let exe_path = std::env::current_dir().unwrap();
        let data_path = exe_path.join("data");
        if !data_path.exists() {
            panic!("No data folder found at {}", data_path.to_str().unwrap());
        }

        let input_path = data_path.join(format!("day{}.test", day));
        if !input_path.exists() {
            panic!("No test found at {}", input_path.to_str().unwrap());
        }

        let input_raw = std::fs::read_to_string(input_path).unwrap();
        let test_set = TestSet::from(&input_raw);

        match solve_test {
            SolveTest::All => { test_set.test_all(|input| self.solve(input)); None },
            SolveTest::Index(index) => Some(test_set.test_one(index, |input| self.solve(input))),
            SolveTest::Last => Some(test_set.test_one(test_set.len() - 1, |input| self.solve(input))),
        }
    }
}