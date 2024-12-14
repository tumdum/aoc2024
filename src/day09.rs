use anyhow::Result;
use rustc_hash::FxHashSet;
use std::{
    collections::VecDeque,
    thread::scope,
    time::{Duration, Instant},
};

use crate::input::tokens;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct File {
    id: usize,
    start: usize,
    size: i64,
}

fn print(prefix: &str, space: &[(Option<usize>, i64)]) {
    println!("{prefix} {space:?}");
    print!("{prefix}");
    for (id, size) in space {
        let name = if let Some(id) = id {
            id.to_string()
        } else {
            ".".to_owned()
        };
        for _ in 0..*size {
            print!("{name}");
        }
    }
    println!();
}

fn compress_blocks(space: &[(Option<usize>, i64)]) -> Vec<(Option<usize>, i64)> {
    let mut ret = vec![];
    let mut files: VecDeque<File> = space
        .iter()
        .enumerate()
        .flat_map(|(idx, (id, size))| {
            id.map(|id| File {
                id,
                size: *size,
                start: idx,
            })
        })
        .collect();

    let mut space: VecDeque<(Option<usize>, i64)> = space.iter().cloned().collect();
    let mut max_seen = 0;

    while let Some(mut file) = files.pop_back() {
        if max_seen >= file.id {
            break;
        }

        while let Some((cand_id, cand_size)) = space.pop_front() {
            if let Some(cand_id) = cand_id {
                if cand_id < file.id {
                    ret.push((Some(cand_id), cand_size));
                    max_seen = max_seen.max(cand_id);
                }
            } else {
                if cand_size > file.size {
                    ret.push((Some(file.id), file.size));
                    space.push_front((None, cand_size - file.size));
                    break;
                } else if cand_size == file.size {
                    ret.push((Some(file.id), file.size));
                    break;
                } else {
                    ret.push((Some(file.id), cand_size));
                    file.size -= cand_size;
                    continue;
                }
            }
        }
    }

    ret
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
    let mut ret = Vec::with_capacity(space.len());
    for (id, _) in space
        .iter()
        .enumerate()
        .filter(|(_, (file_id, _))| file_id.is_none())
    {
        ret.push(id);
    }
    ret
}

fn compress_files(space: &[(Option<usize>, i64)]) -> Vec<(Option<usize>, i64)> {
    fn find_next_free(
        free: &[usize],
        space: &[(Option<usize>, i64)],
        min_size: i64,
    ) -> Option<(usize, usize)> {
        free.iter()
            .enumerate()
            .find(|(in_free, in_space)| space[**in_space].1 >= min_size)
            .map(|(in_free, in_space)| (in_free, *in_space))
    }

    let mut result: Vec<(Option<usize>, i64)> = space.to_vec();
    let mut max_id: usize = space.iter().filter_map(|(id, _)| *id).last().unwrap() + 1;

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

            if let Some((in_free, in_space)) = find_next_free(&free_pos_v, &result, file_size) {
                if in_space > file_pos {
                    max_id = file_id;
                    continue;
                }
                let free_size = result[in_space].1;

                if free_size >= file_size {
                    result[in_space] = result[file_pos];
                    result[file_pos].0 = None;
                    if free_size > file_size {
                        result.insert(in_space + 1, (None, free_size - file_size));
                        free_pos_v = merge_free(&mut result, file_pos);
                    } else {
                        free_pos_v.remove(in_free);
                    }
                }
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

        (part1t.join().unwrap(), part2t.join().unwrap())
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
