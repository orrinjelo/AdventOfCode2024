mod problems;
mod util;

use log::{info, warn, debug}; // trace, debug, info, warn, error, Level
use env_logger;
// use std::io::Write;
use std::time::{Instant, Duration};
use rustop::opts;
use crate::util::RetType;

// A macro to simplify a complex ternary
macro_rules! ifelse {
    ($c:expr, $v:expr, $v1:expr) => {
        if $c {$v} else {$v1}
    };
}

// Function to format a timestamp as millisecs or microsecs
fn format_time(ts: Duration) -> String {
    if ts.as_micros() >= 1_000_000 {
        return format!("{} s", (ts.as_millis() as f32)/1000.0);
    } else if ts.as_micros() >= 1000 {
        return format!("{} ms", (ts.as_micros() as f32)/1000.0);
    } 
    return format!("{} μs", ts.as_micros());
}

// Execute and print the answers and execution time
fn execute_problem(num: i32, input: Vec<String>, part1: fn(Vec<String>) -> RetType, part2: fn(Vec<String>) -> RetType) {
    let start = Instant::now();
    let result_1 = part1(input.clone());
    let then_elapsed = start.elapsed();
    let then = Instant::now();
    let result_2 = part2(input.clone());
    let end_elapsed = then.elapsed();
    info!("Problem {}; Part 1: {} (Runtime: {})", num, result_1, format_time(then_elapsed));
    info!("Problem {}; Part 2: {} (Runtime: {})", num, result_2, format_time(end_elapsed));    
}

fn benchmark_problem(num: i32, input: Vec<String>, part1: fn(Vec<String>) -> RetType, part2: fn(Vec<String>) -> RetType) {
    let start = Instant::now();
    for _ in 0..1000 {
        part1(input.clone());
    }
    let then_elapsed = start.elapsed() / 1000;
    let then = Instant::now();
    for _ in 0..1000 {
        part2(input.clone());
    }
    let end_elapsed = then.elapsed() / 1000;
    info!("Problem {}; Part 1 Runtime: {}", num, format_time(then_elapsed));
    info!("Problem {}; Part 2 Runtime: {}", num, format_time(end_elapsed));    
}

fn run_problem(num: i32, input: Vec<String>) {
    match num {
        // Example problem (problem from a previous year!)
        0 => execute_problem(num, input, problems::problem00::problem_001, problems::problem00::problem_002),
        // Add problems here as they arrive
        1 => execute_problem(num, input, problems::problem01::problem_011, problems::problem01::problem_012),
        2 => execute_problem(num, input, problems::problem02::problem_021, problems::problem02::problem_022),
        3 => execute_problem(num, input, problems::problem03::problem_031, problems::problem03::problem_032),
        _ => warn!("Problem number not available.")
    }
}

fn run_benchmark_problem(num: i32, input: Vec<String>) {
    match num {
        // Example problem (problem from a previous year!)
        0 => benchmark_problem(num, input, problems::problem00::problem_001, problems::problem00::problem_002),
        // Add problems here as they arrive
        1 => benchmark_problem(num, input, problems::problem01::problem_011, problems::problem01::problem_012),
        2 => benchmark_problem(num, input, problems::problem02::problem_021, problems::problem02::problem_022),
        3 => benchmark_problem(num, input, problems::problem03::problem_031, problems::problem03::problem_032),
        _ => warn!("Problem number not available.")
    }
}


fn visualize_problem(num: i32, input: Vec<String>) {
    match num {
        0 => problems::problem00::visualize_00(input),
        _ => warn!("Visualization not available for {}", num)
    }
}

fn main() {
    // Set up logging
    env_logger::builder()
        .init();

    // Set up runtime arguments
    let opts = opts! {
        synopsis "Advent of Code 2024";
        opt run_all:bool, desc: "Run all problems.";
        opt input_file:Option<String>, desc: "Custom input file for this problem.";
        opt visualization:bool, desc: "Run the visualization.";
        opt benchmark:bool, desc: "Run benchmarking.";
        param number:Option<i32>, desc:"Problem number to run.";
    };

    let (args, _rest) = opts.parse_or_exit();

    // debug!("{:?}", args.number);

    info!("==== Advent of Code 2024 ====");

    // Parse args
    if args.run_all {
        for i in 0..3 {
            let filename = format!("aoc2024/inputs/{:02}.txt", i).to_string();
            let input = util::load_file(filename);
            run_problem(i, input);
            info!("=========================");
            // No visualization
        }
    } else if args.benchmark {
        if let Some(num) = args.number {
            let filename = ifelse!(args.input_file.is_none(), format!("aoc2024/inputs/{:02}.txt", num).to_string(), args.input_file.unwrap());
            debug!("filename: {:?}", filename);
            let input = util::load_file(filename);
            run_benchmark_problem(num, input.clone());
        }
    } else {
        if let Some(num) = args.number {
            let filename = ifelse!(args.input_file.is_none(), format!("aoc2024/inputs/{:02}.txt", num).to_string(), args.input_file.unwrap());
            debug!("filename: {:?}", filename);
            let input = util::load_file(filename);
            run_problem(num, input.clone());
            if args.visualization {
                visualize_problem(num, input);
            }
        }
    }
}