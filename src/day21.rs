use crate::input::tokens;
use anyhow::Result;
use itertools::{iproduct, Itertools};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::hash::Hash;
use std::ops::Add;
use std::{
    fmt::{Debug, Display, Write},
    time::{Duration, Instant},
};

type Pos = crate::pos::Pos<i32>;

const UP: Pos = Pos::new(0, -1);
const DOWN: Pos = Pos::new(0, 1);
const LEFT: Pos = Pos::new(-1, 0);
const RIGHT: Pos = Pos::new(1, 0);
const DIRS: [Pos; 4] = [UP, DOWN, LEFT, RIGHT];

const NUM_PANEL: &str = r#"
789
456
123
#0A 
"#;
const DIR_PANEL: &str = r#"
#^A
<v> 
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
            Dir::Activate => 'A',
        };
        f.write_char(c)
    }
}

impl TryFrom<char> for Dir {
    type Error = ();

    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '^' => Ok(Self::Up),
            '<' => Ok(Self::Left),
            '>' => Ok(Self::Right),
            'v' => Ok(Self::Down),
            'A' => Ok(Self::Activate),
            _ => Err(()),
        }
    }
}

fn find_button<T: Clone + Debug + PartialEq>(btn: T, map: &[Vec<Option<T>>]) -> Option<Pos> {
    let (w, h) = (map[0].len(), map.len());
    for p in iproduct!(0..w, 0..h).map(|(x, y)| Pos::new(x as i32, y as i32)) {
        if map[p.y as usize][p.x as usize] == Some(btn.clone()) {
            return Some(p);
        }
    }
    None
}

fn positions_to_dirs(pos: &[Pos]) -> Vec<Dir> {
    let mut ret = vec![];
    for w in pos.windows(2) {
        let (from, to) = (w[0], w[1]);
        if from + UP == to {
            ret.push(Dir::Up);
            continue;
        }
        if from + DOWN == to {
            ret.push(Dir::Down);
            continue;
        }
        if from + LEFT == to {
            ret.push(Dir::Left);
            continue;
        }
        if from + RIGHT == to {
            ret.push(Dir::Right);
            continue;
        }
        unreachable!()
    }
    ret
}

fn press<T: Clone + Hash + Debug + PartialEq>(
    seq: &[T],
    mut start: Pos,
    start_val: T,
    map: &[Vec<Option<T>>],
    cache: &mut FxHashMap<(Pos, Pos), Vec<Vec<Dir>>>,
) -> Vec<Vec<Dir>> {
    assert_eq!(Some(start_val), map[start.y as usize][start.x as usize]);
    let seq_pos = seq
        .iter()
        .map(|v| (v, find_button(v.clone(), map).unwrap()))
        .collect_vec();

    let mut all_possible_action_sequences: Vec<Vec<Vec<Dir>>> = vec![];

    for (_target, target_pos) in seq_pos {
        let all_shortest_path_actions = match cache.entry((start, target_pos)) {
            std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                occupied_entry.get().clone()
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let actions = find_all_shortest_path_actions(start, target_pos, map);
                vacant_entry.insert(actions.clone());
                actions
            }
        };
        all_possible_action_sequences.push(all_shortest_path_actions);
        start = target_pos;
    }

    let ret: FxHashSet<Vec<Dir>> = generate_all(&all_possible_action_sequences)
        .into_iter()
        .collect();

    ret.into_iter().collect_vec()
}

fn find_all_shortest_path_actions<T: Clone>(
    start: Pos,
    target: Pos,
    map: &[Vec<Option<T>>],
) -> Vec<Vec<Dir>> {
    let neighbours = |p: &Pos| -> Vec<(Pos, i64)> {
        let mut ret = vec![];
        for d in DIRS {
            let next = d + *p;
            if let Some(Some(_)) = next.get(map) {
                ret.push((next, 1));
            }
        }
        ret
    };

    let (_, prev) = dijkstra(&[start], neighbours);
    let all_shortest_paths = paths(&start, &target, &prev);
    let mut ret = vec![];
    for mut path in all_shortest_paths {
        path.reverse();
        let mut dirs = positions_to_dirs(&path);
        dirs.push(Dir::Activate);
        ret.push(dirs);
    }
    ret
}

