use std::error::Error;
use std::fmt::Display;

// ==============
// === Consts ===
// ==============

/// The number of digits after the dot.
pub const FRAC_PLACES: u32 = 19;

/// Scale that moves [`FRAC_PLACES`] fractional digits into the integer part when multiplied.
pub const FRAC_SCALE_U128: u128 = 10_u128.pow(FRAC_PLACES);
pub const FRAC_SCALE_I128: i128 = FRAC_SCALE_U128 as i128;

// ======================
// === ParseF128Error ===
// ======================

#[derive(Debug, Eq, PartialEq)]
pub enum ParseDec19x19Error {
    Empty,
    ParseIntError(std::num::ParseIntError),
    OutOfBounds,
    TooPrecise,
    InvalidChar { char: char, pos: usize },
}

impl From<std::num::ParseIntError> for ParseDec19x19Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl Error for ParseDec19x19Error {}
impl Display for ParseDec19x19Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Empty input"),
            Self::ParseIntError(err) => Display::fmt(err, f),
            Self::OutOfBounds => write!(f, "Value out of bounds"),
            Self::TooPrecise => write!(f, "Value too precise"),
            Self::InvalidChar { char, pos } =>
                write!(f, "Invalid character `{char}` at position {pos}"),
        }
    }
}

// ===============
// === Parsing ===
// ===============

/// Shifts digits between the integer and fractional part strings based on the given exponent.
///
/// # Examples
///
/// ```
/// # use fixed_num_helper::*;
/// fn test(inp: (&str, &str, i128), out: (&str, &str)) {
///     assert_eq!(shift_decimal(inp.0, inp.1, inp.2), (out.0.to_string(), out.1.to_string()));
/// }
/// test(("123", "456", -5), ("0", "00123456"));
/// test(("123", "456", -4), ("0", "0123456"));
/// test(("123", "456", -3), ("0", "123456"));
/// test(("123", "456", -2), ("1", "23456"));
/// test(("123", "456", -1), ("12", "3456"));
/// test(("123", "456",  0), ("123", "456"));
/// test(("123", "456",  1), ("1234", "56"));
/// test(("123", "456",  2), ("12345", "6"));
/// test(("123", "456",  3), ("123456", "0"));
/// test(("123", "456",  4), ("1234560", "0"));
/// test(("123", "456",  5), ("12345600", "0"));
///
/// test(("100", "",  -1), ("10", "0"));
/// test(("100", "",  -2), ("1", "0"));
/// test(("100", "",  -3), ("0", "1"));
/// test(("100", "",  -4), ("0", "01"));
///
/// test(("", "001",  1), ("0", "01"));
/// test(("", "001",  2), ("0", "1"));
/// test(("", "001",  3), ("1", "0"));
/// test(("", "001",  4), ("10", "0"));
/// ```
pub fn shift_decimal(
    int_part: &str,
    frac_part: &str,
    exp: i128,
) -> (String, String) {
    let mut int_part = int_part.to_string();
    let mut frac_part = frac_part.to_string();

    #[expect(clippy::comparison_chain)]
    if exp > 0 {
        let exp = exp as usize;
        let move_count = exp.min(frac_part.len());
        int_part.push_str(&frac_part[..move_count]);
        frac_part = frac_part[move_count..].to_string();
        if exp > move_count {
            int_part.push_str(&"0".repeat(exp - move_count));
        }
    } else if exp < 0 {
        let exp = (-exp) as usize;
        let move_count = exp.min(int_part.len());
        let moved = &int_part[int_part.len() - move_count..];
        frac_part = format!("{moved}{frac_part}");
        int_part.truncate(int_part.len() - move_count);
        if exp > move_count {
            frac_part = format!("{}{frac_part}", "0".repeat(exp - move_count));
        }
    }

    let mut int_part = int_part.trim_start_matches('0').to_string();
    let mut frac_part = frac_part.trim_end_matches('0').to_string();

    if int_part.is_empty() {
        int_part = "0".to_string();
    }
    if frac_part.is_empty() {
        frac_part = "0".to_string();
    }

    (int_part, frac_part)
}

