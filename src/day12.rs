use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::VecDeque,
    iter::successors,
    time::{Duration, Instant},
};

use crate::{input::tokens, vec::StrVec};

type Pos = crate::pos::Pos<i64>;

const DELTAS: [Pos; 4] = [
    Pos::new(0, 1),
    Pos::new(0, -1),
    Pos::new(1, 0),
    Pos::new(-1, 0),
];

fn border(region: &FxHashSet<Pos>, map: &[StrVec]) -> Vec<Pos> {
    let id = region.iter().next().unwrap().get(map).unwrap();
    let mut size = vec![];
    for p in region {
        for d in DELTAS {
            let b = *p + d;
            if let Some(new_id) = b.get(map) {
                if new_id != id {
                    size.push(b);
                }
            } else {
                size.push(b);
            }
        }
    }
    size
}

fn neighbours(p: Pos, map: &[StrVec]) -> impl Iterator<Item = Pos> + '_ {
    DELTAS
        .into_iter()
        .map(move |d| p + d)
        .filter(|c| c.get(map).is_some())
}

fn find_region(start: Pos, map: &[StrVec]) -> FxHashSet<Pos> {
    let id = start.get(map).unwrap();
    let mut ret = FxHashSet::default();
    let mut seen = FxHashSet::default();
    let mut todo = VecDeque::default();
    todo.push_back(start);
    seen.insert(start);

    while let Some(curr) = todo.pop_front() {
        ret.insert(curr);
        for p in neighbours(curr, map) {
            if !seen.insert(p) {
                continue;
            }
            if p.get(map) == Some(id) {
                todo.push_back(p);
            }
        }
    }

    ret
}

fn find_all_regions(map: &[StrVec]) -> Vec<FxHashSet<Pos>> {
    let (w, h) = (map[0].len(), map.len());
    let mut regions: Vec<FxHashSet<Pos>> = Default::default();
    let mut seen = vec![vec![false; w]; h];
    for (x, y) in iproduct!(0..w, 0..h) {
        let p = Pos::new(x as i64, y as i64);
        if seen[p.y as usize][p.x as usize] {
            continue;
        }

        let region = find_region(p, map);
        for v in &region {
            seen[v.y as usize][v.x as usize] = true;
        }
        regions.push(region);
    }

    regions
}

fn all_sides(region: &FxHashSet<Pos>, border: &[Pos]) -> Vec<FxHashMap<Pos, Pos>> {
    let border: FxHashSet<Pos> = border.iter().copied().collect();
    let mut all_sides: Vec<FxHashMap<Pos, Pos>> = vec![];
    for p in region {
        for b in border_counterparts(*p, &border) {
            if all_sides.iter().any(|side| side.get(p) == Some(&b)) {
                continue;
            }
            all_sides.push(find_side(region, &border, (*p, b)));
        }
    }
    all_sides
}

fn border_counterparts(p: Pos, border: &FxHashSet<Pos>) -> impl Iterator<Item = Pos> + '_ {
    DELTAS
        .into_iter()
        .map(move |d| p + d)
        .filter(|c| border.contains(c))
}

fn find_side(
    region: &FxHashSet<Pos>,
    border: &FxHashSet<Pos>,
    start: (Pos, Pos),
) -> FxHashMap<Pos, Pos> {
    fn build_side(
        region: &FxHashSet<Pos>,
        border: &FxHashSet<Pos>,
        start: (Pos, Pos),
        d: Pos,
    ) -> FxHashMap<Pos, Pos> {
        successors(Some(start), |(p, b)| Some((*p + d, *b + d)))
            .take_while(|(p, b)| region.contains(p) && border.contains(b))
            .collect()
    }

    let delta = start.1 - start.0;
    let mut side = FxHashMap::default();
    if delta.x == 0 {
        side.extend(build_side(region, border, start, Pos::new(1, 0)));
        side.extend(build_side(region, border, start, Pos::new(-1, 0)));
    } else {
        side.extend(build_side(region, border, start, Pos::new(0, 1)));
        side.extend(build_side(region, border, start, Pos::new(0, -1)));
    }
    side
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<StrVec> = tokens(input, None);

    let s = Instant::now();

    let regions: Vec<(FxHashSet<Pos>, Vec<Pos>)> = find_all_regions(&lines)
        .into_par_iter()
        .map(|r| {
            let b = border(&r, &lines);
            (r, b)
        })
        .collect();

    let part1: usize = regions.iter().map(|(r, b)| r.len() * b.len()).sum();

    let part2: usize = regions
        .into_par_iter()
        .map(|(r, b)| r.len() * all_sides(&r, &b).len())
        .sum();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(1377008, part1);
        assert_eq!(815788, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
