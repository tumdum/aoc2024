use anyhow::Result;
use rayon::prelude::*;
use std::time::{Duration, Instant};

use crate::input::token_groups;

fn apply(op: char, a: i64, b: i64) -> i64 {
    match op {
        '+' => a + b,
        '*' => a * b,
        '|' => (a * 10u64.pow(b.ilog10() + 1) as i64) + b,
        _ => unreachable!(),
    }
}

fn is_possible(t: i64, nums: &mut [i64], ops: &[char]) -> bool {
    if nums.len() <= 1 {
        return nums[0] == t;
    }
    if nums[0] > t {
        return false;
    }

    for op in ops {
        let old = nums[1];
        nums[1] = apply(*op, nums[0], nums[1]);
        if is_possible(t, &mut nums[1..], ops) {
            return true;
        }
        nums[1] = old;
    }
    false
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<String>> = token_groups(input, "\n", None);
    let equations: Vec<(i64, Vec<i64>)> = lines
        .iter()
        .map(|l| {
            let target: i64 = l[0].replace(':', "").parse().unwrap();
            let nums: Vec<i64> = l[1..].iter().map(|s| s.parse().unwrap()).collect();
            (target, nums)
        })
        .collect();
    let s = Instant::now();
    let ops = ['+', '*'];
    let part1: i64 = equations
        .par_iter()
        .filter(|(t, nums)| is_possible(*t, &mut nums.to_vec(), &ops))
        .map(|(t, _)| *t)
        .sum();
    let ops = ['+', '*', '|'];
    let part2: i64 = equations
        .into_par_iter()
        .filter_map(|(t, mut nums)| {
            if is_possible(t, &mut nums, &ops) {
                Some(t)
            } else {
                None
            }
        })
        .sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(1582598718861, part1);
        assert_eq!(165278151522644, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
