use anyhow::Result;
use std::time::{Duration, Instant};

use crate::input::tokens;

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<String> = tokens(input, None);

    let s = Instant::now();

    // TODO

    let e = s.elapsed();

    if verify_expected {
        // assert_eq!(0, part1);
        // assert_eq!(0, part2);
    }
    if output {
        // println!("\t{}", part1);
        // println!("\t{}", part2);
    }
    Ok(e)
}
