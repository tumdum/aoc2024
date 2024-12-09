use anyhow::Result;
use std::{
    thread::scope,
    time::{Duration, Instant},
};

use crate::input::tokens;

fn compress_blocks(space: &[(Option<usize>, i64)]) -> Vec<(Option<usize>, i64)> {
    fn find_next_free(space: &[(Option<usize>, i64)]) -> Option<usize> {
        space.iter().position(|(id, _size)| id.is_none())
    }
    let mut result: Vec<(Option<usize>, i64)> = space.to_vec();

    loop {
        let file_to_compress = result.iter().rposition(|(id, _)| id.is_some());
        if let Some(file_pos) = file_to_compress {
            let file_id = result[file_pos].0.unwrap();
            let file_size = result[file_pos].1;

            if let Some(free_pos) = find_next_free(&result) {
                if free_pos > file_pos {
                    return result;
                }
                let free_size = result[free_pos].1;

                if free_size <= file_size {
                    result[free_pos].0 = Some(file_id);
                    let new_size = file_size - free_size;
                    if new_size == 0 {
                        result[file_pos].0 = None;
                    } else {
                        result[file_pos].1 = new_size;
                    }
                } else {
                    result[free_pos] = result[file_pos];
                    result[file_pos].0 = None;
                    result.insert(free_pos + 1, (None, free_size - file_size));
                }
            }
        } else {
            return result;
        }
    }
}

fn merge_free(space: &mut Vec<(Option<usize>, i64)>, file_pos: usize) -> Vec<usize> {
    loop {
        let mut changed = false;
        for i in (file_pos - 1)..(file_pos + 3).min(space.len()) {
            if space[i].0.is_none() && space[i - 1].0.is_none() {
                changed = true;
                space[i - 1].1 += space[i].1;
                space.remove(i);
                break;
            }
        }
        if !changed {
            break;
        }
    }
    space
        .iter()
        .enumerate()
        .filter(|(_, (file_id, _))| file_id.is_none())
        .map(|(id, _)| id)
        .collect()
}

fn compress_files(space: &[(Option<usize>, i64)]) -> Vec<(Option<usize>, i64)> {
    fn find_next_free(
        free: &[usize],
        space: &[(Option<usize>, i64)],
        min_size: i64,
    ) -> Option<usize> {
        free.iter().find(|pos| space[**pos].1 >= min_size).copied()
    }

    let mut result: Vec<(Option<usize>, i64)> = space.to_vec();
    let mut max_id: usize = space
        .iter()
        .filter_map(|(id, _)| id.clone())
        .last()
        .unwrap()
        + 1;

    let mut free_pos_v: Vec<usize> = space
        .iter()
        .enumerate()
        .filter(|(_, (file_id, _))| file_id.is_none())
        .map(|(id, _)| id)
        .collect();

    loop {
        let file_to_compress = result.iter().rposition(|(id, _)| {
            if let Some(id) = id {
                *id < max_id
            } else {
                false
            }
        });
        if let Some(file_pos) = file_to_compress {
            let (file_id, file_size) = (result[file_pos].0.unwrap(), result[file_pos].1);

            if let Some(free_pos) = find_next_free(&free_pos_v, &result, file_size) {
                if free_pos > file_pos {
                    max_id = file_id;
                    continue;
                }
                let free_size = result[free_pos].1;

                if free_size >= file_size {
                    result[free_pos] = result[file_pos];
                    result[file_pos].0 = None;
                    if free_size > file_size {
                        result.insert(free_pos + 1, (None, free_size - file_size));
                    }
                }
                free_pos_v = merge_free(&mut result, file_pos);
            } else {
                max_id = file_id;
            }
        } else {
            break;
        }
    }

    result
}

fn checksum(space: &[(Option<usize>, i64)]) -> usize {
    let mut result = 0;
    let mut curr = 0;

    for (id, size) in space {
        if let Some(file_id) = id {
            for _ in 0..*size {
                result += curr * file_id;
                curr += 1;
            }
        } else {
            curr += *size as usize;
        }
    }

    result
}
pub fn solve(input: &str, verify_expected: bool, output: bool) -> Result<Duration> {
    let lines: Vec<String> = tokens(input, None);
    let disk = lines[0].clone();
    let s = Instant::now();

    let mut space: Vec<(Option<usize>, i64)> = vec![];
    let mut id = 0;
    let mut free = false;
    for size in disk.bytes() {
        let size = (size - b'0') as i64;
        if !free {
            space.push((Some(id), size));
            id += 1;
        } else {
            space.push((None, size));
        }
        free = !free;
    }

    let (part1, part2) = scope(move |s| {
        let space1 = space.clone();
        let part1t = s.spawn(move || checksum(&compress_blocks(&space1)));
        let part2t = s.spawn(move || checksum(&compress_files(&space)));
        let part1 = part1t.join().unwrap();
        let part2 = part2t.join().unwrap();
        (part1, part2)
    });

    let e = s.elapsed();

    if verify_expected {
        assert_eq!(6435922584968, part1);
        assert_eq!(6469636832766, part2);
    }
    if output {
        println!("\t{}", part1);
        println!("\t{}", part2);
    }
    Ok(e)
}
