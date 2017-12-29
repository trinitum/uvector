// Copyright 2017 Pavel Shaydo
//
// Licensed under the MIT license see LICENSE file

//! Allows access two read-only slices as a single vector.
use std::ops::Index;
use std::iter::Iterator;
use std::iter::IntoIterator;

/// Read-only array type allowing access two slices as a single continuous vector.
///
/// # Examples
///
/// ```
/// use std::collections::VecDeque;
/// use uvector::UVec;
///
/// // Return sum of the first 3 numbers in VecDeque
/// fn head3_sum(vd: &VecDeque<i32>) -> i32 {
///     let uv = UVec::new(vd.as_slices());
///     uv.range(0,3).iter().fold(0, |sum, x| sum + x)
/// }
///
/// fn main() {
///     let mut vd: VecDeque<i32> = VecDeque::new();
///     for i in 1..6 {
///         vd.push_back(i);
///     }
///     let s = head3_sum(&vd);
///     assert_eq!(s, 6);
/// }
/// ```
///
/// # Indexing
///
/// The `UVec` type allows you to access values by index, because it implements the `Index`
/// trait. For example:
///
/// ```
/// # use uvector::UVec;
/// let uv = UVec::new((&[1, 2, 3], &[4, 5, 6]));
/// assert_eq!(uv[3], 4);
/// ```
///
/// If you try accessing an index which is out of range it will panic:
///
/// ```should_panic
/// # use uvector::UVec;
/// let uv = UVec::new((&[1, 2, 3], &[4, 5, 6]));
/// assert_eq!(uv[9], 0); // it will panic
/// ```
///
/// # Ranges
///
/// You can get a subset of values using `range` method. It returns a new `UVec` which contains
/// only specified range of values:
///
/// ```
/// # use uvector::UVec;
/// let uv = UVec::new((&[1, 2, 3], &[4, 5, 6]));
/// let sub = uv.range(2, 4); // that will only contain [3, 4]
/// assert_eq!(uv[2], sub[0]);
/// assert_eq!(uv[3], sub[1]);
/// ```
///
/// # Iterator
///
/// You can get iterator for `UVec` using `iter()` method. It also implements `IntoIterator`
/// trait, so you can iterate over it directly:
///
/// ```
/// # use uvector::UVec;
/// let uv = UVec::new((&[1, 2, 3], &[4, 5, 6]));
/// let mut sum = 0;
/// for i in &uv {
///     sum += i;
/// }
/// assert_eq!(sum, 21);
/// ```
#[derive(Debug)]
pub struct UVec<'a, T: 'a> {
    s: (&'a [T], &'a [T]),
}

impl<'a, T> UVec<'a, T> {
    /// Constructs a new `UVec<T>` from a tupple of two slices
    pub fn new(s: (&'a [T], &'a [T])) -> Self {
        UVec { s }
    }
    /// Constructs a new empty `UVec<T>`
    ///
    /// ```
    /// # use uvector::UVec;
    /// let uv: UVec<u32> = UVec::empty();
    /// assert_eq!(uv.len(), 0);
    /// ```
    pub fn empty() -> Self {
        UVec { s: (&[], &[]) }
    }
    /// Returns the length of the vector. The length is determined as the sum of lengths of all the
    /// components.
    pub fn len(&self) -> usize {
        self.s.0.len() + self.s.1.len()
    }
    /// Returns iterator over `UVec`
    pub fn iter(&self) -> Iter<T> {
        Iter { pos: 0, s: self.s }
    }
    /// Returns a new UVec that only includes the values from the specified range.
    ///
    /// # Panics
    ///
    /// Panics if the specified range is not contained within the `UVec`
    pub fn range(&self, start: usize, end: usize) -> Self {
        let len1 = self.s.0.len();
        let start1 = if start < len1 { start } else { len1 };
        let end1 = if end < len1 { end } else { len1 };
        let start2 = if start < len1 { 0 } else { start - len1 };
        let end2 = if end < len1 { 0 } else { end - len1 };
        Self::new((&self.s.0[start1..end1], &self.s.1[start2..end2]))
    }
}

impl<'a, T> Index<usize> for UVec<'a, T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        let len = self.s.0.len();
        if index < len {
            &self.s.0[index]
        } else {
            &self.s.1[index - len]
        }
    }
}

/// An iterator over the elements of a `UVec`
pub struct Iter<'a, T: 'a> {
    pos: usize,
    s: (&'a [T], &'a [T]),
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let len1 = self.s.0.len();
        let pos = self.pos;
        if pos < len1 {
            self.pos += 1;
            Some(&self.s.0[pos])
        } else {
            let len2 = self.s.1.len();
            if pos < len1 + len2 {
                self.pos += 1;
                Some(&self.s.1[pos - len1])
            } else {
                None
            }
        }
    }
}

impl<'a, T> IntoIterator for UVec<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        Iter { pos: 0, s: self.s }
    }
}

impl<'a, T> IntoIterator for &'a UVec<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn index() {
        let one = &[5, 10, 15];
        let two = &[20, 25];
        let uv1 = UVec::new((one, two));
        assert_eq!(uv1.len(), 5);
        assert_eq!(uv1[0], 5);
        assert_eq!(uv1[2], 15);
        assert_eq!(uv1[3], 20);
        let uv2 = UVec::new((&[], two));
        assert_eq!(uv2.len(), 2);
        assert_eq!(uv2[0], 20);
        assert_eq!(uv2[1], 25);
    }

    #[test]
    #[should_panic]
    fn index_outofrange() {
        let uv = UVec::new((&[1, 2], &[]));
        let _r = uv[2];
    }

    #[test]
    fn subrange() {
        let uv = UVec::new((&[1, 2, 3], &[4, 5, 6]));
        let uv1 = uv.range(1, 5);
        assert_eq!(uv1.len(), 4);
        assert_eq!(uv1[0], 2);
        assert_eq!(uv1[3], 5);
        let uv2 = uv.range(0, 2);
        assert_eq!(uv2.len(), 2);
        assert_eq!(uv2[1], 2);
        let uv3 = uv.range(3, 4);
        assert_eq!(uv3.len(), 1);
        assert_eq!(uv3[0], 4);
        let uv4 = uv.range(4, 4);
        assert_eq!(uv4.len(), 0);
    }

    #[test]
    fn iter() {
        let uv = UVec::new((&[1i32, 2, 3], &[4, 5, 6]));
        assert_eq!(
            uv.range(2, 4).iter().map(|x| *x).collect::<Vec<i32>>(),
            vec![3, 4]
        );
        let mut sum = 0i32;
        for i in &uv {
            sum += i
        }
        assert_eq!(sum, 21);
        let mut sum2 = 0;
        for i in uv.range(1, 5) {
            sum2 += i
        }
        assert_eq!(sum2, 14);
    }
}