pub fn parse_dec19x19_internal(s: &str) -> Result<i128, ParseDec19x19Error> {
    // let debug_pfx = "debug";
    // let (s, debug) = if s.starts_with(debug_pfx) {
    //     (&s[debug_pfx.len()..], true)
    // } else {
    //     (s, false)
    // };
    let clean = s.replace(['_', ' '], "");
    let trimmed = clean.trim();
    let is_negative = trimmed.starts_with('-');
    let e_parts: Vec<&str> = trimmed.split('e').collect();
    if e_parts.len() > 2 {
        let pos = e_parts[0].len() + e_parts[1].len() + 1;
        return Err(ParseDec19x19Error::InvalidChar { char: 'e', pos })
    }
    let exp: i128 = e_parts.get(1).map_or(Ok(0), |t| t.parse())?;
    let parts: Vec<&str> = e_parts[0].split('.').collect();
    let parts_count = parts.len();
    if parts_count > 2 {
        let pos = parts[0].len() + parts[1].len() + 1;
        return Err(ParseDec19x19Error::InvalidChar { char: '.', pos })
    }
    let int_part_str = parts[0].to_string();
    let frac_part_str = parts.get(1).map(|t| t.to_string()).unwrap_or_default();
    let (int_part_str2, frac_part_str2) = shift_decimal(&int_part_str, &frac_part_str, exp);
    let int_part: i128 = int_part_str2.parse()?;
    let frac_part: i128 = {
        if frac_part_str2.len() > FRAC_PLACES as usize {
            return Err(ParseDec19x19Error::TooPrecise);
        }
        let mut buffer = [b'0'; FRAC_PLACES as usize];
        let frac_bytes = frac_part_str2.as_bytes();
        buffer[..frac_bytes.len()].copy_from_slice(frac_bytes);
        #[allow(clippy::unwrap_used)]
        let padded = std::str::from_utf8(&buffer).unwrap();
        padded.parse()?
    };
    let scaled = int_part.checked_mul(FRAC_SCALE_I128).ok_or(ParseDec19x19Error::OutOfBounds)?;
    let repr = if is_negative {
        scaled.checked_sub(frac_part)
    } else {
        scaled.checked_add(frac_part)
    }.ok_or(ParseDec19x19Error::OutOfBounds)?;
    Ok(repr)
}

const SCALE: i128 = 10_000_000_000_000_000_000; // 10^19

