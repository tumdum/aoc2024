use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
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

fn run(
    mut pos: Pos,
    mut dir: Pos,
    map: &[StrVec],
    additional: Option<Pos>,
) -> Option<Vec<(Pos, Pos)>> {
    let mut positions: FxHashMap<Pos, Pos> = Default::default();
    let mut path: Vec<(Pos, Pos)> = Default::default();

    loop {
        if let Some(old) = positions.insert(pos, dir) {
            if old == dir {
                return None;
            }
        }
        path.push((pos, dir));

        let mut next = pos + dir;
        if let Some(next_v) = next.get(map) {
            if next_v == b'.' && Some(next) != additional {
                pos = next;
            } else {
                dir = rotate(dir);
                next = pos + dir;
                if let Some(next_v) = next.get(map) {
                    if next_v == b'#' || Some(next) == additional {
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

    Some(path)
}

fn part2(path: &[(Pos, Pos)], map: &[StrVec]) -> usize {
    let mut counts: FxHashMap<Pos, Vec<usize>> = Default::default();
    for (id, (pos, _)) in path.iter().enumerate() {
        counts.entry(*pos).or_default().push(id);
    }
    let crossings: FxHashMap<Pos, usize> = counts
        .into_iter()
        .filter(|(_, c)| c.len() > 1)
        .map(|(p, c)| (p, c[0]))
        .collect();

    path.par_iter()
        .enumerate()
        .filter(|(id, (pos, _dir))| {
            if *id == 0 {
                return false;
            }
            if let Some(first_id) = crossings.get(pos) {
                if *first_id != *id {
                    // No way to get to this point. Putting an obstacle here, would prevent us from
                    // reaching this place in the step 'id' - we would have turned at the first time
                    // we tried to reach this place.
                    return false;
                }
            }

            // let mut map_cand = map.to_vec();
            // map_cand[pos.y as usize][pos.x as usize] = b'#';
            // No need to re-run the simulation up to this point again and again, we know that will be
            // the first id-1 steps and so what will be the state just before hitting the obstacle.
            let (start, dir) = path[id - 1];
            run(start, dir, map, Some(*pos)).is_none()
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

    let path = run(pos, dir, &map, None).unwrap();
    let part1 = path
        .iter()
        .map(|(p, _)| *p)
        .collect::<FxHashSet<Pos>>()
        .len();
    let part2 = part2(&path, &map);

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
