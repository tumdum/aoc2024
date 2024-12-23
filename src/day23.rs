use anyhow::Result;
use itertools::Itertools;
use num::traits::ToBytes;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::{smallvec, SmallVec};
use std::{
    sync::Mutex,
    time::{Duration, Instant},
};

use crate::input::token_groups;

#[derive(Eq, Hash, PartialEq, Default, Clone)]
struct Set {
    names: SmallVec<[u16; 13]>,
}

impl Set {
    fn len(&self) -> usize {
        self.names.len()
    }

    fn into_iter(self) -> impl Iterator<Item = u16> {
        self.names.into_iter()
    }

    fn iter(&self) -> impl Iterator<Item = &u16> {
        self.names.iter()
    }

    fn contains(&self, v: &u16) -> bool {
        self.names.binary_search(v).is_ok()
    }

    fn insert(&mut self, v: u16) {
        match self.names.binary_search(&v) {
            Ok(_) => { /*already here*/ }
            Err(i) => {
                self.names.insert(i, v);
            }
        }
    }

    fn intersection(&self, other: &Self) -> Self {
        let mut names = smallvec![];
        for v in &self.names {
            for k in &other.names {
                if v == k {
                    names.push(*v);
                }
            }
        }
        names.sort_unstable();
        names.dedup();
        Self { names }
    }

    fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
}

impl FromIterator<u16> for Set {
    fn from_iter<T: IntoIterator<Item = u16>>(iter: T) -> Self {
        let names = iter.into_iter().collect();
        Self { names }
    }
}

impl<'a> IntoIterator for &'a Set {
    type Item = &'a u16;

    type IntoIter = std::slice::Iter<'a, u16>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}

impl IntoIterator for Set {
    type Item = u16;

    type IntoIter = smallvec::IntoIter<[u16; 13]>;

    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

fn find_connected_with_all(set: &Set, connections: &FxHashMap<u16, Set>) -> Set {
    connections
        .keys()
        .filter(|cand| {
            set.iter()
                .all(|v| v != *cand && is_connected(*v, **cand, connections))
        })
        .cloned()
        .collect()
}

fn is_connected(a: u16, b: u16, map: &FxHashMap<u16, Set>) -> bool {
    if let Some(n) = map.get(&a) {
        return n.contains(&b);
    }
    false
}

fn all_connected(set: &Set, map: &FxHashMap<u16, Set>) -> bool {
    for a in set {
        for b in set {
            if a != b {
                if !is_connected(*a, *b, map) {
                    return false;
                }
            }
        }
    }
    true
}

fn flood(start: &u16, map: &FxHashMap<u16, Set>) -> Set {
    let mut cands: FxHashSet<Set> = Default::default();
    let mut last: Set = Set::default();

    let mut starting: Set = Default::default();
    starting.insert(start.to_owned());
    cands.insert(starting);

    while !cands.is_empty() {
        let mut new_cands: FxHashSet<Set> = Default::default();

        for cand in cands {
            let next = find_connected_with_all(&cand, map);
            if next.is_empty() {
                last = cand;
                break;
            } else {
                new_cands.extend(next.into_iter().map(|next| {
                    let mut s = cand.clone();
                    s.insert(next);
                    debug_assert!(all_connected(&s, map));
                    s
                }));
            }
        }

        cands = new_cands;
    }

    last
}

fn third(
    a: &u16,
    b: &u16,
    map: &FxHashMap<u16, Set>,
    pred: impl Fn(&u16) -> bool,
) -> Vec<Vec<u16>> {
    let mut ret: Vec<Vec<u16>> = vec![];
    let aset: &Set = map.get(a).unwrap();
    let bset: &Set = map.get(b).unwrap();
    let common = aset.intersection(bset);
    for third in common {
        if third != *a && third != *b {
            if pred(a) || pred(b) || pred(&third) {
                let mut set = vec![a.to_owned(), b.to_owned(), third.to_owned()];
                set.sort();
                ret.push(set);
            }
        }
    }
    ret
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<u16>> = token_groups::<String>(input, "\n", Some("-"))
        .into_iter()
        .map(|l| {
            l.iter()
                .map(|c| {
                    let tmp: [u8; 2] = c.bytes().collect_vec().try_into().unwrap();
                    u16::from_be_bytes(tmp)
                })
                .collect_vec()
        })
        .collect_vec();

    let s = Instant::now();

    let mut connections: FxHashMap<u16, Set> = Default::default();
    for conn in lines {
        assert_eq!(2, conn.len());
        connections
            .entry(conn[0].clone())
            .or_default()
            .insert(conn[1].clone());
        connections
            .entry(conn[1].clone())
            .or_default()
            .insert(conn[0].clone());
    }

    let mut sets3: FxHashSet<Vec<u16>> = Default::default();
    for (conn, neighbours) in &connections {
        for other in neighbours {
            sets3.extend(third(conn, &other, &connections, |name| {
                name.to_be_bytes()[0] == b't'
            }));
        }
    }
    let part1 = sets3.len();

    use rayon::prelude::*;

    let keys = connections.keys().copied().collect_vec();
    let seen: Mutex<FxHashSet<u16>> = Default::default();
    let n = keys.len() / num_cpus::get();
    let largest_set = keys
        .par_chunks(n)
        .map(|keys| solve_subset(keys, &connections, &seen))
        .max_by_key(|s| s.len())
        .unwrap();
    let part2: String = largest_set
        .into_iter()
        .map(|cn| {
            format!(
                "{}{}",
                cn.to_be_bytes()[0] as char,
                cn.to_be_bytes()[1] as char
            )
        })
        .join(",");

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(1302, part1);
        assert_eq!("cb,df,fo,ho,kk,nw,ox,pq,rt,sf,tq,wi,xz", part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn solve_subset(
    keys: &[u16],
    connections: &FxHashMap<u16, Set>,
    seen: &Mutex<FxHashSet<u16>>,
) -> Set {
    let mut sets: FxHashSet<Set> = Default::default();
    for computer in keys {
        if seen.lock().unwrap().contains(computer) {
            continue;
        }
        let set = flood(&computer, &connections);
        seen.lock().unwrap().extend(set.iter().cloned());
        sets.insert(set.into_iter().collect());
    }

    if sets.is_empty() {
        return Default::default();
    }

    let largest = sets.iter().map(|s| s.len()).max().unwrap();
    let largest_set = sets.into_iter().find(|s| s.len() == largest).unwrap();
    largest_set
}
