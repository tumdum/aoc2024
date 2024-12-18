use anyhow::Result;
use rayon::prelude::*;
use std::time::{Duration, Instant};

use crate::{
    dijkstra::{bfs, path},
    input::ints,
};

const DIRS: [Pos; 4] = [
    Pos::new(1, 0),
    Pos::new(-1, 0),
    Pos::new(0, 1),
    Pos::new(0, -1),
];

type Pos = crate::pos::Pos<i8>;

fn find_path_length(start: Pos, target: Pos, corrupted: &[Pos]) -> Option<usize> {
    let mut bytes = vec![vec![false; 71]; 71];
    for b in corrupted {
        bytes[b.y as usize][b.x as usize] = true;
    }

    let neighbours = |p: &Pos| -> Vec<Pos> {
        let mut ret = vec![];
        for d in DIRS {
            let next = *p + d;
            if next.x < 0 || next.y < 0 || next.x > target.x || next.y > target.y {
                continue;
            }
            if bytes[next.y as usize][next.x as usize] {
                continue;
            }
            ret.push(next);
        }
        ret
    };

    path(&start, &target, &bfs(start, |p| *p == target, neighbours)).map(|path| path.len() - 1)
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let bytes: Vec<Pos> = ints(input)
        .chunks(2)
        .map(|v| Pos::new(v[0], v[1]))
        .collect();

    let start = Pos::new(0, 0);
    let target = Pos::new(70, 70);

    let s = Instant::now();

    let part1 = find_path_length(start, target, &bytes[..1024]).unwrap();
    let part2 = (1025..bytes.len())
        .par_bridge()
        .filter(|len| find_path_length(start, target, &bytes[..*len]).is_none())
        .min()
        .unwrap()
        - 1;

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(436, part1);
        assert_eq!(Pos::new(61, 50), bytes[part2]);
    }
    if output {
        // println!("\t{}", part1);
        // println!("\t{}", part2);
    }
    Ok(e)
}
