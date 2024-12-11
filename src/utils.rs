use rustc_hash::FxHashMap;
use std::hash::Hash;

pub fn count_elements<'a, T: Hash + Eq + Copy + Clone + 'a>(
    values: impl IntoIterator<Item = &'a T>,
) -> FxHashMap<T, usize> {
    let mut ret: FxHashMap<T, usize> = Default::default();
    for v in values.into_iter() {
        *ret.entry(*v).or_default() += 1;
    }
    ret
}
