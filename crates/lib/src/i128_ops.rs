// ================
// === i128 ops ===
// ================

pub const POW10: [i128; 39] = {
    let mut arr = [1i128; 39];
    let mut val = 1;
    let mut i = 0;
    loop {
        arr[i] = val;
        if i == 38 { break }
        val *= 10;
        i += 1;
    }
    arr
};

pub const P0:  i128 = 1;
pub const P1:  i128 = P0  * 10;
pub const P2:  i128 = P1  * 10;
pub const P3:  i128 = P2  * 10;
pub const P4:  i128 = P3  * 10;
pub const P5:  i128 = P4  * 10;
pub const P6:  i128 = P5  * 10;
pub const P7:  i128 = P6  * 10;
pub const P8:  i128 = P7  * 10;
pub const P9:  i128 = P8  * 10;
pub const P10: i128 = P9  * 10;
pub const P11: i128 = P10 * 10;
pub const P12: i128 = P11 * 10;
pub const P13: i128 = P12 * 10;
pub const P14: i128 = P13 * 10;
pub const P15: i128 = P14 * 10;
pub const P16: i128 = P15 * 10;
pub const P17: i128 = P16 * 10;
pub const P18: i128 = P17 * 10;
pub const P19: i128 = P18 * 10;
pub const P20: i128 = P19 * 10;
pub const P21: i128 = P20 * 10;
pub const P22: i128 = P21 * 10;
pub const P23: i128 = P22 * 10;
pub const P24: i128 = P23 * 10;
pub const P25: i128 = P24 * 10;
pub const P26: i128 = P25 * 10;
pub const P27: i128 = P26 * 10;
pub const P28: i128 = P27 * 10;
pub const P29: i128 = P28 * 10;
pub const P30: i128 = P29 * 10;
pub const P31: i128 = P30 * 10;
pub const P32: i128 = P31 * 10;
pub const P33: i128 = P32 * 10;
pub const P34: i128 = P33 * 10;
pub const P35: i128 = P34 * 10;
pub const P36: i128 = P35 * 10;
pub const P37: i128 = P36 * 10;
pub const P38: i128 = P37 * 10;

/// Get the scale factor for rounding to a given number of digits
#[inline(always)]
pub(crate) const fn scale_for(digits: i64) -> i128 {
    let digits = if digits < -19 { -19 } else if digits > 19 { 19 } else { digits };
    let idx = (19 - digits) as usize;
    POW10[idx]
}

/// Returns the number of decimal digits in an `i128`.
///
/// This function calculates how many digits are needed to represent the absolute value of the input
/// number in base 10. The result is always in the range `1..=39`, inclusive. It uses a
/// fully-unrolled, balanced binary tree of comparisons for maximum performance.
///
/// # Tests
///
/// ```
/// use fixed_num::i128_ops::*;
///
/// assert_eq!(digit_count(0), 1);
/// for i in 1..=38 {
///     let j = i as i32;
///     assert_eq!(digit_count(POW10[i]-1), j);
///     assert_eq!(digit_count(POW10[i]),   j + 1);
/// }
/// assert_eq!(digit_count(i128::MAX), 39);
/// assert_eq!(digit_count(i128::MIN), 39);
/// ```
#[expect(clippy::cognitive_complexity)]
#[expect(clippy::collapsible_else_if)]
#[inline(always)]
pub const fn digit_count(n: i128) -> i32 {
    if n == i128::MIN {
        return 39;
    }
    let n = n.abs();

    if n < P19 {
        if n < P9 {
            if n < P4 {
                if n < P2 {
                    if n < P1 { 1 } else { 2 }
                } else {
                    if n < P3 { 3 } else { 4 }
                }
            } else {
                if n < P7 {
                    if n < P5 { 5 } else if n < P6 { 6 } else { 7 }
                } else {
                    if n < P8 { 8 } else { 9 }
                }
            }
        } else {
            if n < P14 {
                if n < P12 {
                    if n < P10 { 10 } else if n < P11 { 11 } else { 12 }
                } else {
                    if n < P13 { 13 } else { 14 }
                }
            } else {
                if n < P17 {
                    if n < P15 { 15 } else if n < P16 { 16 } else { 17 }
                } else {
                    if n < P18 { 18 } else { 19 }
                }
            }
        }
    } else {
        if n < P29 {
            if n < P24 {
                if n < P22 {
                    if n < P20 { 20 } else if n < P21 { 21 } else { 22 }
                } else {
                    if n < P23 { 23 } else { 24 }
                }
            } else {
                if n < P27 {
                    if n < P25 { 25 } else if n < P26 { 26 } else { 27 }
                } else {
                    if n < P28 { 28 } else { 29 }
                }
            }
        } else {
            if n < P34 {
                if n < P32 {
                    if n < P30 { 30 } else if n < P31 { 31 } else { 32 }
                } else {
                    if n < P33 { 33 } else { 34 }
                }
            } else {
                if n < P37 {
                    if n < P35 { 35 } else if n < P36 { 36 } else { 37 }
                } else {
                    if n < P38 { 38 } else { 39 }
                }
            }
        }
    }
}
