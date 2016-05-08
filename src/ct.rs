//! Constant Time

use std::u64::MIN;

/// ```
/// use std::u64::{ MIN, MAX };
/// use rlwekex::ct::isnonzero;
/// assert_eq!(isnonzero(1), 1);
/// assert_eq!(isnonzero(0), 0);
/// assert_eq!(isnonzero(MIN), 0);
/// assert_eq!(isnonzero(MAX.wrapping_add(1)), 0);
/// ```
pub fn isnonzero(x: u64) -> u64 {
    (x | (!x).wrapping_add(1)) >> 63
}

/// ```
/// use std::u64::{ MIN, MAX };
/// use rlwekex::ct::ne;
/// assert_eq!(ne(1, 2), 1);
/// assert_eq!(ne(1, 1), 0);
/// assert_eq!(ne(MIN, MAX.wrapping_add(1)), 0);
/// ```
pub fn ne(x: u64, y: u64) -> u64 {
    (x.wrapping_sub(y) | y.wrapping_sub(x)) >> 63
}

/// ```
/// use rlwekex::ct::eq;
/// assert_eq!(eq(1, 2), 0);
/// assert_eq!(eq(0, 0), 1);
/// ```
pub fn eq(x: u64, y: u64) -> u64 {
    1 ^ ne(x, y)
}

/// ```
/// use std::u64::MAX;
/// use rlwekex::ct::lt;
/// assert_eq!(lt(1, 2), 1);
/// assert_eq!(lt(1, 1), 0);
/// assert_eq!(lt(2, 1), 0);
/// assert_eq!(lt(MAX, MAX.wrapping_add(1)), 0);
/// ```
pub fn lt(x: u64, y: u64) -> u64 {
    (x ^ ((x ^ y) | (x.wrapping_sub(y) ^ y))) >> 63
}

/// ```
/// use rlwekex::ct::gt;
/// assert_eq!(gt(1, 2), 0);
/// assert_eq!(gt(2, 1), 1);
/// assert_eq!(gt(1, 1), 0);
/// ```
pub fn gt(x: u64, y: u64) -> u64 {
    lt(y, x)
}

/// ```
/// use rlwekex::ct::le;
/// assert_eq!(le(1, 2), 1);
/// assert_eq!(le(1, 1), 1);
/// assert_eq!(le(2, 1), 0);
/// ```
pub fn le(x: u64, y: u64) -> u64 {
    1 ^ gt(x, y)
}

/// ```
/// use rlwekex::ct::ge;
/// assert_eq!(ge(1, 2), 0);
/// assert_eq!(ge(0, 0), 1);
/// assert_eq!(ge(2, 1), 1);
/// ```
pub fn ge(x: u64, y: u64) -> u64 {
    1 ^ lt(x, y)
}

/// ```
/// use std::u64::MAX;
/// use rlwekex::ct::mask;
/// assert_eq!(mask(1), MAX);
/// assert_eq!(mask(0), 0);
/// ```
pub fn mask(bit: u64) -> u64 {
    MIN.wrapping_sub(isnonzero(bit))
}

/// ```
/// use rlwekex::ct::select;
/// assert_eq!(select(2, 3, 0), 3);
/// assert_eq!(select(2, 3, 1), 2);
/// assert_eq!(select(2, 3, 99), 2);
/// ```
pub fn select(x: u64, y: u64, bit: u64) -> u64 {
    let m = mask(bit);
    (x & m) | (y & !m)
}

/// ```
/// use rlwekex::ct::cmplt;
/// assert_eq!(cmplt(&[1, 2, 3], &[3, 2, 1]), 0);
/// assert_eq!(cmplt(&[9, 9, 9], &[0, 0, 0]), 0);
/// assert_eq!(cmplt(&[2, 2, 2], &[1, 3, 3]), 1);
/// ```
pub fn cmplt(a: &[u64], b: &[u64]) -> u64 {
    let (mut r, mut m) = (0, 0);
    for i in (0..3).rev() {
        r |= lt(a[i], b[i]) & !m;
        m |= mask(ne(a[i], b[i]));
    }

    r & 1
}
