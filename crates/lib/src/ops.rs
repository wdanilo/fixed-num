pub use std::ops::Add;
pub use std::ops::Sub;
pub use std::ops::Mul;
pub use std::ops::Div;
pub use std::ops::Rem;
pub use std::ops::AddAssign;
pub use std::ops::SubAssign;
pub use std::ops::MulAssign;
pub use std::ops::DivAssign;
pub use std::ops::Neg;

// ====================
// === const_trait! ===
// ====================

#[cfg(nightly)]
macro_rules! const_trait {
    ($(#[$meta:meta])* $vis:vis trait $($rest:tt)*) => {
        $(#[$meta])* $vis const trait $($rest)*
    };
}

#[cfg(not(nightly))]
macro_rules! const_trait {
    ($(#[$meta:meta])* $vis:vis trait $($rest:tt)*) => {
        $(#[$meta])* $vis trait $($rest)*
    };
}

// ==============
// === Traits ===
// ==============

pub mod traits {
    pub use std::ops::Add as _;
    pub use std::ops::Sub as _;
    pub use std::ops::Mul as _;
    pub use std::ops::Div as _;
    pub use std::ops::Rem as _;
    pub use std::ops::AddAssign as _;
    pub use std::ops::SubAssign as _;
    pub use std::ops::MulAssign as _;
    pub use std::ops::DivAssign as _;
    pub use std::ops::Neg as _;
    pub use super::HasMax as _;
    pub use super::HasMin as _;
    pub use super::Signum as _;
    pub use super::Abs as _;
    pub use super::UncheckedAdd as _;
    pub use super::CheckedAdd as _;
    pub use super::SaturatingAdd as _;
    pub use super::UncheckedSub as _;
    pub use super::CheckedSub as _;
    pub use super::SaturatingSub as _;
    pub use super::UncheckedMul as _;
    pub use super::CheckedMul as _;
    pub use super::SaturatingMul as _;
    pub use super::UncheckedDiv as _;
    pub use super::CheckedDiv as _;
    pub use super::SaturatingDiv as _;
    pub use super::Trunc as _;
    pub use super::TruncTo as _;
    pub use super::Floor as _;
    pub use super::FloorTo as _;
    pub use super::Ceil as _;
    pub use super::CeilTo as _;
    pub use super::Round as _;
    pub use super::RoundTo as _;
    pub use super::UncheckedSqrt as _;
    pub use super::CheckedSqrt as _;
    pub use super::UncheckedPow as _;
    pub use super::CheckedPow as _;
    pub use super::UncheckedLog10Floor as _;
    pub use super::CheckedLog10Floor as _;
    pub use super::UncheckedLn as _;
    pub use super::CheckedLn as _;
}

// ==============
// === HasMax ===
// ==============

const_trait! {
    /// ✅ Checks if `self` is the maximum value.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait HasMax: Sized {
        const MAX: Self;
        #[allow(clippy::wrong_self_convention)]
        fn is_max(self) -> bool;
    }
}

// ==============
// === HasMin ===
// ==============

const_trait! {
    /// ✅ Checks if `self` is the minimum value.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait HasMin: Sized {
        const MIN: Self;
        #[allow(clippy::wrong_self_convention)]
        fn is_min(self) -> bool;
    }
}

// ==============
// === Signum ===
// ==============

const_trait! {
    /// ✅ The sign of the number.
    ///
    /// Returns:
    /// - `1.0` if positive,
    /// - `0.0` if zero,
    /// - `-1.0` if negative.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait Signum {
        fn signum(self) -> Self;
        fn signum_i128(self) -> i128;
    }
}

// ===========
// === Abs ===
// ===========

const_trait! {
    /// ✅ The absolute value of `self`.
    ///
    /// # Panics
    ///
    /// This function never panics. If the value is the minimum representable number, it returns the
    /// nearest valid value (e.g. `Self::MAX`).
    pub trait Abs {
        fn abs(self) -> Self;
    }
}

// ===========
// === Add ===
// ===========

const_trait! {
    /// Addition without checking for overflow.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows.
    pub trait UncheckedAdd<Rhs = Self> {
        type Output;
        fn unchecked_add(self, rhs: Rhs) -> Self::Output;
    }
}

const_trait! {
    /// ✅ Checked addition. Returns `None` if the result overflows.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedAdd<Rhs = Self> {
        type Output;
        fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
    }
}

const_trait! {
    /// ✅ Saturating addition. Clamps the result on overflow.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait SaturatingAdd<Rhs = Self> {
        type Output;
        fn saturating_add(self, rhs: Rhs) -> Self::Output;
    }
}

// ===========
// === Sub ===
// ===========

const_trait! {
    /// Subtraction without checking for overflow.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows.
    pub trait UncheckedSub<Rhs = Self> {
        type Output;
        fn unchecked_sub(self, rhs: Rhs) -> Self::Output;
    }
}

const_trait! {
    /// ✅ Checked subtraction. Returns `None` if the result overflows.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedSub<Rhs = Self> {
        type Output;
        fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
    }
}

const_trait! {
    /// ✅ Saturating subtraction. Clamps the result on overflow.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait SaturatingSub<Rhs = Self> {
        type Output;
        fn saturating_sub(self, rhs: Rhs) -> Self::Output;
    }
}

// ===========
// === Mul ===
// ===========

const_trait! {
    /// Multiplication without checking for overflow.
    ///
    /// # Panics
    ///
    /// Panics if the result overflows.
    pub trait UncheckedMul<Rhs = Self> {
        type Output;
        fn unchecked_mul(self, rhs: Rhs) -> Self::Output;
    }
}

