use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Pos<T: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Debug for Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<T> Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn manhattan_dist(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline(always)]
    pub fn idx_1d(&self, w: T) -> usize {
        (self.y * w + self.x).try_into().unwrap()
    }

    #[inline(always)]
    pub fn row(&self) -> T {
        self.y
    }

    #[inline(always)]
    pub fn col(&self) -> T {
        self.x
    }
}

impl<T> Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<i64>,
    <T as TryInto<i64>>::Error: Debug,
{
    pub fn get(&self, map: &[crate::vec::StrVec]) -> Option<u8> {
        let x: i64 = self.x.try_into().unwrap();
        let y: i64 = self.y.try_into().unwrap();
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(row) = map.get(y as usize) {
            return row.get(x as usize).copied();
        }
        None
    }
    pub fn get_mut<'a>(&self, map: &'a mut [crate::vec::StrVec]) -> Option<&'a mut u8> {
        let x: i64 = self.x.try_into().unwrap();
        let y: i64 = self.y.try_into().unwrap();
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(row) = map.get_mut(y as usize) {
            return row.get_mut(x as usize);
        }
        None
    }
}

impl<T> Mul<T> for Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> AddAssign<Pos<T>> for Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + AddAssign
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    fn add_assign(&mut self, rhs: Pos<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, U> Add<Pos<U>> for Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + From<U>
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
    U: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <U as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Pos<U>) -> Self::Output {
        let x: T = rhs.x.into();
        let y: T = rhs.y.into();
        Pos::new(self.x + x, self.y + y)
    }
}

impl<T> Sub for Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
