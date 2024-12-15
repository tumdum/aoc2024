use anyhow::Result;
use rustc_hash::FxHashMap;

use std::{
    cmp::Reverse,
    collections::VecDeque,
    time::{Duration, Instant},
};

use crate::input::tokens;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct File {
    id: usize,
    start: usize,
    size: i64,
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

fn compress_files(space: &[(Option<usize>, i64)]) -> usize {
    let add = |l: &mut Vec<(i64, Vec<Reverse<usize>>)>, size: i64, offset: usize| {
        debug_assert!(l.is_sorted_by_key(|(size, _)| *size));
        match l.binary_search_by_key(&size, |(s, _)| *s) {
            Ok(idx) => {
                let insert_idx = match l[idx].1.binary_search(&Reverse(offset)) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                l[idx].1.insert(insert_idx, Reverse(offset));
            }
            Err(idx) => {
                l.insert(idx, (size, vec![Reverse(offset)]));
            }
        }
        debug_assert!(l.is_sorted_by_key(|(size, _)| *size));
    };
    let remove_first = |l: &mut Vec<(i64, Vec<Reverse<usize>>)>,
                        size: i64,
                        file_start: usize|
     -> Option<(i64, usize)> {
        debug_assert!(l.is_sorted_by_key(|(size, _)| *size));
        let start_idx = match l.binary_search_by_key(&size, |(s, _)| *s) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        let mut earliest: Option<(usize, i64, usize)> = None;
        for idx in start_idx..l.len() {
            if l[idx].1.is_empty() {
                continue;
            }

            debug_assert!(l[idx].0 >= size);
            debug_assert!(l[idx].1.is_sorted());
            let last = l[idx].1.len() - 1;
            let ret = l[idx].1[last];
            if ret.0 > file_start {
                continue;
            }

            if let Some(e) = earliest {
                if e.2 > ret.0 {
                    earliest = Some((idx, l[idx].0, ret.0))
                }
            } else {
                earliest = Some((idx, l[idx].0, ret.0))
            }
        }

        if let Some((idx, size, offset)) = earliest {
            let last = l[idx].1.len() - 1;
            l[idx].1.remove(last);
            Some((size, offset))
        } else {
            None
        }
    };
    let mut free_list: Vec<(i64, Vec<Reverse<usize>>)> = vec![];
    let mut orig_files: FxHashMap<usize, (usize, i64)> = Default::default();
    let mut files: Vec<File> = vec![];
    let mut files_todo: VecDeque<File> = VecDeque::default();

    let mut offset = 0;
    for entry in space {
        if entry.0.is_none() {
            add(&mut free_list, entry.1, offset);
        } else {
            orig_files.insert(entry.0.unwrap(), (offset, entry.1));
            files_todo.push_back(File {
                id: entry.0.unwrap(),
                start: offset,
                size: entry.1,
            });
        }
        offset += entry.1 as usize;
    }

    while let Some(file) = files_todo.pop_back() {
        if let Some((free_size, free_offset)) = remove_first(&mut free_list, file.size, file.start)
        {
            debug_assert!(free_size >= file.size);

            files.push(File {
                id: file.id,
                start: free_offset,
                size: file.size,
            });
            if free_size > file.size {
                let new_free_size = free_size - file.size;
                let new_free_offset = free_offset + file.size as usize;
                add(&mut free_list, new_free_size, new_free_offset);
            }
        } else {
            let (offset, size) = orig_files.get(&file.id).unwrap();
            files.push(File {
                id: file.id,
                start: *offset,
                size: *size,
            });
        }
    }

    files.sort_unstable_by_key(|file| file.start);

    let mut cksum = 0;

    for file in files {
        let mut curr = file.start;
        for _ in 0..file.size {
            cksum += curr * file.id;
            curr += 1;
        }
    }

    cksum
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

    let part1 = checksum(&compress_blocks(&space));
    let part2 = compress_files(&space);

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