const_trait! {
    /// ✅ Checked multiplication. Returns `None` if the result overflows.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedMul<Rhs = Self> {
        type Output;
        fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;
    }
}

const_trait! {
    /// ✅ Saturating multiplication. Clamps the result on overflow.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait SaturatingMul<Rhs = Self> {
        type Output;
        fn saturating_mul(self, rhs: Rhs) -> Self::Output;
    }
}

// ===========
// === Div ===
// ===========

const_trait! {
    /// Division without checking for division by zero or overflow.
    ///
    /// # Panics
    ///
    /// Panics if dividing by zero or if the result overflows.
    pub trait UncheckedDiv<Rhs = Self> {
        type Output;
        fn unchecked_div(self, rhs: Rhs) -> Self::Output;
    }
}

const_trait! {
    /// ✅ Checked division. Returns `None` on division by zero or overflow.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedDiv<Rhs = Self> {
        type Output;
        fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
    }
}

const_trait! {
    /// ✅ Saturating division. Returns `Self::MAX` or `Self::MIN` if division by zero or overflow
    /// occurs.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait SaturatingDiv<Rhs = Self> {
        type Output;
        fn saturating_div(self, rhs: Rhs) -> Self::Output;
    }
}

// =============
// === Trunc ===
// =============

const_trait! {
    /// ✅ Truncates fractional digits, rounding toward zero.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait Trunc {
        fn trunc(self) -> Self;
    }
}

const_trait! {
    /// ✅ Truncates to the specified number of fractional digits.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait TruncTo {
        fn trunc_to(self, digits: i64) -> Self;
    }
}

// =============
// === Floor ===
// =============

const_trait! {
    /// ✅ Rounds the number toward negative infinity if the result is representable. If rounding would
    /// cause an overflow, returns the original value unchanged.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait Floor {
        fn floor(self) -> Self;
    }
}

const_trait! {
    /// ✅ Rounds the number toward negative infinity to the specified number of fractional digits. If
    /// rounding would cause an overflow, returns the original value unchanged.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait FloorTo {
        fn floor_to(self, digits: i64) -> Self;
    }
}

// ============
// === Ceil ===
// ============

const_trait! {
    /// ✅ Rounds the number toward positive infinity if the result is representable. If rounding would
    /// cause an overflow, returns the original value unchanged.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait Ceil {
        fn ceil(self) -> Self;
    }
}

const_trait! {
    /// ✅ Rounds the number toward positive infinity to the specified number of fractional digits. If
    /// rounding would cause an overflow, returns the original value unchanged.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CeilTo {
        fn ceil_to(self, digits: i64) -> Self;
    }
}

// =============
// === Round ===
// =============

const_trait! {
    /// ✅ Rounds the number to the nearest integer, away from zero on tie. If rounding would cause an
    /// overflow, returns the nearest representable result instead.
    ///
    /// # Examples
    ///
    /// - `...123.4` -> `...123`
    /// - `...123.5` -> `...124`
    /// - `...123.6` -> `...124`
    /// - `...123.6` -> `...123` if `...124` is not representable.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait Round {
        fn round(self) -> Self;
    }
}

const_trait! {
    /// ✅ Rounds the number to the nearest value with the specified number of fractional digits, away
    /// from zero on tie. If rounding would cause an overflow, returns the closest representable result
    /// instead.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait RoundTo {
        fn round_to(self, digits: i64) -> Self;
    }
}

// ============
// === Sqrt ===
// ============

const_trait! {
    /// Returns the square root of `self` without checking the input.
    ///
    /// # Panics
    ///
    /// Panics if `self` is negative.
    pub trait UncheckedSqrt {
        fn unchecked_sqrt(self) -> Self;
    }
}

const_trait! {
    /// ✅ Returns the square root of `self`, or `None` if `self` is negative.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedSqrt: Sized {
        fn checked_sqrt(self) -> Option<Self>;
    }
}

// ===========
// === Pow ===
// ===========

const_trait! {
    /// Raises `self` to the power of `exp` without checking for overflow or invalid input.
    ///
    /// # Panics
    ///
    /// Panics on overflow or if `exp` is negative and `self` is zero.
    pub trait UncheckedPow<Exp = Self> {
        type Output;
        fn unchecked_pow(self, exp: Exp) -> Self::Output;
    }
}

const_trait! {
    /// ✅ aises `self` to the power of `exp`, returning `None` on overflow or invalid input.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedPow<Rhs = Self> {
        type Output;
        fn checked_pow(self, exp: Rhs) -> Option<Self::Output>;
    }
}

// ==================
// === Log10Floor ===
// ==================

const_trait! {
    /// Returns the base-10 logarithm of `self`, rounded down to the nearest integer.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero or negative.
    pub trait UncheckedLog10Floor {
        fn unchecked_log10_floor(self) -> Self;
    }
}

const_trait! {
    /// ✅ Returns the base-10 logarithm of `self`, rounded down to the nearest integer,
    /// or `None` if `self` is zero or negative.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedLog10Floor: Sized {
        fn checked_log10_floor(self) -> Option<Self>;
    }
}

// ==========
// === Ln ===
// ==========

const_trait! {
    /// Returns the natural logarithm of `self`.
    ///
    /// # Panics
    ///
    /// Panics if `self` is zero or negative.
    pub trait UncheckedLn {
        fn unchecked_ln(self) -> Self;
    }
}

const_trait! {
    /// ✅ Returns the natural logarithm of `self`, or `None` if `self` is zero or negative.
    ///
    /// # Panics
    ///
    /// This function never panics.
    pub trait CheckedLn: Sized {
        fn checked_ln(self) -> Option<Self>;
    }
}
