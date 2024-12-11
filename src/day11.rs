use anyhow::Result;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{
    iter::successors,
    time::{Duration, Instant},
};

use crate::{input::tokens, utils::count_elements};

fn split(v: u64) -> (u64, u64) {
    let div = 10u64.pow((v.ilog10() + 1) / 2);
    (v / div, v % div)
}

fn blink(input: &FxHashMap<u64, usize>) -> FxHashMap<u64, usize> {
    let mut ret: FxHashMap<u64, usize> =
        FxHashMap::with_capacity_and_hasher(input.len() + 100, FxBuildHasher::default());

    for (stone, count) in input {
        if *stone == 0 {
            *ret.entry(1).or_default() += count;
        } else if stone.ilog10() % 2 == 1 {
            let (a, b) = split(*stone);
            *ret.entry(a).or_default() += count;
            *ret.entry(b).or_default() += count;
        } else {
            *ret.entry(*stone * 2024).or_default() += count;
        }
    }
    ret
}

fn blinks(input: &[u64], n: usize) -> FxHashMap<u64, usize> {
    successors(Some(count_elements(input)), |nums| Some(blink(nums)))
        .nth(n)
        .unwrap()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<u64> = tokens(input, None);

    let s = Instant::now();

    let part1: usize = blinks(&lines, 25).values().copied().sum();
    let part2: usize = blinks(&lines, 75).values().copied().sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(213625, part1);
        assert_eq!(252442982856820, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
