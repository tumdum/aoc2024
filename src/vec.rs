use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
    str::{from_utf8, FromStr},
};

use itertools::Itertools;
use num::integer::Roots;
use smallvec::{smallvec, SmallVec};

pub fn get_at<T>(x: i64, y: i64, dx: i64, dy: i64, input: &[Vec<T>]) -> Option<&T> {
    let (x, y) = (x + dx, y + dy);
    if x < 0 || y < 0 {
        return None;
    }
    if let Some(line) = input.get(y as usize) {
        return line.get(x as usize);
    }
    None
}

pub fn transpose<T: Clone, const N: usize>(original: &[SmallVec<[T; N]>]) -> Vec<SmallVec<[T; N]>> {
    assert!(!original.is_empty());
    let mut transposed: Vec<SmallVec<[T; N]>> = vec![smallvec![]; original[0].len()];

    for original_row in original {
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item.clone());
        }
    }

    transposed
}
pub fn transpose_vec<T: Clone>(original: &[Vec<T>]) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let mut transposed: Vec<Vec<T>> = vec![vec![]; original[0].len()];

    for original_row in original {
        for (item, transposed_row) in original_row.iter().zip(&mut transposed) {
            transposed_row.push(item.clone());
        }
    }

    transposed
}

pub fn diagonals(input: &[String]) -> Vec<String> {
    let input: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();
    let (w, h) = (input[0].len(), input.len());

    let mut ret = vec![];

    for len in 1..((w * h).sqrt() + 1) {
        let mut tmp = vec![];
        for i in 0..len {
            let l = &input[i];
            tmp.push(l[l.len() - len + i]);
        }
        let tmp: String = tmp.into_iter().collect();
        ret.push(tmp);
    }
    for (v, len) in (0..(w * h).sqrt()).skip(1).rev().enumerate() {
        let mut tmp = vec![];
        for i in 1..(len + 1) {
            let l = &input[i + v];
            tmp.push(l[i - 1]);
        }
        let tmp: String = tmp.into_iter().collect();
        ret.push(tmp);
    }

    ret
}

#[derive(Clone)]
pub struct StrVec(Vec<u8>);

impl Deref for StrVec {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for StrVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(|c| *c as char).collect();
        s.fmt(f)
    }
}

impl std::fmt::Debug for StrVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = self.0.iter().map(|c| *c as char).collect();
        write!(f, "\"{s}\"")
    }
}

impl FromStr for StrVec {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.bytes().collect()))
    }
}
impl DerefMut for StrVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn print_2d_map(m: &[StrVec]) {
    let (h, w) = (m.len(), m[0].len());

    const DIGITS: [u8; 10] = *b"0123456789";
    const HEADER_PREFIX: &str = "    ";
    let header = format!(
        "{HEADER_PREFIX}{}",
        from_utf8(&DIGITS.iter().cycle().take(w).copied().collect_vec()).unwrap()
    );
    let second_header = format!(
        "{HEADER_PREFIX} {}",
        DIGITS
            .iter()
            .cycle()
            .skip(1)
            .take((w - 1) / 10)
            .copied()
            .map(|c| format!("         {}", c as char))
            .collect_vec()
            .join("")
    );

    println!("{second_header}");
    println!("{header}");
    for y in 0..h {
        print!("{y:3} ");
        for x in 0..w {
            print!("{}", m[y][x] as char);
        }
        println!()
    }
    println!("{header}");
    println!("{second_header}");
}
