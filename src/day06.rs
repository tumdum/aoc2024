use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::time::{Duration, Instant};

use crate::{input::tokens, vec::StrVec};
type Pos = crate::pos::Pos<i16>;

fn is_dir(c: u8) -> Option<Pos> {
    match c {
        b'^' => Some(Pos::new(0, -1)),
        b'v' => Some(Pos::new(0, 1)),
        b'>' => Some(Pos::new(1, 0)),
        b'<' => Some(Pos::new(-1, 0)),
        _ => None,
    }
}

fn rotate(p: Pos) -> Pos {
    match p {
        Pos { x: 1, y: 0 } => Pos::new(0, 1),
        Pos { x: 0, y: 1 } => Pos::new(-1, 0),
        Pos { x: -1, y: 0 } => Pos::new(0, -1),
        Pos { x: 0, y: -1 } => Pos::new(1, 0),
        _ => unreachable!(),
    }
}

fn run(mut pos: Pos, mut dir: Pos, map: &[StrVec]) -> Option<FxHashMap<Pos, Pos>> {
    let mut positions: FxHashMap<Pos, Pos> = Default::default();

    loop {
        if let Some(old) = positions.insert(pos, dir) {
            if old == dir {
                return None;
            }
        }

        let mut next = pos + dir;
        if let Some(next_v) = next.get(&map) {
            if next_v == b'.' {
                pos = next;
            } else {
                dir = rotate(dir);
                next = pos + dir;
                if let Some(next_v) = next.get(&map) {
                    if next_v == b'#' {
                        dir = rotate(dir);
                        next = pos + dir;
                    }
                }
                pos = next;
            }
        } else {
            break;
        }
    }

    Some(positions)
}

fn part2(start: Pos, dir: Pos, path: &FxHashMap<Pos, Pos>, map: &[StrVec]) -> usize {
    path.keys()
        .par_bridge()
        .filter(|pos| {
            if **pos == start {
                return false;
            }

            let mut map_cand = map.to_vec();
            map_cand[pos.y as usize][pos.x as usize] = b'#';
            run(start, dir, &map_cand).is_none()
        })
        .count()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let mut map: Vec<StrVec> = tokens(input, Some("\n"));

    let s = Instant::now();

    let pos: Pos = iproduct!(0..map[0].len(), 0..map.len())
        .find(|(x, y)| is_dir(map[*y][*x]).is_some())
        .map(|(x, y)| Pos::new(x.try_into().unwrap(), y.try_into().unwrap()))
        .unwrap();
    let dir = is_dir(pos.get(&map).unwrap()).unwrap();

    map[pos.y as usize][pos.x as usize] = b'.';

    let path = run(pos, dir, &map).unwrap();
    let part1 = path.len();
    let part2 = part2(pos, dir, &path, &map);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(4454, part1);
        assert_eq!(1503, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