fn generate_all<T: Clone + Debug>(actions_per_step: &[Vec<Vec<T>>]) -> Vec<Vec<T>> {
    let mut ret: Vec<Vec<T>> = vec![];
    for actions in &actions_per_step[0] {
        if actions_per_step.len() == 1 {
            ret.push(actions.clone());
        } else {
            let remaining = generate_all(&actions_per_step[1..]);
            // println!("{actions:?} + {remaining:?}");
            for suffix in remaining {
                let mut this = actions.clone();
                this.extend_from_slice(&suffix);
                ret.push(this.clone());
            }
        }
    }
    ret
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<String> = tokens(input, None);
    let num_panel: Vec<Vec<Option<char>>> = tokens::<String>(NUM_PANEL, None)
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| {
                    if "9876543210A".contains(c) {
                        Some(c)
                    } else {
                        None
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    let dir_panel: Vec<Vec<Option<Dir>>> = tokens::<String>(DIR_PANEL, None)
        .into_iter()
        .map(|s| s.chars().map(|c| Dir::try_from(c).ok()).collect_vec())
        .collect();
    dbg!(&lines, &num_panel, &dir_panel);

    let s = Instant::now();

    let mut num_cache: FxHashMap<(Pos, Pos), Vec<Vec<Dir>>> = Default::default();
    let mut dir_cache: FxHashMap<(Pos, Pos), Vec<Vec<Dir>>> = Default::default();

    let mut part1 = 0;
    for line in lines {
        let code_value: String = line
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let code_value: i64 = code_value.parse().unwrap();
        let mut possible_actions: Vec<Vec<Dir>> = press(
            &line.chars().collect_vec(),
            Pos::new(2, 3),
            'A',
            &num_panel,
            &mut num_cache,
        );
        println!("Posible actions: {}", possible_actions.len());

        for i in 0..2 {
            let mut possible_actions2: Vec<Vec<Dir>> = possible_actions
                .into_iter()
                .map(|actions| {
                    press(
                        &actions,
                        Pos::new(2, 0),
                        Dir::Activate,
                        &dir_panel,
                        &mut dir_cache,
                    )
                })
                .flatten()
                .collect();
            println!("Posible actions {i}: {}", possible_actions2.len());
            let min_len = possible_actions2.iter().map(|a| a.len()).min().unwrap();
            possible_actions2.retain_mut(|a| a.len() == min_len);
            println!(
                "Posible actions {i}: {} with len {min_len}",
                possible_actions2.len()
            );
            possible_actions = possible_actions2;
        }

        part1 += dbg!(
            dbg!(possible_actions
                .into_iter()
                .map(|actions| actions.len() as i64)
                .min()
                .unwrap())
                * code_value
        );
        // let actions3 = press(&actions2, Pos::new(2, 0), Dir::Activate, &dir_panel);
    }

    dbg!(&part1);
    // panic!();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(184716, part1);
        // assert_eq!(0, part2);
    }
    if output {
        println!("\t{}", part1);
        // println!("\t{}", part2);
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

pub fn paths<T: PartialEq + Eq + Hash + Clone + Ord>(
    from: &T,
    to: &T,
    prev: &FxHashMap<T, FxHashSet<T>>,
) -> Vec<Vec<T>> {
    let mut ret = vec![vec![to.clone()]];

    loop {
        let mut new_ret = vec![];
        let mut any_change = false;
        for suffix in ret {
            let last = suffix.last().unwrap();
            if last == from {
                new_ret.push(suffix.to_vec());
                continue;
            }
            for possible in prev.get(last).unwrap() {
                let mut new_suffix = suffix.clone();
                new_suffix.push(possible.clone());
                new_ret.push(new_suffix);
                any_change = true;
            }
        }
        ret = new_ret;
        if !any_change {
            break;
        }
    }
    debug_assert!(ret.iter().all(|path| path.len() == ret[0].len()));
    // ret.sort_unstable();
    // ret.dedup();
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_all() {
        let input: Vec<Vec<Vec<i32>>> = vec![vec![vec![1], vec![2]], vec![vec![3], vec![4]]];
        let out: Vec<Vec<i32>> = generate_all(&input);
        assert_eq!(out, vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4]]);
    }
}