pub fn parse_dec19x19_internal2(s: &str) -> Result<i128, ParseDec19x19Error> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len == 0 {
        return Err(ParseDec19x19Error::Empty);
    }

    let mut i = 0;

    // Sign
    let mut neg = false;
    match bytes[i] {
        b'+' => i += 1,
        b'-' => {
            neg = true;
            i += 1;
        }
        _ => {}
    }

    let mut digits: i128 = 0;
    let mut tail_zero_count: i32 = 0;
    let mut digit_count: u32 = 0;
    let mut frac_digit_count: i32 = 0;
    let mut seen_digit = false;
    let mut seen_dot = false;
    let mut seen_e = false;

    // Main number
    while i < len {
        let b = bytes[i];
        match b {
            b'0' => {
                seen_digit = true;
                tail_zero_count += 1;
                let d = (b - b'0') as i128;
                digit_count += 1;
            }
            b'1'..=b'9' => {
                seen_digit = true;
                let d = (b - b'0') as i128;
                for _ in 0 ..= tail_zero_count {
                    digits *= 10;
                }
                digits += d;
                digit_count += 1;
                if seen_dot {
                    frac_digit_count += 1 + tail_zero_count;
                }
                tail_zero_count = 0;
            }
            b'_' => {}
            b'.' if !seen_dot => {
                seen_dot = true;
            }
            b'e' | b'E' => {
                seen_e = true;
                break
            },
            _ => return Err(ParseDec19x19Error::InvalidChar { char: char::from(b), pos: i }),
        }
        i += 1;
    }

    if seen_dot {
        tail_zero_count = 0
    }

    // if !seen_digit {
    //     return Err(ParseDecError::Invalid);
    // }

    // Exponent
    let mut exp: i32 = 0;
    if seen_e && i < len {
        i += 1; // skip e/E
        if i == len {
            return Err(ParseDec19x19Error::OutOfBounds);
        }

        let mut exp_neg = false;
        match bytes[i] {
            b'+' => i += 1,
            b'-' => {
                exp_neg = true;
                i += 1;
            }
            _ => {}
        }

        let mut seen_exp_digit = false;
        while i < len {
            let b = bytes[i];
            match b {
                b'0'..=b'9' => {
                    seen_exp_digit = true;
                    exp *= 10;
                    exp += (b - b'0') as i32;
                }
                b'_' => {}
                _ => return Err(ParseDec19x19Error::OutOfBounds),
            }
            i += 1;
        }

        if !seen_exp_digit {
            return Err(ParseDec19x19Error::OutOfBounds);
        }

        if exp_neg {
            exp = -exp;
        }
    }

    // Apply exponent
    dbg!(exp);
    dbg!(digits);
    dbg!(tail_zero_count);

    let exp2 = exp + 19 - frac_digit_count + tail_zero_count;
    dbg!(exp2);
    // let int_scaled = int.checked_mul(10_i128.pow((19 + exp) as u32)).ok_or(ParseDec19x19Error::OutOfBounds)?;
    // let frac_scaled = frac.checked_mul(10_i128.pow((19 - frac_digit_count as i32 + exp) as u32)).ok_or(ParseDec19x19Error::OutOfBounds)?;
    // dbg!(int_scaled, frac_scaled);
    if exp2 < 0 {
        return Err(ParseDec19x19Error::TooPrecise);
    }
    let mut value = digits.checked_mul(10_i128.pow(exp2 as u32)).ok_or(ParseDec19x19Error::OutOfBounds)?;
    // let mut value = int
    //     .checked_mul(SCALE)
    //     .and_then(|v| v.checked_add(frac))
    //     .ok_or(ParseDec19x19Error::OutOfBounds)?;

    dbg!(value);
    // if scale_exp > 0 {
    //     let mut k = scale_exp;
    //     while k > 0 {
    //         value = value
    //             .checked_mul(10)
    //             .ok_or(ParseDec19x19Error::OutOfBounds)?;
    //         k -= 1;
    //     }
    // } else if scale_exp < 0 {
    //     let mut k = -scale_exp;
    //     while k > 0 {
    //         value /= 10;
    //         k -= 1;
    //     }
    // }

    if neg {
        value = value.checked_neg().ok_or(ParseDec19x19Error::OutOfBounds)?;
    }

    Ok(value)
}

// ====================
// === FmtSeparated ===
// ====================

#[derive(Debug, Clone, Copy)]
pub struct Formatter {
    pub separator: Option<char>,
    pub precision: Option<usize>,
    pub width: Option<usize>,
    pub align: Option<std::fmt::Alignment>,
    pub fill: char,
    pub sign_plus: bool
}

pub trait Format {
    fn format(&self, f: &mut Formatter) -> String;
}

// ============
// === Rand ===
// ============

pub trait Rand {
    fn rand(seed: u64, int: impl IntoRandRange, frac: impl IntoRandRange) -> Self;
}

pub type RandRange = std::ops::RangeInclusive<u32>;

pub trait IntoRandRange {
    fn into_rand_range(self) -> RandRange;
}

impl IntoRandRange for RandRange {
    fn into_rand_range(self) -> RandRange {
        self
    }
}

impl IntoRandRange for u32 {
    fn into_rand_range(self) -> RandRange {
        self ..= self
    }
}
