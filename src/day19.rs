use anyhow::Result;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::input::tokens;

fn is_possible(design: &str, patterns: &[&str], cache: &mut FxHashMap<String, u64>) -> u64 {
    if let Some(possible_ways) = cache.get(design) {
        return *possible_ways;
    }

    let mut ways = 0;
    for pattern in patterns {
        if let Some(suffix) = design.strip_prefix(pattern) {
            let result = is_possible(suffix, patterns, cache);
            cache.insert(suffix.to_owned(), result);
            ways += result;
        }
    }
    ways
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<String> = tokens(input, Some("\n"));

    let patterns = lines[0].split(", ").collect_vec();
    let designs = lines[1..].to_vec();
    let s = Instant::now();

    let mut part1 = 0;
    let mut part2 = 0u64;
    let mut cache: FxHashMap<String, u64> = Default::default();
    cache.insert("".to_owned(), 1);
    for design in designs {
        let possible = is_possible(&design, &patterns, &mut cache);
        part2 += possible;
        if possible > 0 {
            part1 += 1;
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(272, part1);
        assert_eq!(1041529704688380, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
