use crate::input::tokens;
use anyhow::Result;
use itertools::{repeat_n, Itertools};
use maplit::hashmap;
use once_cell::sync::Lazy;
use rustc_hash::FxHashMap;
use std::iter::once;
use std::time::{Duration, Instant};

type Pos = crate::pos::Pos<i32>;

static NUMPAD: Lazy<FxHashMap<char, Pos>> = Lazy::new(|| {
    hashmap! {
        '7' => Pos::new(0, 0), '8' => Pos::new(1, 0), '9' => Pos::new(2, 0), '4' => Pos::new(0, 1), '5' => Pos::new(1, 1), '6' => Pos::new(2, 1),
        '1' => Pos::new(0, 2), '2' => Pos::new(1, 2), '3' => Pos::new(2, 2), '0' => Pos::new(1, 3), 'A' => Pos::new(2, 3)
    }
    .into_iter()
    .collect()
});

static DIRPAD: Lazy<FxHashMap<char, Pos>> = Lazy::new(|| {
    hashmap! {
        '^' => Pos::new(1, 0), 'A' => Pos::new(2, 0), '<' => Pos::new(0, 1), 'v' => Pos::new(1, 1), '>' => Pos::new(2, 1)
    }
    .into_iter()
    .collect()
});

static DIRS: Lazy<FxHashMap<char, Pos>> = Lazy::new(|| {
    hashmap! {
        '^' => Pos::new(0, -1), '>' =>  Pos::new(1, 0), 'v' => Pos::new(0, 1), '<' =>  Pos::new(-1, 0)
    }
    .into_iter()
    .collect()
});

pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<String> = tokens(input, None);

    let s = Instant::now();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut cache = Default::default();
    for line in lines {
        let code_value: String = line
            .chars()
            .skip_while(|c| !c.is_ascii_digit())
            .take_while(|c| c.is_ascii_digit())
            .collect();
        let code_value: u64 = code_value.parse().unwrap();
        let shortest_steps =
            shortest_steps_count(&line.chars().collect_vec(), 2, false, None, &mut cache);
        let shortest_steps_part2 =
            shortest_steps_count(&line.chars().collect_vec(), 25, false, None, &mut cache);

        part1 += shortest_steps * code_value;
        part2 += shortest_steps_part2 * code_value;
    }

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(184716, part1);
        assert_eq!(229403562787554, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}

fn shortest_steps_count(
    sequence: &[char],
    depth: usize,
    is_dir_pad: bool,
    mut curr: Option<Pos>,
    cache: &mut FxHashMap<(String, usize, bool, Option<Pos>), u64>,
) -> u64 {
    if sequence.is_empty() {
        return 0;
    }
    let key = (sequence.iter().copied().collect(), depth, is_dir_pad, curr);
    if let Some(ret) = cache.get(&key) {
        return *ret;
    }

    let keypad = if is_dir_pad {
        DIRPAD.clone()
    } else {
        NUMPAD.clone()
    };

    if curr.is_none() {
        curr = Some(keypad[&'A']);
    }

    let p = keypad[&sequence[0]];
    let d = p - curr.unwrap();

    let buttons: Vec<char> = repeat_n('>', d.x.max(0) as usize)
        .chain(repeat_n('<', (-d.x).max(0) as usize))
        .chain(repeat_n('v', d.y.max(0) as usize))
        .chain(repeat_n('^', (-d.y).max(0) as usize))
        .collect();

    let best_solution_for_first = if depth > 0 {
        buttons
            .iter()
            .permutations(buttons.len())
            .filter(|perm| is_valid_perm(curr.unwrap(), perm, &keypad))
            .map(|perm| {
                shortest_steps_count(
                    &perm.iter().chain(once(&&'A')).map(|c| **c).collect_vec(),
                    depth - 1,
                    true,
                    None,
                    cache,
                )
            })
            .min()
            .unwrap()
    } else {
        buttons.len() as u64 + 1
    };
    let solution = best_solution_for_first
        + shortest_steps_count(&sequence[1..], depth, is_dir_pad, Some(p), cache);
    cache.insert(key, solution);
    solution
}

fn is_valid_perm(mut curr: Pos, perm: &[&char], keypad: &FxHashMap<char, Pos>) -> bool {
    for button in perm {
        curr += DIRS[button];
        if !keypad.values().contains(&curr) {
            return false;
        }
    }
    true
}
