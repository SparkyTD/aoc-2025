# ❄️ Advent of Code 2025 ❄️
This is my second year participating in Advent of Code.

## My solutions so far:
<!-- {RESULTS_START} -->
| Day | Status | Execution Time Comparison | Execution Time |
|-----|--------|---------------------------|----------------|
|[1](src/days/day1.rs)|![Static Badge](https://img.shields.io/badge/Success-green?style=flat)|![Static Badge](https://progress-bar.xyz/83/?width=500&progress_color=8935D9&progress_background=404040&show_text=false)|47µs|
<!-- {DAY 1=47} -->
<!-- {RESULTS_END} -->
*The table above is automatically generated with each execution of the test suite.*

**NOTE:** Every solution has been validated against **two separate inputs** from two different [adventofcode.com](https://adventofcode.com) accounts.
A `Success` badge next to a day means that the solution produced the correct results for both **Path 1 and Part 2** of **both** input sets.

## Running individual solutions
If you would like to run any of my solutions against your own input, the easiest way to do so if **cloning this repository**
and converting the input provided by AoC to the [custom test format](#test-input-format) used by this project.

If you don't want to clone the entire project, and just want to run a single solution, you can simply copy the contents of
the `fn solve(&self, input: String)` function to an empty Rust project's `main()`. Make sure to also copy any additional
structs, enums or functions that are also defined in the solution's module file, as well as any potential utility structures
from the [utils](src/utils) folder (e.g. [matrix.rs](src/utils/matrix.rs), [prefix_tree.rs](src/utils/prefix_tree.rs),
[position.rs](src/utils/position.rs), [facing.rs](src/utils/facing.rs)).

## Test Input Format
To respect the rules of Advent of Code, as well as the event's creator, my personalized inputs are not included in this repository.
This project uses a custom test file format that allows multiple different test cases to be checked easily.
Each test is prefixed with `@test`, which is followed by the program input. To perform result checking,
the `@test` tag can be extended with `@part1 <PART1_RESULT>` and/or `@part2 <PART1_RESULT>`.

Here's an example test file (`./data/day17.test`)
```
@test
@part1 4,6,3,5,6,3,5,2,1,0
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0

@test
@part2 117440
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0

@test
@part1 [Redacted]
@part2 [Redacted]
Register A: [Redacted]
Register B: 0
Register C: 0

Program: [Redacted]
```