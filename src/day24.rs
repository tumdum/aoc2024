use anyhow::Result;
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::time::{Duration, Instant};

use crate::input::token_groups;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Op {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl Debug for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::And(a, b) => write!(f, "{a} AND {b}"),
            Op::Or(a, b) => write!(f, "{a} OR {b}"),
            Op::Xor(a, b) => write!(f, "{a} XOR {b}"),
        }
    }
}

impl Op {
    fn names(&self) -> [&str; 2] {
        match self {
            Op::And(a, b) | Op::Or(a, b) | Op::Xor(a, b) => [a, b],
        }
    }
    fn eval(&self, values: &FxHashMap<String, bool>) -> Option<bool> {
        let names = self.names();
        match (values.get(names[0]), values.get(names[1])) {
            (None, None) => None,
            (None, Some(_)) => None,
            (Some(_), None) => None,
            (Some(l), Some(r)) => match self {
                Op::And(_, _) => Some(*l && *r),
                Op::Or(_, _) => Some(*l || *r),
                Op::Xor(_, _) => Some(*l ^ *r),
            },
        }
    }
}

fn parse(s: &[String]) -> (String, Op) {
    match s[1].as_str() {
        "AND" => (s[4].clone(), Op::And(s[0].clone(), s[2].clone())),
        "OR" => (s[4].clone(), Op::Or(s[0].clone(), s[2].clone())),
        "XOR" => (s[4].clone(), Op::Xor(s[0].clone(), s[2].clone())),
        _ => unreachable!(),
    }
}

fn step(values: &mut FxHashMap<String, bool>, g: &FxHashMap<String, Op>) -> bool {
    let mut made_changes = false;
    for (wire, op) in g {
        if values.contains_key(wire) {
            continue;
        }
        if let Some(new_value) = op.eval(values) {
            made_changes = true;
            values.insert(wire.clone(), new_value);
        }
    }
    return made_changes;
}

fn step_till_no_change(values: &mut FxHashMap<String, bool>, g: &FxHashMap<String, Op>) {
    loop {
        if !step(values, g) {
            break;
        }
    }
}

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<Vec<String>> = token_groups(input, "\n\n", None);

    let s = Instant::now();
    let start: FxHashMap<String, bool> = lines[0]
        .chunks_exact(2)
        .map(|v| (v[0].strip_suffix(":").unwrap().to_owned(), v[1] == "1"))
        .collect();

    let g: FxHashMap<String, Op> = lines[1].chunks_exact(5).map(|line| parse(line)).collect();

    let part1;
    {
        let mut values = start.clone();
        step_till_no_change(&mut values, &g);
        let output: u64 = u64::from_str_radix(
            &values
                .into_iter()
                .filter(|(k, _)| k.starts_with("z"))
                .sorted_unstable_by_key(|(k, _)| k.to_owned())
                .rev()
                .map(|(_, v)| if v { '1' } else { '0' })
                .collect::<String>(),
            2,
        )
        .unwrap();

        part1 = output;
    }

    fn is_bit(s: &str) -> bool {
        s.starts_with('x') || s.starts_with('y') || s.starts_with('z')
    }

    let mut ans: BTreeSet<String> = Default::default();
    for (out, op) in &g {
        if out.starts_with("z") && !matches!(op, Op::Xor(_, _)) && out != "z45" {
            ans.insert(out.to_owned());
        } else if matches!(op, Op::Xor(_, _))
            && !is_bit(out)
            && !is_bit(op.names()[0])
            && !is_bit(op.names()[1])
        {
            ans.insert(out.to_owned());
        } else if matches!(op, Op::Xor(_, _)) {
            for sub in g.values() {
                if matches!(sub, Op::Or(_, _)) && (sub.names().contains(&out.as_str())) {
                    ans.insert(out.to_owned());
                }
            }
        } else if matches!(op, Op::And(_, _)) && !op.names().contains(&"x00") {
            for sub in g.values() {
                if !matches!(sub, Op::Or(_, _)) && (sub.names().contains(&out.as_str())) {
                    ans.insert(out.to_owned());
                }
            }
        }
    }
    let part2 = ans.into_iter().join(",");

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(60714423975686, part1);
        assert_eq!("cgh,frt,pmd,sps,tst,z05,z11,z23", part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }

    Ok(e)
}
