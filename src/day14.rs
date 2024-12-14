use anyhow::Result;
use itertools::Itertools;
use std::{
    ops::Range,
    time::{Duration, Instant},
};

use crate::input::ints;

type Pos = crate::pos::Pos<i16>;

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let mut robots = ints(input)
        .chunks_exact(4)
        .map(|v| (Pos::new(v[0], v[1]), Pos::new(v[2], v[3])))
        .collect_vec();

    let s = Instant::now();

    let (w, h) = (101i16, 103i16);
    let mut buf: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];

    let mut part1 = 0;
    let mut part2 = 0;
    for s in 1.. {
        for (p, v) in &mut robots {
            p.x = (p.x + v.x).rem_euclid(w);
            p.y = (p.y + v.y).rem_euclid(h);
        }

        if s == 100 {
            part1 = count(w, h, &robots);
        }

        if is_tree(&robots, &mut buf) {
            part2 = s;
            break;
        }
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(223020000, part1);
        assert_eq!(7338, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn is_tree(robots: &[(Pos, Pos)], occupied: &mut [Vec<bool>]) -> bool {
    let w = occupied[0].len() as i16;
    for row in &mut *occupied {
        row.fill(false);
    }
    for (p, _) in robots {
        occupied[p.y as usize][p.x as usize] = true;
    }

    robots.iter().any(|(p, _)| {
        if w - p.x < 31 {
            return false;
        }
        (0..31).all(|d| occupied[p.y as usize][(p.x + d) as usize])
    })
}

fn count(w: i16, h: i16, robots: &[(Pos, Pos)]) -> usize {
    assert!(w % 2 == 1 && h % 2 == 1);
    fn aux((xs, ys): (Range<i16>, Range<i16>), robots: &[(Pos, Pos)]) -> usize {
        robots
            .iter()
            .map(|(p, _)| *p)
            .filter(|p| xs.contains(&p.x) && ys.contains(&p.y))
            .count()
    }

    aux((0..w / 2, 0..h / 2), robots)
        * aux((w / 2 + 1..w, 0..h / 2), robots)
        * aux((0..w / 2, h / 2 + 1..h), robots)
        * aux((w / 2 + 1..w, h / 2 + 1..h), robots)
}
