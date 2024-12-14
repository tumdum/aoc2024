use anyhow::Result;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::{
    ops::Range,
    time::{Duration, Instant},
};

use crate::input::ints;

type Pos = crate::pos::Pos<i16>;
type Space = (i16, i16);

fn step((w, h): Space, robots: &[(Pos, Pos)]) -> Vec<(Pos, Pos)> {
    let step = |(p, v): &(Pos, Pos)| -> (Pos, Pos) {
        let mut np = *p + *v;
        np.x = np.x.rem_euclid(w);
        np.y = np.y.rem_euclid(h);
        (np, *v)
    };
    robots.iter().map(step).collect()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let mut robots = ints(input)
        .chunks_exact(4)
        .map(|v| (Pos::new(v[0], v[1]), Pos::new(v[2], v[3])))
        .collect_vec();

    let s = Instant::now();

    let space = (101, 103);

    let mut part1 = 0;
    let mut part2 = 0;
    for s in 1.. {
        robots = step(space, &robots);
        if s == 100 {
            part1 = count(space.0, space.1, &robots);
        }
        if is_tree(&robots) {
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

fn is_tree(robots: &[(Pos, Pos)]) -> bool {
    let pos: FxHashSet<Pos> = robots.iter().map(|(p, _)| *p).collect();

    const D: Pos = Pos::new(1, 0);

    pos.iter()
        .any(|p| (0..31).all(|d| pos.contains(&(*p + D * d))))
}

fn count(w: i16, h: i16, robots: &[(Pos, Pos)]) -> usize {
    assert!(w % 2 == 1 && h % 2 == 1);
    let up_left = (0..w / 2, 0..h / 2);
    let up_right = (w / 2 + 1..w, 0..h / 2);
    let down_left = (0..w / 2, h / 2 + 1..h);
    let down_right = (w / 2 + 1..w, h / 2 + 1..h);
    fn aux((xs, ys): (Range<i16>, Range<i16>), robots: &[(Pos, Pos)]) -> usize {
        robots
            .iter()
            .map(|(p, _)| *p)
            .filter(|p| xs.contains(&p.x) && ys.contains(&p.y))
            .count()
    }

    aux(up_left, robots) * aux(up_right, robots) * aux(down_left, robots) * aux(down_right, robots)
}
