use crate::dijkstra::{dijkstra, path};
use crate::{input::tokens, vec::StrVec};
use anyhow::Result;
use itertools::iproduct;
use rayon::prelude::*;
use smallvec::smallvec;
use std::time::{Duration, Instant};
type Pos = crate::pos::Pos<i32>;
type Small<T> = smallvec::SmallVec<[T; 3]>;

const UP: Pos = Pos::new(0, -1);
const DOWN: Pos = Pos::new(0, 1);
const LEFT: Pos = Pos::new(-1, 0);
const RIGHT: Pos = Pos::new(1, 0);
const DIRS: [Pos; 4] = [UP, DOWN, LEFT, RIGHT];

fn find_best_cheats(max_len: i32, min_saving: i32, best_path: &[Pos], map: &[StrVec]) -> u64 {
    let (w, h) = (map[0].len(), map.len());

    let l = best_path.len() as i32;
    let mut length_after = vec![vec![i32::MAX; w]; h];
    let mut length_before = vec![vec![i32::MAX; w]; h];
    for (id, p) in best_path.iter().enumerate() {
        length_after[p.y as usize][p.x as usize] = l - id as i32;
        length_before[p.y as usize][p.x as usize] = id as i32;
    }

    let normal_score = best_path.len() as i32 - 1;

    best_path
        .into_par_iter()
        .map(|pos| {
            find_cheats(
                *pos,
                max_len,
                &map,
                min_saving,
                normal_score,
                length_before[pos.y as usize][pos.x as usize],
                &length_after,
            )
        })
        .sum()
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let mut map: Vec<StrVec> = tokens(input, None);

    let s = Instant::now();

    let (w, h) = (map[0].len(), map.len());
    let start: Pos = iproduct!(0..w, 0..h)
        .find(|(x, y)| map[*y][*x] == b'S')
        .map(|(x, y)| Pos::new(x as i32, y as i32))
        .unwrap();
    let end: Pos = iproduct!(0..w, 0..h)
        .find(|(x, y)| map[*y][*x] == b'E')
        .map(|(x, y)| Pos::new(x as i32, y as i32))
        .unwrap();
    *start.get_mut(&mut map).unwrap() = b'.';
    *end.get_mut(&mut map).unwrap() = b'.';
    let neighbours = |curr: &Pos| -> Small<(Pos, usize)> {
        let mut ret = smallvec![];
        for d in DIRS {
            let next = *curr + d;
            if next.get(&map) == Some(b'.') {
                ret.push((next, 1));
            }
        }
        ret
    };
    let (_costs, prev) = dijkstra(&[start], neighbours);

    let best_path = path(&start, &end, &prev).unwrap();

    let part1 = find_best_cheats(2, 100, &best_path, &map);
    let part2 = find_best_cheats(20, 100, &best_path, &map);

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(1311, part1);
        assert_eq!(961364, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn find_cheats(
    start: Pos,
    max_steps: i32,
    map: &[StrVec],
    min_saving: i32,
    normal_score: i32,
    before_cost: i32,
    length_after: &[Vec<i32>],
) -> u64 {
    let startx = start.x.saturating_sub(max_steps);
    let starty = start.y.saturating_sub(max_steps);
    let mut total = 0;
    for y in starty..(starty + 2 * max_steps + 1) {
        for x in startx..(startx + 2 * max_steps + 1) {
            let p = Pos::new(x, y);

            if p.get(map) == Some(b'.') {
                let dx = (start.x - p.x).abs();
                let dy = (start.y - p.y).abs();
                let d = dx + dy - 1;
                if d < max_steps && d >= 0 {
                    let saving = normal_score
                        - (before_cost + length_after[p.y as usize][p.x as usize] - 1 + d);
                    if saving >= min_saving {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}
