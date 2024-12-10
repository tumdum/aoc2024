use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::{smallvec, SmallVec};
use std::fmt::Debug;
use std::hash::Hash;
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

type Pos = crate::pos::Pos<i16>;
type Small = SmallVec<[Pos; 4]>;

use crate::{input::tokens, vec::StrVec};

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<StrVec> = tokens(input, None);

    let (w, h) = (lines[0].len(), lines.len());

    let s = Instant::now();
    let trailheads: Vec<Pos> = iproduct!(0..w, 0..h)
        .filter(|(x, y)| lines[*y][*x] == b'0')
        .map(|(x, y)| Pos::new(x as i16, y as i16))
        .collect();
    let ends: FxHashSet<Pos> = iproduct!(0..w, 0..h)
        .filter(|(x, y)| lines[*y][*x] == b'9')
        .map(|(x, y)| Pos::new(x as i16, y as i16))
        .collect();

    let paths: Vec<_> = trailheads
        .par_iter()
        .map(|th| find_paths(*th, &lines, &ends))
        .collect();

    let part1: usize = paths.iter().map(|(score, _)| *score).sum();
    let part2: usize = paths.iter().map(|(_, trails)| *trails).sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(574, part1);
        assert_eq!(1238, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

pub fn find_trail_ends<T>(
    start: T,
    is_target: impl Fn(&T) -> bool,
    neighbours_of: impl Fn(&T) -> SmallVec<[T; 4]>,
) -> Vec<T>
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Clone,
{
    let mut prev: FxHashMap<T, T> = Default::default();
    let mut todo: VecDeque<T> = Default::default();
    let mut path_ends: Vec<T> = Default::default();
    todo.push_back(start.clone());

    while let Some(next) = todo.pop_front() {
        if is_target(&next) {
            assert!(next == start || prev.contains_key(&next));
            path_ends.push(next.clone());
        }
        for candidate in neighbours_of(&next) {
            if prev.contains_key(&candidate) {
                continue;
            }
            todo.push_back(candidate.clone());
            prev.insert(candidate, next.clone());
        }
    }

    path_ends
}

fn number_of_paths(to: Pos, from: Pos, map: &[StrVec]) -> usize {
    let mut ways = 0;
    let mut todo: VecDeque<Pos> = Default::default();
    todo.push_front(from);
    while let Some(next) = todo.pop_front() {
        if next == to {
            ways += 1;
            continue;
        }

        todo.extend(neighbours(next, map));
    }
    ways
}

fn neighbours(p: Pos, map: &[StrVec]) -> Small {
    const D: [Pos; 4] = [
        Pos::new(0, 1),
        Pos::new(1, 0),
        Pos::new(0, -1),
        Pos::new(-1, 0),
    ];
    let mut ret = smallvec![];
    let start_h = p.get(map).unwrap();
    for d in D {
        let c = p + d;
        if let Some(height) = c.get(map) {
            if start_h + 1 == height {
                ret.push(c);
            }
        }
    }
    ret
}

fn find_paths(from: Pos, map: &[StrVec], ends: &FxHashSet<Pos>) -> (usize, usize) {
    let ends = find_trail_ends(from, |p: &Pos| ends.contains(p), |p| neighbours(*p, map));

    let paths = ends
        .iter()
        .map(|end| number_of_paths(*end, from, map))
        .sum();

    (ends.len(), paths)
}
