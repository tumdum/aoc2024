use anyhow::Result;
use aoc24::input::read_input;
use clap::Parser;
use std::path::PathBuf;
use std::time::{Duration, Instant};

/// Solution for the Advent Of Code 2024
#[derive(Parser, Debug)]
#[command(version)]
struct Opt {
    /// Skip verification against known solutions
    #[arg(short, long)]
    skip_verification: bool,

    /// Solution to run
    #[arg(short, long)]
    day_to_run: Option<usize>,

    /// Passphrase
    #[arg(short, long, env)]
    passphrase: Option<String>,

    /// Input file for selected solution
    #[arg(short, long)]
    input_file: Option<PathBuf>,

    /// Skip printing output
    #[arg(long)]
    skip_output: bool,

    /// How many times each solution will be run, usefull for better time measurements
    #[arg(long, default_value_t = 1)]
    loops: usize,
}

fn main() {
    let opt = Opt::parse();
    let mut times = vec![];
    let mut times_io = vec![];

    let solutions: Vec<&dyn Fn(&str, bool, bool) -> Result<Duration>> = vec![
        &aoc24::day01::solve,
        &aoc24::day02::solve,
        &aoc24::day03::solve,
        &aoc24::day04::solve,
        &aoc24::day05::solve,
        &aoc24::day06::solve,
        &aoc24::day07::solve,
        // &aoc24::day08::solve,
        // &aoc24::day09::solve,
        // &aoc24::day10::solve,
        // &aoc24::day11::solve,
        // &aoc24::day12::solve,
        // &aoc24::day13::solve,
        // &aoc24::day14::solve,
        // &aoc24::day15::solve,
        // &aoc24::day16::solve,
        // &aoc24::day17::solve,
        // &aoc24::day18::solve,
        // &aoc24::day19::solve,
        // &aoc24::day20::solve,
        // &aoc24::day21::solve,
        // &aoc24::day22::solve,
        // &aoc24::day23::solve,
        // &aoc24::day24::solve,
        // &aoc24::day25::solve,
    ];

    let mut running_sum_compute = Duration::from_secs(0);
    let mut running_sum_io = Duration::from_secs(0);
    for (i, solution) in solutions.iter().enumerate() {
        if Some(i + 1) == opt.day_to_run || opt.day_to_run.is_none() {
            let input_file_path = match &opt.input_file {
                Some(path) => path.to_owned(),
                None => PathBuf::from(format!("inputs/day{:02}", i + 1)),
            };

            let mut solution_times = vec![];

            for i in 0..opt.loops {
                let (input, io_time) = read_input(&input_file_path, opt.passphrase.as_deref());

                let start = Instant::now();
                let t = match solution(
                    &input,
                    !opt.skip_verification,
                    if i == 0 { !opt.skip_output } else { false },
                ) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Solution {i} failed: {e}");
                        continue;
                    }
                };

                solution_times.push((t, start.elapsed() + io_time));
                if t > Duration::from_secs(1) {
                    break;
                }
            }
            let (t, solution_with_io) = solution_times.into_iter().min().unwrap();
            running_sum_compute += t;
            running_sum_io += solution_with_io;
            println!(
                "Day {:02} took {:>9} to compute (rsum {:>9}) (with i/o: {:>9}, rsum {:>9})",
                i + 1,
                d2s(t),
                d2s(running_sum_compute),
                d2s(solution_with_io),
                d2s(running_sum_io)
            );
            times.push(t);
            times_io.push(solution_with_io);
        }
    }

    times.sort();
    times_io.sort();

    let total = times.iter().sum();
    let min = times.iter().min();
    let max = times.iter().max();

    let total_io = times_io.iter().sum();
    let min_io = times_io.iter().min();
    let max_io = times_io.iter().max();
    if opt.day_to_run.is_none() {
        println!(
            "\n         Total time for {} days: {:>9} (avg per day {:>9}, med: {:>9}, min: {:>9}, max: {:>9})",
            solutions.len(),
            d2s(total),
            d2s(total.div_f64(solutions.len() as f64)),
            d2s(median(&times)),
            d2s(*min.unwrap()),
            d2s(*max.unwrap()),
        );
        println!(
            "Total time with i/o for {} days: {:>9} (avg per day {:>9}, med: {:>9}, min: {:>9}, max: {:>9})",
            solutions.len(),
            d2s(total_io),
            d2s(total_io.div_f64(solutions.len() as f64)),
            d2s(median(&times_io)),
            d2s(*min_io.unwrap()),
            d2s(*max_io.unwrap()),
        );
    }
}

fn median(array: &[Duration]) -> Duration {
    if (array.len() % 2) == 0 {
        let ind_left = array.len() / 2 - 1;
        let ind_right = array.len() / 2;
        (array[ind_left] + array[ind_right]).div_f64(2.0)
    } else {
        array[array.len() / 2]
    }
}

fn d2s(d: Duration) -> String {
    format!("{:.1?}", d)
}
