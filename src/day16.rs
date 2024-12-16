use anyhow::Result;
use itertools::iproduct;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::smallvec;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Add;
use std::time::{Duration, Instant};

use crate::{input::tokens, vec::StrVec};

type Pos = crate::pos::Pos<i32>;
type Small<T> = smallvec::SmallVec<[T; 3]>;

const UP: Pos = Pos::new(0, -1);
const DOWN: Pos = Pos::new(0, 1);
const LEFT: Pos = Pos::new(-1, 0);
const RIGHT: Pos = Pos::new(1, 0);

fn rotations(dir: Pos) -> [Pos; 2] {
    if dir == UP || dir == DOWN {
        return [LEFT, RIGHT];
    }

    if dir == LEFT || dir == RIGHT {
        return [UP, DOWN];
    }

    unreachable!()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pos: Pos,
    dir: Pos,
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let mut map: Vec<StrVec> = tokens(input, None);

    let s = Instant::now();

    let (w, h) = (map[0].len(), map.len());
    let start = iproduct!(0..w, 0..h)
        .find(|(x, y)| map[*y][*x] == b'S')
        .map(|(x, y)| Pos::new(x as i32, y as i32))
        .unwrap();
    let end = iproduct!(0..w, 0..h)
        .find(|(x, y)| map[*y][*x] == b'E')
        .map(|(x, y)| Pos::new(x as i32, y as i32))
        .unwrap();

    *start.get_mut(&mut map).unwrap() = b'.';
    *end.get_mut(&mut map).unwrap() = b'.';

    let start = State {
        pos: start,
        dir: RIGHT,
    };

    let neighbours = |state: &State| -> Small<(State, usize)> {
        let mut ret = smallvec![];
        for rot in rotations(state.dir) {
            ret.push((
                State {
                    pos: state.pos,
                    dir: rot,
                },
                1000,
            ));
        }
        let next = state.pos + state.dir;
        if let Some(v) = next.get(&map) {
            if v == b'.' {
                ret.push((
                    State {
                        pos: next,
                        dir: state.dir,
                    },
                    1,
                ));
            }
        }
        ret
    };

    let (costs, prev) = dijkstra(&[start], neighbours);

    let (part1, min_end) = [UP, DOWN, LEFT, RIGHT]
        .into_iter()
        .map(|dir| State { pos: end, dir })
        .flat_map(|s| costs.get(&s).map(|c| (*c, s)))
        .min_by_key(|(c, _)| *c)
        .unwrap();

    let points_on_best_paths: FxHashSet<Pos> = path(&start, &min_end, &prev)
        .unwrap()
        .into_iter()
        .map(|s| s.pos)
        .collect();

    let part2 = points_on_best_paths.len();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(143564, part1);
        assert_eq!(593, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
pub fn dijkstra<T, P, V>(
    start: &[T],
    neighbours_of: impl Fn(&T) -> V,
) -> (FxHashMap<T, P>, FxHashMap<T, FxHashSet<T>>)
where
    T: Debug + PartialEq + Eq + PartialOrd + Ord + Hash + Clone,
    P: Debug + PartialEq + Eq + PartialOrd + Ord + Default + Clone + Add<Output = P>,
    V: IntoIterator<Item = (T, P)>,
{
    let mut dist: FxHashMap<T, P> = Default::default();
    for s in start {
        dist.insert(s.clone(), P::default());
    }

    let mut prev: FxHashMap<T, FxHashSet<T>> = Default::default();

    #[derive(Debug, PartialEq, Eq)]
    struct State<U: Debug + PartialEq + Eq + PartialOrd + Ord, V: Debug + PartialOrd + Ord> {
        key: U,
        prio: V,
    }
    impl<U: Debug + PartialOrd + Ord, V: Debug + PartialOrd + Ord> Ord for State<U, V> {
        fn cmp(&self, other: &Self) -> Ordering {
            let o = self.prio.cmp(&other.prio);
            if o == Ordering::Equal {
                self.key.cmp(&other.key)
            } else {
                o
            }
        }
    }
    impl<U: Debug + PartialOrd + Ord, V: Debug + PartialOrd + Ord> PartialOrd for State<U, V> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let mut todo: BinaryHeap<Reverse<State<T, P>>> = BinaryHeap::default();
    for s in start {
        todo.push(Reverse(State {
            key: s.clone(),
            prio: dist.get(s).unwrap().clone(),
        }));
    }

    while let Some(Reverse(State { key, prio })) = todo.pop() {
        for (neighbour, cost) in neighbours_of(&key) {
            let alt = prio.clone() + cost;
            match dist.get(&neighbour) {
                Some(old_cost) => {
                    if old_cost > &alt {
                        dist.insert(neighbour.clone(), alt.clone());
                        let mut set = FxHashSet::default();
                        set.insert(key.clone());
                        prev.insert(neighbour.clone(), set);
                        todo.push(Reverse(State {
                            key: neighbour,
                            prio: alt,
                        }));
                    } else if old_cost == &alt {
                        dist.insert(neighbour.clone(), alt.clone());
                        prev.get_mut(&neighbour).unwrap().insert(key.clone());
                        todo.push(Reverse(State {
                            key: neighbour,
                            prio: alt,
                        }));
                    }
                }
                None => {
                    dist.insert(neighbour.clone(), alt.clone());
                    let mut set = FxHashSet::default();
                    set.insert(key.clone());
                    prev.insert(neighbour.clone(), set);
                    todo.push(Reverse(State {
                        key: neighbour,
                        prio: alt,
                    }));
                }
            }
        }
    }
    (dist, prev)
}

pub fn path<T: PartialEq + Eq + Hash + Clone>(
    from: &T,
    to: &T,
    prev: &FxHashMap<T, FxHashSet<T>>,
) -> Option<FxHashSet<T>> {
    let mut set = FxHashSet::default();
    set.insert(to.clone());
    let mut path: Vec<FxHashSet<T>> = vec![set];

    loop {
        let mut next: FxHashSet<T> = Default::default();
        let mut ended = 0;
        for p in path.last().unwrap() {
            if p == from {
                ended += 1;
                continue;
            }
            if let Some(prev_points) = prev.get(&p) {
                for v in prev_points {
                    next.insert(v.clone());
                }
            }
        }
        path.push(next);
        if ended == path.last().unwrap().len() {
            break;
        }
    }
    Some(path.into_iter().flatten().collect())
}
