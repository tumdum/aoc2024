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

    const W: i16 = 101i16;
    const H: i16 = 103i16;

    let mut buf: [u128; H as usize] = [0; H as usize];

    let mut part1 = 0;
    let mut part2 = 0;
    for s in 1.. {
        for (p, v) in &mut robots {
            p.x = (p.x + v.x).rem_euclid(W);
            p.y = (p.y + v.y).rem_euclid(H);
        }

        if s == 100 {
            part1 = count(W, H, &robots);
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

fn is_tree<const N: usize>(robots: &[(Pos, Pos)], occupied: &mut [u128; N]) -> bool {
    occupied.fill(0);

    for (p, _) in robots {
        occupied[p.y as usize] |= 1 << (p.x as u128);
    }

    robots.iter().any(|(p, _)| {
        let mask = ((1u128 << 31) - 1) << (p.x as u128);
        (occupied[p.y as usize] & mask) == mask
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
