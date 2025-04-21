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
