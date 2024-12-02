use anyhow::Result;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::{input::token_groups, vec::transpose_vec};

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<i64>> = token_groups(input, "\n", Some("   "));

    let s = Instant::now();

    let mut lists = transpose_vec(&lines);
    let mut g1 = lists.swap_remove(0);
    let mut g2 = lists.swap_remove(0);

    g1.sort_unstable();
    g2.sort_unstable();

    let part1: i64 = g1.iter().zip(&g2).map(|(a, b)| (a - b).abs()).sum();

    let mut counts: FxHashMap<i64, usize> = g1.iter().map(|i| (*i, 0)).collect();

    for v in &g2 {
        counts.entry(*v).and_modify(|v| *v += 1);
    }

    let part2: i64 = g1
        .into_iter()
        .map(|v| v * counts.get(&v).copied().unwrap_or_default() as i64)
        .sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(1223326, part1);
        assert_eq!(21070419, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
