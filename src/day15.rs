use anyhow::Result;
use itertools::iproduct;
use rustc_hash::{FxHashMap, FxHashSet};
use std::time::{Duration, Instant};

use crate::{input::token_groups, vec::StrVec};

type Pos = crate::pos::Pos<i8>;

const UP: Pos = Pos::new(0, -1);
const DOWN: Pos = Pos::new(0, 1);
const LEFT: Pos = Pos::new(-1, 0);
const RIGHT: Pos = Pos::new(1, 0);

fn parse_move(c: char) -> Pos {
    match c {
        '^' => UP,
        'v' => DOWN,
        '<' => LEFT,
        '>' => RIGHT,
        c => unreachable!("c: {c}"),
    }
}

fn step(
    current: Pos,
    dir: Pos,
    map: &mut [StrVec],
    to_shift: impl Fn(Pos, Pos, &[StrVec]) -> Option<FxHashMap<Pos, u8>>,
) -> Pos {
    let next = current + dir;
    if next.get(map) == Some(b'.') {
        return next;
    }

    if let Some(boxes) = to_shift(current, dir, map) {
        shift(boxes, dir, map);
        next
    } else {
        current
    }
}

fn next_free(start: Pos, dir: Pos, map: &[StrVec]) -> Option<FxHashMap<Pos, u8>> {
    let mut ret: FxHashMap<Pos, u8> = Default::default();
    let mut current = start + dir;
    while let Some(v) = current.get(map) {
        if v == b'#' {
            return None;
        }
        if v == b'.' {
            return Some(ret);
        }
        ret.insert(current, v);
        current += dir;
    }
    None
}

fn next_free_horizontal(start: Pos, dir: Pos, map: &[StrVec]) -> Option<FxHashMap<Pos, u8>> {
    fn find_free_length(start: Pos, dir: Pos, map: &[StrVec]) -> Option<Vec<(Pos, u8)>> {
        let mut current = start + dir;
        let mut ret = vec![];
        while let Some(v) = current.get(map) {
            if v == b'#' {
                return None;
            }
            if v == b'.' {
                return Some(ret);
            }
            ret.push((current, v));
            current += dir;
        }
        None
    }

    find_free_length(start, dir, map).map(|v| v.into_iter().collect())
}

fn get_other(p: Pos, map: &[StrVec], value: Option<u8>) -> Pos {
    match value.or_else(|| p.get(map)) {
        Some(b'[') => p + RIGHT,
        Some(b']') => p + LEFT,
        _ => unimplemented!(),
    }
}

fn flood_fill_vertical(start: Pos, map: &[StrVec], dir: Pos) -> Option<Vec<Pos>> {
    let mut ret = Vec::default();

    let mut todo = FxHashSet::default();
    todo.insert(start);
    todo.insert(get_other(start, map, None));

    while let Some(p) = todo.iter().copied().next() {
        todo.remove(&p);
        ret.push(p);
        match p.get(map).unwrap() {
            b'[' | b']' => {
                let next = p + dir;
                match next.get(map) {
                    Some(b'#') => return None,
                    c @ (Some(b'[') | Some(b']')) => {
                        todo.insert(next);
                        todo.insert(get_other(next, map, c));
                    }
                    Some(b'.') => {}
                    _ => unimplemented!(),
                }
            }
            _ => unreachable!(),
        }
    }
    Some(ret)
}
fn next_free_vertical(start: Pos, dir: Pos, map: &[StrVec]) -> Option<FxHashMap<Pos, u8>> {
    match (start + dir).get(map) {
        Some(b'[') | Some(b']') => flood_fill_vertical(start + dir, map, dir).map(|positions| {
            positions
                .into_iter()
                .map(|p| (p, p.get(map).unwrap()))
                .collect()
        }),
        Some(b'#') => None,
        _ => unimplemented!(),
    }
}

fn boxes_to_shift(start: Pos, dir: Pos, map: &[StrVec]) -> Option<FxHashMap<Pos, u8>> {
    if dir.y == 0 {
        return next_free_horizontal(start, dir, map);
    }
    next_free_vertical(start, dir, map)
}

fn shift(boxes: FxHashMap<Pos, u8>, dir: Pos, map: &mut [StrVec]) {
    boxes.keys().for_each(|p| *p.get_mut(map).unwrap() = b'.');
    boxes
        .into_iter()
        .for_each(|(p, v)| *(p + dir).get_mut(map).unwrap() = v);
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<StrVec>> = token_groups(input, "\n\n", Some("\n"));

    let s = Instant::now();
    let starting_map = lines[0].clone();
    let mut map = lines[0].clone();
    let moves = lines[1].clone();
    let moves: Vec<Pos> = moves
        .into_iter()
        .flat_map(|l| (*l).to_vec())
        .map(|b| parse_move(b as char))
        .collect();

    let part1 = {
        let (w, h) = (map[0].len(), map.len());
        let current: Pos = iproduct!(0..w, 0..h)
            .find(|(x, y)| map[*y][*x] == b'@')
            .map(|(x, y)| Pos::new(x as i8, y as i8))
            .unwrap();

        *current.get_mut(&mut map).unwrap() = b'.';

        moves
            .iter()
            .fold(current, |s, m| step(s, *m, &mut map, next_free));
        score(&map, b'O')
    };

    let part2 = {
        let mut map = to_part2(&starting_map);
        let (w, h) = (map[0].len(), map.len());

        let current: Pos = iproduct!(0..w, 0..h)
            .find(|(x, y)| map[*y][*x] == b'@')
            .map(|(x, y)| Pos::new(x as i8, y as i8))
            .unwrap();

        *current.get_mut(&mut map).unwrap() = b'.';

        moves
            .iter()
            .fold(current, |s, m| step(s, *m, &mut map, boxes_to_shift));

        score(&map, b'[')
    };

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(1495147, part1);
        assert_eq!(1524905, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn score(map: &[StrVec], c: u8) -> usize {
    let (w, h) = (map[0].len(), map.len());
    iproduct!(0..w, 0..h)
        .flat_map(|(x, y)| {
            let p = Pos::new(x as i8, y as i8);
            if let Some(v) = p.get(&map) {
                if v == c {
                    Some(p.y as usize * 100 + p.x as usize)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum()
}

fn to_part2(map: &[StrVec]) -> Vec<StrVec> {
    let mut ret = vec![];
    for row in map {
        ret.push(StrVec::new(vec![]));
        for v in row {
            match *v {
                b'#' => {
                    ret.last_mut().unwrap().push(b'#');
                    ret.last_mut().unwrap().push(b'#');
                }
                b'O' => {
                    ret.last_mut().unwrap().push(b'[');
                    ret.last_mut().unwrap().push(b']');
                }
                b'.' => {
                    ret.last_mut().unwrap().push(b'.');
                    ret.last_mut().unwrap().push(b'.');
                }
                b'@' => {
                    ret.last_mut().unwrap().push(b'@');
                    ret.last_mut().unwrap().push(b'.');
                }
                b => unreachable!("byte: {b}"),
            }
        }
    }
    ret
}
