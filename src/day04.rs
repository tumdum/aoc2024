use anyhow::Result;
use itertools::iproduct;
use std::time::{Duration, Instant};

use crate::vec::get_at;

fn count_many_from(
    x: i64,
    y: i64,
    input: &[Vec<char>],
    needles: &[&[u8]],
    deltas: &[(i64, i64)],
) -> usize {
    needles
        .iter()
        .map(|needle| count_from(x, y, input, needle, deltas))
        .sum()
}

fn count_from(x: i64, y: i64, input: &[Vec<char>], needle: &[u8], deltas: &[(i64, i64)]) -> usize {
    let mut count = 0;
    'out: for d in deltas {
        for (i, expected) in needle.iter().enumerate() {
            match get_at(x, y, i as i64 * d.0, i as i64 * d.1, input) {
                Some(v) if *v as u8 != *expected => continue 'out,
                Some(_) => {}
                None => continue 'out,
            }
        }

        count += 1;
    }
    count
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let (w, h) = (lines[0].len(), lines.len());

    let s = Instant::now();

    let all_dirs: [(i64, i64); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let part1: usize = iproduct!(0..w, 0..h)
        .map(|(x, y)| count_from(x as i64, y as i64, &lines, b"XMAS", &all_dirs))
        .sum();

    let pred = |x: usize, y: usize, dir: &[(i64, i64)]| {
        count_many_from(x as i64, y as i64, &lines, &[b"SAM", b"MAS"], dir) != 0
    };
    let part2: usize = iproduct!(0..w, 0..h)
        .filter(|(x, y)| pred(*x, *y, &[(1, 1)]) && pred(x + 2, *y, &[(-1, 1)]))
        .count();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(2401, part1);
        assert_eq!(1822, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
