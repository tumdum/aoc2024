use anyhow::Result;
use regex::Regex;
use std::time::{Duration, Instant};

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let s = Instant::now();
    let re = Regex::new(r#"(?<mul>mul\((\d{1,3})),(\d{1,3})\)|(?<dont>don't\(\))|(?<do>do\(\))"#)
        .unwrap();

    let (mut part1, mut part2, mut dont) = (0, 0, false);

    for c in re.captures_iter(input) {
        if c.name("mul").is_some() {
            let a: i64 = c.get(2).unwrap().as_str().parse().unwrap();
            let b: i64 = c.get(3).unwrap().as_str().parse().unwrap();
            part1 += a * b;
            if !dont {
                part2 += a * b;
            }
        } else if c.name("dont").is_some() {
            dont = true;
        } else if c.name("do").is_some() {
            dont = false;
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(179571322, part1);
        assert_eq!(103811193, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
