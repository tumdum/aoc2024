use std::{fmt::Display, ops::Deref, str::FromStr};

use smallvec::{smallvec, SmallVec};

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
        for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
            transposed_row.push(item.clone());
        }
    }

    transposed
}

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
