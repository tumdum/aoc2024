use anyhow::Result;
use itertools::iproduct;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    iter::successors,
    time::{Duration, Instant},
};

use crate::{input::tokens, vec::StrVec};

type Pos = crate::pos::Pos<i64>;

fn is_on_map(map: &[StrVec], p: Pos) -> bool {
    p.get(map).is_some()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let map: Vec<StrVec> = tokens(input, None);

    let s = Instant::now();

    let mut antenas: FxHashMap<u8, Vec<Pos>> = Default::default();
    for (x, y) in iproduct!(0..map[0].len(), 0..map.len()) {
        let id = map[y][x];
        if id != b'.' {
            antenas
                .entry(id)
                .or_default()
                .push(Pos::new(x as i64, y as i64));
        }
    }

    let e = s.elapsed();

    let mut antinodes: FxHashSet<Pos> = Default::default();
    let mut antinodes2: FxHashSet<Pos> = Default::default();

    for positions in antenas.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let i = positions[i];
                let j = positions[j];
                let d = j - i;
                let node1 = i - d;
                let node2 = j + d;

                if is_on_map(&map, node1) {
                    antinodes.insert(node1);
                }

                if is_on_map(&map, node2) {
                    antinodes.insert(node2);
                }

                antinodes2.extend(
                    successors(Some(i), |p| Some(*p - d)).take_while(|p| is_on_map(&map, *p)),
                );
                antinodes2.extend(
                    successors(Some(j), |p| Some(*p + d)).take_while(|p| is_on_map(&map, *p)),
                );
            }
        }
    }

    let part1 = antinodes.len();
    let part2 = antinodes2.len();

    if verify_expected {
        assert_eq!(336, part1);
        assert_eq!(1131, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
