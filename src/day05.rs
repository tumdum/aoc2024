use anyhow::Result;
use rayon::prelude::*;
use std::{
    cmp::Ordering,
    time::{Duration, Instant},
};

use crate::input::{token_groups, tokens};

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<String>> = token_groups(input, "\n\n", Some("\n"));

    let mut rules: Vec<(u8, u8)> = lines[0]
        .iter()
        .map(|s| {
            let mut s = s.split("|");
            let a = s.next().unwrap().parse().unwrap();
            let b = s.next().unwrap().parse().unwrap();
            (a, b)
        })
        .collect();

    let mut updates: Vec<Vec<u8>> = lines[1].iter().map(|s| tokens(s, Some(","))).collect();

    let s = Instant::now();

    rules.sort_unstable();
    let order = |a: &u8, b: &u8| -> Ordering {
        if a == b {
            Ordering::Equal
        } else {
            for (x, y) in &rules {
                if x == a && y == b {
                    return Ordering::Less;
                }
                if x == b && y == a {
                    return Ordering::Greater;
                }
            }
            a.cmp(&b)
        }
    };

    let part1: i64 = updates
        .par_iter()
        .filter(|u| u.is_sorted_by(|a, b| order(a, b).is_lt()))
        .map(|u| u[u.len() / 2] as i64)
        .sum();
    let part2: i64 = updates
        .par_iter_mut()
        .filter_map(|u| {
            if !u.is_sorted_by(|a, b| order(a, b).is_lt()) {
                u.sort_unstable_by(order);
                Some(u[u.len() / 2] as i64)
            } else {
                None
            }
        })
        .sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(5087, part1);
        assert_eq!(4971, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
