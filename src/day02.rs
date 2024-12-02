use anyhow::Result;
use itertools::Itertools;
use std::time::{Duration, Instant};

use crate::input::token_groups;

fn is_safe2(report: impl AsRef<[i64]>) -> bool {
    let report = report.as_ref();
    (0..report.len()).any(|i| {
        is_safe(
            report
                .iter()
                .enumerate()
                .filter(|(id, _)| *id != i)
                .map(|(_, v)| *v),
        )
    })
}

fn is_safe(report: impl Iterator<Item = i64>) -> bool {
    let (mut inc, mut dec, mut count) = (0, 0, 0);
    for d in report.tuple_windows().map(|(a, b)| (a - b)) {
        count += 1;
        if (1..=3).contains(&d) {
            inc += 1;
        } else if (-3..=-1).contains(&d) {
            dec += 1;
        }
    }

    inc == count || dec == count
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<i64>> = token_groups(input, "\n", Some(" "));

    let s = Instant::now();

    let part1 = lines
        .iter()
        .map(|l| is_safe(l.iter().copied()))
        .filter(|ret| *ret)
        .count();
    let part2 = lines.iter().map(is_safe2).filter(|ret| *ret).count();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(526, part1);
        assert_eq!(566, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
