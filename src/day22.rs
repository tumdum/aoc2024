use anyhow::Result;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    collections::hash_map::Entry,
    time::{Duration, Instant},
};
use wide::i64x4;

use crate::input::tokens;

fn mix(secret: i64, v: i64) -> i64 {
    secret ^ v
}

fn prune(secret: i64) -> i64 {
    secret & 0b11111111_11111111_11111111
}

fn price(secret: i64) -> i64 {
    secret % 10
}

fn next(mut secret: i64) -> i64 {
    secret = prune(mix(secret, secret << 6));
    secret = prune(mix(secret, secret >> 5));
    secret = prune(mix(secret, secret << 11));
    secret
}

fn mixw(secret: i64x4, v: i64x4) -> i64x4 {
    secret ^ v
}
fn prunew(secret: i64x4) -> i64x4 {
    secret & i64x4::new([0b11111111_11111111_11111111; 4])
}
fn nextw(mut secret: i64x4) -> i64x4 {
    secret = prunew(mixw(secret, secret << 6));
    secret = prunew(mixw(secret, secret >> 5));
    secret = prunew(mixw(secret, secret << 11));
    secret
}

#[derive(Debug, PartialEq, Eq)]
struct Secret {
    value: i64,
    price: i64,
    change: i8,
}

fn next_n(mut secret: i64, n: usize) -> Vec<Secret> {
    let mut changes = Vec::with_capacity(n);

    for _ in 0..n {
        let v = next(secret);

        changes.push(Secret {
            value: v,
            price: price(v),
            change: (price(v) - price(secret)) as i8,
        });

        secret = v;
    }

    changes
}
fn nextw_n(mut secret: i64x4, n: usize) -> [Vec<Secret>; 4] {
    let mut changes = [
        Vec::with_capacity(n),
        Vec::with_capacity(n),
        Vec::with_capacity(n),
        Vec::with_capacity(n),
    ];

    for _ in 0..n {
        let v = nextw(secret);
        let arr = v.as_array_ref();

        changes[0].push(Secret {
            value: arr[0],
            price: price(arr[0]),
            change: (price(arr[0]) - price(secret.as_array_ref()[0])) as i8,
        });
        changes[1].push(Secret {
            value: arr[1],
            price: price(arr[1]),
            change: (price(arr[1]) - price(secret.as_array_ref()[1])) as i8,
        });
        changes[2].push(Secret {
            value: arr[2],
            price: price(arr[2]),
            change: (price(arr[2]) - price(secret.as_array_ref()[2])) as i8,
        });
        changes[3].push(Secret {
            value: arr[3],
            price: price(arr[3]),
            change: (price(arr[3]) - price(secret.as_array_ref()[3])) as i8,
        });

        secret = v;
    }

    changes
}

fn find_first_prices(c: &[Secret]) -> FxHashMap<u32, i64> {
    let mut first_price: FxHashMap<u32, i64> = Default::default();
    for seq in c.windows(4) {
        let key = [
            seq[0].change as u8,
            seq[1].change as u8,
            seq[2].change as u8,
            seq[3].change as u8,
        ];

        match first_price.entry(u32::from_be_bytes(key)) {
            Entry::Occupied(_occupied_entry) => {}
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(seq[3].price);
            }
        }
    }
    first_price
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<i64> = tokens(input, None);

    let s = Instant::now();

    let mut all_price_change: Vec<_> = lines
        .par_chunks_exact(4)
        .flat_map(|v| nextw_n(i64x4::new(v.try_into().unwrap()), 2000))
        .collect();

    let n = lines.len() % 4;
    for i in lines.len() - n..lines.len() {
        all_price_change.push(next_n(lines[i], 2000));
    }

    let part1: i64 = all_price_change
        .par_iter()
        .map(|price_changes| price_changes.last().unwrap().value)
        .sum();

    let all_seq: Vec<FxHashMap<u32, i64>> = all_price_change
        .par_iter()
        .map(|price_change| find_first_prices(&price_change))
        .collect();

    let windows: FxHashSet<u32> = all_seq
        .iter()
        .flat_map(|changes| changes.keys().cloned())
        .collect();

    let part2 = windows
        .par_iter()
        .map(|seq| {
            all_seq
                .iter()
                .flat_map(|price_changes| price_changes.get(&*seq))
                .sum::<i64>() as i64
        })
        .max()
        .unwrap();

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(20215960478, part1);
        assert_eq!(2221, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_secret_next() {
        assert_eq!(37, mix(42, 15));
        assert_eq!(16113920, prune(100000000));
        assert_eq!(15887950, next(123));
        // assert_eq!(5908254, next_n(123, 10));
    }

    #[test]
    fn wide() {
        let normal = [1i64, 2, 3, 4];
        let normalw = i64x4::new(normal);
        assert_eq!(
            nextw(normalw),
            i64x4::new([
                next(normal[0]),
                next(normal[1]),
                next(normal[2]),
                next(normal[3])
            ])
        );
    }
}
