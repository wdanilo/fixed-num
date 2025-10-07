use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use paste::paste;
use std::str::FromStr;
use fixed_num_helper::*;
use crate::ops::*;

pub use fixed_num_macro::*;

// ============
// === i256 ===
// ============
// The ethnum implementation is a bit faster, but it contained some serious bugs that we reported
// during development and they got fixed. On the other hand, the arrow implementation did not have
// any obvious bugs.

#[cfg(feature="i256_arrow")]
use arrow_buffer::i256;

#[cfg(feature="i256_arrow")]
#[inline(always)]
const fn i256_from_i128(val: i128) -> i256 {
    i256::from_i128(val)
}

#[cfg(feature="i256_arrow")]
#[inline(always)]
fn i256_to_i128(val: i256) -> Option<i128> {
    i256::to_i128(val)
}

#[cfg(feature="i256_ethnum")]
use ethnum::I256 as i256;

#[cfg(feature="i256_ethnum")]
#[inline(always)]
const fn i256_from_i128(val: i128) -> i256 {
    i256::new(val)
}

#[cfg(feature="i256_ethnum")]
#[inline(always)]
fn i256_to_i128(val: i256) -> Option<i128> {
    i128::try_from(val).ok()
}

// =================
// === Constants ===
// =================

const FRAC_SCALE_U128: u128 = FRAC_SCALE_I128 as u128;
const FRAC_SCALE_F64: f64 = FRAC_SCALE_I128 as f64;
const FRAC_SCALE_I256: i256 = i256_from_i128(FRAC_SCALE_I128);
const FRAC_SCALE_I128_HALF: i128 = FRAC_SCALE_I128 / 2;
const I256_TWO: i256 = i256_from_i128(2);
const LN_2_I256: i256 = i256_from_i128(Dec19x19::LN_2.repr);

// ================
// === Dec19x19 ===
// ================

/// A high-precision, high-performance fixed-point decimal type.
///
/// Internally, values are stored as `i128`, which supports 39 digits with the first digit never
/// exceeding `1`. The last 19 digits are interpreted as the fractional part. This allows all
/// operations to perform without rounding or approximations within the full range of exactly 19
/// fractional and 19 integer digits.
#[repr(transparent)]
pub struct Dec19x19 {
    pub repr: i128,
}

impl Dec19x19 {
    /// Creates a new `Dec19x19` from the given `i128` representation, assuming the last 19 digits
    /// are the fractional part.
    #[inline(always)]
    pub const fn from_repr(repr: i128) -> Self {
        Self { repr }
    }

    #[inline(always)]
    pub const fn is_zero(self) -> bool {
        self.repr == 0
    }
}

// =================
// === Std Impls ===
// =================
// Implemented manually to mark all methods as inline.

impl Copy for Dec19x19 {}
impl Clone for Dec19x19 {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for Dec19x19 {
    #[inline(always)]
    fn default() -> Self {
        Self { repr: 0 }
    }
}

impl Eq for Dec19x19 {}
impl PartialEq for Dec19x19 {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.repr == other.repr
    }
}

impl Ord for Dec19x19 {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.repr.cmp(&other.repr)
    }
}

impl PartialOrd for Dec19x19 {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(nightly)]
impl std::iter::Step for Dec19x19 {
    #[inline(always)]
    fn forward(start: Self, count: usize) -> Self {
        Self::from_repr(<i128 as std::iter::Step>::forward(start.repr, count))
    }

    #[inline(always)]
    fn backward(start: Self, count: usize) -> Self {
        Self::from_repr(<i128 as std::iter::Step>::backward(start.repr, count))
    }

    #[inline(always)]
    unsafe fn forward_unchecked(start: Self, count: usize) -> Self {
        unsafe {
            Self::from_repr(<i128 as std::iter::Step>::forward_unchecked(start.repr, count))
        }
    }

    #[inline(always)]
    unsafe fn backward_unchecked(start: Self, count: usize) -> Self {
        unsafe {
            Self::from_repr(<i128 as std::iter::Step>::backward_unchecked(start.repr, count))
        }
    }

    #[inline(always)]
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        <i128 as std::iter::Step>::steps_between(&start.repr, &end.repr)
    }

    #[inline(always)]
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        <i128 as std::iter::Step>::forward_checked(start.repr, count).map(Self::from_repr)
    }

    #[inline(always)]
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        <i128 as std::iter::Step>::backward_checked(start.repr, count).map(Self::from_repr)
    }
}

// ==========================
// === Dec19x19 Constants ===
// ==========================

impl Dec19x19 {
    /// The biggest possible integer value that can be stored in a `Dec19x19`.
    ///
    /// # Tests
    ///
    /// ```
    /// # use fixed_num::*;
    /// assert_eq!(Dec19x19::MAX_INT, Dec19x19::MAX.trunc());
    /// ```
    pub const MAX_INT: Self = Dec19x19!(17_014_118_346_046_923_173);

    /// The smallest possible integer value that can be stored in a `Dec19x19`.
    ///
    /// # Tests
    ///
    /// ```
    /// # use fixed_num::*;
    /// assert_eq!(Dec19x19::MIN_INT, Dec19x19::MIN.trunc());
    /// ```
    pub const MIN_INT: Self = Dec19x19!(-17_014_118_346_046_923_173);

    /// The natural logarithm of 2 (`ln(2)`), accurate to all 19 decimal places of the `Dec19x19`
    /// fixed-point format.
    pub const LN_2: Self = Dec19x19!(0.693_147_180_559_945_309_4);

    /// The smallest possible value that can be stored in a `Dec19x19`.
    ///
    /// # Tests
    ///
    /// ```
    /// # use fixed_num::*;
    /// assert_eq!(Dec19x19::SMALLEST_STEP / Dec19x19!(2), Dec19x19!(0));
    /// ```
    pub const SMALLEST_STEP: Self = Dec19x19!(0.000_000_000_000_000_000_1);
}

// ==============
// === Random ===
// ==============

/// Generates a deterministic random `Dec19x19` value using a seed, an integer precision, and a
/// fractional precision. Never returns zero.
///
/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::rand] {
///     (0,  6, 0) => Dec19x19!(-758_415),
///     (1,  6, 0) => Dec19x19!(-717_558),
///     (2,  6, 0) => Dec19x19!(-149_577),
///     (3,  6, 0) => Dec19x19!(-442_649),
///     (4,  6, 0) => Dec19x19!( 658_419),
///     (5,  6, 0) => Dec19x19!( 165_296),
///
///     (6,  3, 0) => Dec19x19!(-787),
///     (7,  3, 0) => Dec19x19!(-354),
///     (8,  3, 0) => Dec19x19!( 745),
///     (9,  3, 0) => Dec19x19!( 163),
///     (10, 3, 0) => Dec19x19!(-211),
///     (11, 3, 0) => Dec19x19!(-719),
///
///     (12, 3, 3) => Dec19x19!(-698.488),
///     (13, 3, 3) => Dec19x19!( 354.710),
///     (14, 3, 3) => Dec19x19!( 807.648),
///     (15, 3, 3) => Dec19x19!(-392.145),
///     (16, 3, 3) => Dec19x19!(-243.552),
///     (17, 3, 3) => Dec19x19!( 378.313),
///
///     (18, 6, 6) => Dec19x19!( 428_879.493_071),
///     (19, 6, 6) => Dec19x19!( 414_719.622_665),
///     (20, 6, 6) => Dec19x19!( 154_184.335_022),
///     (21, 6, 6) => Dec19x19!( 335_592.781_210),
///     (22, 6, 6) => Dec19x19!(-562_472.732_119),
///     (23, 6, 6) => Dec19x19!(-990_435.673_210),
///
///     (0, 0, 6) => Dec19x19!(-0.758_415),
///     (1, 0, 6) => Dec19x19!(-0.617_558),
///     (2, 0, 6) => Dec19x19!(-0.049_577),
///     (3, 0, 6) => Dec19x19!(-0.342_649),
///     (4, 0, 6) => Dec19x19!( 0.658_419),
///     (5, 0, 6) => Dec19x19!( 0.065_296),
///
///     (1, 19, 19) => Dec19x19!(-7_175_586_050_193_843_404.647_199_936_274_331_797_4),
///
///     (0, 0, 0) => Dec19x19!(-7),
///     (1, 0, 0) => Dec19x19!(-6),
///     (2, 0, 0) => Dec19x19!(-1),
///     (3, 0, 0) => Dec19x19!(-3),
///     (4, 0, 0) => Dec19x19!(6),
///     (5, 0, 0) => Dec19x19!(1),
///
///     (1, 0..=9, 0..=9) => Dec19x19!(42545517.614973869),
///     (2, 0..=9, 0..=9) => Dec19x19!(-0.41),
///     (3, 0..=9, 0..=9) => Dec19x19!(-224053),
///     (4, 0..=9, 0..=9) => Dec19x19!(662259.83081),
///     (5, 0..=9, 0..=9) => Dec19x19!(-5.748),
/// });
/// ```
impl Rand for Dec19x19 {
    fn rand(seed: u64, int: impl IntoRandRange, frac: impl IntoRandRange) -> Self {
        let int_prec_range = int.into_rand_range();
        let frac_prec_range = frac.into_rand_range();
        assert!(*int_prec_range.end() <= 19);
        assert!(*frac_prec_range.end() <= 19);
        let mut rng = StdRng::seed_from_u64(seed);
        let int_prec = if int_prec_range.start() == int_prec_range.end() {
            *int_prec_range.start()
        } else {
            rng.random_range(int_prec_range)
        };
        let frac_prec = if frac_prec_range.start() == frac_prec_range.end() {
            *frac_prec_range.start()
        } else {
            rng.random_range(frac_prec_range)
        };

        let digit_count = (int_prec + frac_prec).max(1);
        let scale = 10_i128.pow(digit_count - 1);
        let max_val = scale - 1;
        let first_digit_start = if int_prec > 0 { 1 } else { 0 };
        let first_digit = rng.random_range(first_digit_start..=9);
        let mut val = first_digit * scale + rng.random_range(0..=max_val);
        if val == 0 {
            val = 1;
        }

        val *= 10_i128.pow(19 - frac_prec);
        if rng.random_bool(0.5) {
            val = -val;
        }
        Self::from_repr(val)
    }
}

// ====================
// === Impl Helpers ===
// ====================

macro_rules! impl_op_for_refs {
    ($op:ident :: $f:ident) => {
        impl<'t> $op<&'t Dec19x19> for &'t Dec19x19 {
            type Output = Dec19x19;
            #[inline(always)]
            fn $f(self, rhs: Self) -> Self::Output {
                $op::<Dec19x19>::$f(*self, *rhs)
            }
        }

        impl<'t> $op<&'t Dec19x19> for Dec19x19 {
            type Output = Dec19x19;
            #[inline(always)]
            fn $f(self, rhs: &'t Dec19x19) -> Self::Output {
                $op::<Dec19x19>::$f(self, *rhs)
            }
        }

        impl $op<Dec19x19> for &Dec19x19 {
            type Output = Dec19x19;
            #[inline(always)]
            fn $f(self, rhs: Dec19x19) -> Self::Output {
                $op::<Dec19x19>::$f(*self, rhs)
            }
        }
    };
}

#[cfg(nightly)]
macro_rules! const_impl {
    ($(#$meta:tt)* impl $($ts:tt)*) => {
        $(#$meta)*
        impl const $($ts)*
    };
}

#[cfg(not(nightly))]
macro_rules! const_impl {
    ($(#$meta:tt)* impl $($ts:tt)*) => {
        $(#$meta)*
        impl $($ts)*
    };
}

// =================
// === Max / Min ===
// =================

const_impl!{
/// The biggest possible value that can be stored in a `Dec19x19`, equal to
/// ```
/// # assert_eq!(fixed_num::Dec19x19!(
/// 17_014_118_346_046_923_173.168_730_371_588_410_572_7
/// # ).repr, i128::MAX);
/// ```
impl HasMax for Dec19x19 {
    const MAX: Self = Self { repr: i128::MAX };
    fn is_max(self) -> bool {
        self.repr == i128::MAX
    }
}}

const_impl!{
/// The smallest possible value that can be stored in a `Dec19x19`, equal to
/// ```
/// # assert_eq!(fixed_num::Dec19x19!(
/// -17_014_118_346_046_923_173.168_730_371_588_410_572_8
/// # ).repr, i128::MIN);
/// ```
impl HasMin for Dec19x19 {
    const MIN: Self = Self { repr: i128::MIN };
    fn is_min(self) -> bool {
        self.repr == i128::MIN
    }
}}

// ==============
// === Signum ===
// ==============

const_impl!{
/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::signum] {
///     (Dec19x19::MAX)   => Dec19x19!(1.0),
///     (Dec19x19!(3.0))  => Dec19x19!(1.0),
///     (Dec19x19!(0.0))  => Dec19x19!(0.0),
///     (Dec19x19!(-3.0)) => Dec19x19!(-1.0),
///     (Dec19x19::MIN)   => Dec19x19!(-1.0),
/// });
/// ```
impl Signum for Dec19x19 {
    #[inline(always)]
    fn signum(self) -> Self {
        Self { repr: self.signum_i128() * FRAC_SCALE_I128 }
    }

    #[inline(always)]
    fn signum_i128(self) -> i128 {
        self.repr.signum()
    }
}}

// ===========
// === Neg ===
// ===========

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::neg] {
///     (Dec19x19::MAX)   => Dec19x19::MIN + Dec19x19::SMALLEST_STEP,
///     (Dec19x19!(3.0))  => Dec19x19!(-3.0),
///     (Dec19x19!(0.0))  => Dec19x19!(0.0),
///     (Dec19x19!(-3.0)) => Dec19x19!(3.0),
///     (Dec19x19::MIN)   => Dec19x19::MAX,
///     ((Dec19x19::MIN + Dec19x19::SMALLEST_STEP)) => Dec19x19::MAX,
/// });
/// ```
impl Neg for Dec19x19 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        if self == Self::MIN {
            Self::MAX
        } else {
            Self::from_repr(-self.repr)
        }
    }
}

// ===========
// === Abs ===
// ===========

const_impl!{
/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check!( [Dec19x19::abs] {
///     (Dec19x19::MAX)   => Dec19x19::MAX,
///     (Dec19x19!(3.0))  => Dec19x19!(3.0),
///     (Dec19x19!(0.0))  => Dec19x19!(0.0),
///     (Dec19x19!(-3.0)) => Dec19x19!(3.0),
///     (Dec19x19::MIN)   => Dec19x19::MAX,
/// });
/// ```
impl Abs for Dec19x19 {
    #[inline(always)]
    fn abs(self) -> Self {
        if self.is_min() {
            return Self::MAX;
        }
        Self { repr: self.repr.abs() }
    }
}}

// ===========
// === Rem ===
// ===========

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check!( [Dec19x19::rem] {
///     (Dec19x19!(14.7), Dec19x19!(5))             => Dec19x19!(4.7),
///     (Dec19x19!(14.7), Dec19x19!(0))             => Dec19x19!(14.7),
///     (Dec19x19!(14.7), Dec19x19::SMALLEST_STEP)  => Dec19x19!(0),
///     (Dec19x19::MAX,   Dec19x19::SMALLEST_STEP)  => Dec19x19!(0),
///     (Dec19x19::MIN,   -Dec19x19::SMALLEST_STEP) => Dec19x19!(0),
///     (Dec19x19::MAX,   Dec19x19::MAX)            => Dec19x19!(0),
///     (Dec19x19::MIN,   Dec19x19::MIN)            => Dec19x19!(0),
///     (Dec19x19::MAX,   Dec19x19::MIN)            => Dec19x19::MAX,
///     (Dec19x19::MIN,   Dec19x19::MAX)            => -Dec19x19::SMALLEST_STEP,
/// });
/// ```
impl Rem for Dec19x19 {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.repr == 0 {
            self
        } else if self == Self::MIN && rhs == -Self::SMALLEST_STEP {
            Dec19x19!(0)
        } else {
            Self { repr: self.repr % rhs.repr }
        }
    }
}

// ===========
// === Add ===
// ===========

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check!( [Dec19x19::add, Dec19x19::checked_add] {
///     (Dec19x19::MAX, Dec19x19::MIN) => -Dec19x19::SMALLEST_STEP,
///     (Dec19x19::MAX - Dec19x19!(1), Dec19x19!(1)) => Dec19x19::MAX,
///     (Dec19x19::MAX, Dec19x19!(0)) => Dec19x19::MAX,
///     (Dec19x19::MIN, Dec19x19!(0)) => Dec19x19::MIN,
///     (Dec19x19::MAX, Dec19x19::SMALLEST_STEP) => FAIL,
///     (Dec19x19::MAX, Dec19x19!(1)) => FAIL,
///     (Dec19x19::MIN, -Dec19x19::SMALLEST_STEP) => FAIL
/// });
/// ```
///
/// # Fuzzy
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// fuzzy2::<Dec19x19, BigDecimal>(Series::new(0..=18, 0..=19), Series::new(0..=18, 0..=19),
///     |(f1, b1), (f2, b2)| should_eq(f1 + f2, b1 + b2)
/// );
/// ```
impl Add for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self.unchecked_add(rhs)
    }
}

const_impl!{ impl UncheckedAdd for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn unchecked_add(self, rhs: Self) -> Self {
        Self::from_repr(self.repr + rhs.repr)
    }
}}

const_impl!{ impl CheckedAdd for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn checked_add(self, rhs: Self) -> Option<Self> {
        if let Some(result) = self.repr.checked_add(rhs.repr) {
            Some(Self::from_repr(result))
        } else {
            None
        }
    }
}}

const_impl!{
/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::saturating_add] {
///     (Dec19x19::MAX,  Dec19x19::SMALLEST_STEP) => Dec19x19::MAX,
///     (Dec19x19::MIN, -Dec19x19::SMALLEST_STEP) => Dec19x19::MIN,
/// });
/// ```
impl SaturatingAdd for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn saturating_add(self, rhs: Self) -> Self {
        if let Some(result) = self.checked_add(rhs) {
            result
        } else if self.signum_i128() >= 0 {
            Self::MAX
        } else {
            Self::MIN
        }
    }
}}

impl AddAssign for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl_op_for_refs!(Add::add);

// ===========
// === Sub ===
// ===========

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check!( [Dec19x19::sub, Dec19x19::checked_sub] {
///     (Dec19x19::MIN + Dec19x19!(1), Dec19x19!(1)) => Dec19x19::MIN,
///     (-Dec19x19::SMALLEST_STEP, Dec19x19::MIN) => Dec19x19::MAX,
///     (Dec19x19::MIN, Dec19x19::SMALLEST_STEP) => FAIL,
///     (Dec19x19::MIN, Dec19x19!(1)) => FAIL,
///     (Dec19x19!(0), Dec19x19::MIN) => FAIL,
/// });
/// ```
///
/// # Fuzzy
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// fuzzy2::<Dec19x19, BigDecimal>(Series::new(0..=18, 0..=19), Series::new(0..=18, 0..=19),
///     |(f1, b1), (f2, b2)| should_eq(f1 - f2, b1 - b2)
/// );
/// ```
impl Sub for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        self.unchecked_sub(rhs)
    }
}

const_impl!{ impl UncheckedSub for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn unchecked_sub(self, rhs: Self) -> Self {
        Self::from_repr(self.repr - rhs.repr)
    }
}}

const_impl!{ impl CheckedSub for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        if let Some(result) = self.repr.checked_sub(rhs.repr) {
            Some(Self::from_repr(result))
        } else {
            None
        }
    }
}}

const_impl!{
/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::saturating_sub] {
///     (Dec19x19::MIN, Dec19x19!(1))  => Dec19x19::MIN,
///     (Dec19x19!(10), Dec19x19::MIN) => Dec19x19::MAX,
///     (Dec19x19!(0), Dec19x19::MIN)  => Dec19x19::MAX,
/// });
/// ```
impl SaturatingSub for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn saturating_sub(self, rhs: Self) -> Self {
        if let Some(result) = self.checked_sub(rhs) {
            result
        } else if self.signum_i128() >= 0 {
            Self::MAX
        } else {
            Self::MIN
        }
    }
}}

impl SubAssign for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl_op_for_refs!(Sub::sub);

// ==========
// === Mul ==
// ==========

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check!(
///     [
///         Dec19x19::unchecked_mul_no_opt,
///         Dec19x19::unchecked_mul_opt,
///         Dec19x19::checked_mul_no_opt,
///         Dec19x19::checked_mul_opt,
///     ] {
///     (Dec19x19!(20), Dec19x19!(2.2)) => Dec19x19!(44.0),
///     (Dec19x19::MAX, Dec19x19!(10)) => FAIL,
///     (Dec19x19::MAX - Dec19x19!(10), Dec19x19!(2)) => FAIL,
/// });
/// ```
///
/// # Fuzzy
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// let series = [
///     Series::new(0..=9, 0..=19),
///     Series::new(9, 19),
///     Series::new(0, 19),
///     Series::new(9, 0),
/// ];
/// for s in series {
///     fuzzy2::<Dec19x19, BigDecimal>(s.clone(), s,
///         |(f1, b1), (f2, b2)| {
///             should_eq(f1.unchecked_mul_opt(f2), b1.clone() * b2.clone());
///             should_eq(f1.unchecked_mul_no_opt(f2), b1.clone() * b2.clone());
///             should_eq(f1.checked_mul_opt(f2).unwrap(), b1.clone() * b2.clone());
///             should_eq(f1.checked_mul_no_opt(f2).unwrap(), b1 * b2);
///         }
///     );
/// }
/// ```
impl Mul for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        self.unchecked_mul(rhs)
    }
}

impl Dec19x19 {
    /// Multiplication without checking for overflow and no optimization for LHS or RHS being ints
    /// or fracs only. You probably want to use `Dec19x19::mul` with `mul_opt` flag disabled
    /// instead.
    #[track_caller]
    #[inline(always)]
    pub fn unchecked_mul_no_opt(self, rhs: Self) -> Self {
        // 1) sign & magnitudes
        let neg = (self.repr < 0) ^ (rhs.repr < 0);
        let ua  = self.repr.unsigned_abs();
        let ub  = rhs.repr.unsigned_abs();

        // 2) split into integer/fraction parts
        let ai = ua / FRAC_SCALE_U128;
        let af = ua % FRAC_SCALE_U128;
        let bi = ub / FRAC_SCALE_U128;
        let bf = ub % FRAC_SCALE_U128;

        // 3) 128×128 multiplies
        let int   = ai * bi;
        let cross = ai * bf + bi * af;
        let frac  = af * bf / FRAC_SCALE_U128;

        // 4) reassemble
        let mag = int * FRAC_SCALE_U128 + cross + frac;
        let mut repr: i128 = mag.try_into().expect("Overflow");
        if neg { repr = -repr; }
        Self { repr }
    }

    /// Multiplication without checking for overflow and optimization for LHS or RHS being ints or
    /// fracs only. You probably want to use `Dec19x19::mul` with `mul_opt` flag enabled instead
    /// (default).
    #[track_caller]
    #[inline(always)]
    pub fn unchecked_mul_opt(self, rhs: Self) -> Self {
        // 1) sign & magnitudes
        let neg = (self.repr < 0) ^ (rhs.repr < 0);
        let ua  = self.repr.unsigned_abs();
        let ub  = rhs.repr.unsigned_abs();

        // 2) split into integer/fraction parts
        let bi = ub / FRAC_SCALE_U128;
        let bf = ub % FRAC_SCALE_U128;

        // 3) 28×128 multiplies
        let mag = if bf == 0 {
            ua * bi
        } else if bi == 0 {
            let ai = ua / FRAC_SCALE_U128;
            let af = ua % FRAC_SCALE_U128;
            let cross = ai * bf;
            let frac = af * bf / FRAC_SCALE_U128;
            cross + frac
        } else {
            let ai = ua / FRAC_SCALE_U128;
            let af = ua % FRAC_SCALE_U128;
            let int = ai * bi * FRAC_SCALE_U128;
            if af == 0 {
                let cross = ai * bf;
                int + cross
            } else {
                let cross = ai * bf + bi * af;
                let frac = af * bf / FRAC_SCALE_U128;
                int + cross + frac
            }
        };

        // 4) reassemble
        let mut repr: i128 = mag.try_into().expect("Overflow");
        if neg { repr = -repr; }
        Self { repr }
    }

    /// Multiplication with checking for overflow and no optimization for LHS or RHS being ints. You
    /// probably want to use `Dec19x19::checked_mul` with `mul_opt` flag disabled instead.
    #[track_caller]
    #[inline(always)]
    pub fn checked_mul_no_opt(self, rhs: Self) -> Option<Self> {
        // 1) sign & magnitudes
        let neg = (self.repr < 0) ^ (rhs.repr < 0);
        let ua  = self.repr.unsigned_abs();
        let ub  = rhs.repr.unsigned_abs();

        // 2) split into integer/fraction parts
        let ai = ua / FRAC_SCALE_U128;
        let af = ua % FRAC_SCALE_U128;
        let bi = ub / FRAC_SCALE_U128;
        let bf = ub % FRAC_SCALE_U128;

        // 3) 128×128 multiplies
        let int      = ai.checked_mul(bi)?;
        let t1       = ai.checked_mul(bf)?;
        let t2       = bi.checked_mul(af)?;
        let cross    = t1.checked_add(t2)?;
        let frac_mul = af.checked_mul(bf)?;
        let frac     = frac_mul / FRAC_SCALE_U128; // Safe

        // 4) reassemble
        let scaled_int = int.checked_mul(FRAC_SCALE_U128)?;
        let sum1       = scaled_int.checked_add(cross)?;
        let mag        = sum1.checked_add(frac)?;
        let mut repr: i128 = mag.try_into().ok()?;
        if neg { repr = repr.checked_neg()?; }
        Some(Self { repr })
    }

    /// Multiplication with checking for overflow and optimization for LHS or RHS being ints. You
    /// probably want to use `Dec19x19::checked_mul` with `mul_opt` flag enabled instead.
    #[track_caller]
    #[inline(always)]
    pub fn checked_mul_opt(self, rhs: Self) -> Option<Self> {
        // 1) sign & magnitudes
        let neg = (self.repr < 0) ^ (rhs.repr < 0);
        let ua  = self.repr.unsigned_abs();
        let ub  = rhs.repr.unsigned_abs();

        // 2) split into integer/fraction parts

        let bi = ub / FRAC_SCALE_U128;
        let bf = ub % FRAC_SCALE_U128;

        // 3) 128×128 multiplies
        let mag = if bf == 0 {
            ua.checked_mul(bi)?
        } else if bi == 0 {
            let ai = ua / FRAC_SCALE_U128;
            let af = ua % FRAC_SCALE_U128;
            let cross    = ai.checked_mul(bf)?;
            let frac_mul = af.checked_mul(bf)?;
            let frac     = frac_mul / FRAC_SCALE_U128; // Safe
            cross.checked_add(frac)?
        } else {
            let ai = ua / FRAC_SCALE_U128;
            let af = ua % FRAC_SCALE_U128;
            let int = ai.checked_mul(bi)?.checked_mul(FRAC_SCALE_U128)?;
            if af == 0 {
                let cross = ai.checked_mul(bf)?;
                int.checked_add(cross)?
            } else {
                let t1       = ai.checked_mul(bf)?;
                let t2       = bi.checked_mul(af)?;
                let cross    = t1.checked_add(t2)?;
                let frac_mul = af.checked_mul(bf)?;
                let frac     = frac_mul / FRAC_SCALE_U128; // Safe
                let sum1     = int.checked_add(cross)?;
                sum1.checked_add(frac)?
            }
        };
        let mut repr: i128 = mag.try_into().ok()?;
        if neg { repr = repr.checked_neg()?; }
        Some(Self { repr })
    }
}

impl UncheckedMul for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn unchecked_mul(self, rhs: Self) -> Self {
        #[cfg(feature = "mul_opt")]
        { self.unchecked_mul_opt(rhs) }
        #[cfg(not(feature = "mul_opt"))]
        { self.unchecked_mul_no_opt(rhs) }
    }
}

impl CheckedMul for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn checked_mul(self, rhs: Self) -> Option<Self> {
        #[cfg(feature = "mul_opt")]
        { self.checked_mul_opt(rhs) }
        #[cfg(not(feature = "mul_opt"))]
        { self.checked_mul_no_opt(rhs) }
    }
}

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::saturating_mul] {
///     (Dec19x19::MAX - Dec19x19!(10), Dec19x19!(2)) => Dec19x19::MAX,
///     (Dec19x19::MAX - Dec19x19!(10), Dec19x19!(-2)) => Dec19x19::MIN,
/// });
/// ```
impl SaturatingMul for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn saturating_mul(self, rhs: Self) -> Self {
        self.checked_mul(rhs).unwrap_or_else(||
            if self.signum_i128() * rhs.signum_i128() > 0 { Self::MAX } else { Self::MIN },
        )
    }
}

impl MulAssign for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl_op_for_refs!(Mul::mul);

// ===========
// === Div ===
// ===========

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::div, Dec19x19::checked_div] {
///     (Dec19x19!(20), Dec19x19!(0.2)) => Dec19x19!(100.0),
///     (Dec19x19::MAX, Dec19x19!(-1)) => Dec19x19::MIN + Dec19x19::SMALLEST_STEP,
///     (Dec19x19::MIN + Dec19x19::SMALLEST_STEP, Dec19x19!(-1)) => Dec19x19::MAX,
///     (Dec19x19::MAX - Dec19x19!(10), Dec19x19!(0.1)) => FAIL,
///     (Dec19x19::MAX - Dec19x19!(10), Dec19x19!(0)) => FAIL,
///     (Dec19x19!(10), Dec19x19!(0)) => FAIL,
///     (Dec19x19::MAX, Dec19x19!(0.1)) => FAIL,
/// });
/// ```
///
/// # Fuzzy
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// fuzzy2::<Dec19x19, BigDecimal>(Series::new(0..=9, 0..=9), Series::new(0..=9, 0..=9),
///     |(f1, b1), (f2, b2)| should_eq(f1 / f2, b1 / b2)
/// );
/// ```
impl Div for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        self.unchecked_div(rhs)
    }
}

impl UncheckedDiv for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn unchecked_div(self, rhs: Self) -> Self {
        let lhs_i256 = i256_from_i128(self.repr);
        let scaled_lhs = lhs_i256 * FRAC_SCALE_I256;
        let result = scaled_lhs / rhs.repr;
        #[cfg(inherit_overflow_checks)]
        { Self::from_repr(i256_to_i128(result).expect("Overflow in Dec19x19 division")) }
        #[cfg(not(inherit_overflow_checks))]
        { Self::from_repr(result.as_i128()) }
    }
}

impl CheckedDiv for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn checked_div(self, rhs: Self) -> Option<Self> {
        let lhs_i256 = i256_from_i128(self.repr);
        let rhs_i256 = i256_from_i128(rhs.repr);
        let scaled_lhs = lhs_i256 * FRAC_SCALE_I256;
        let result = scaled_lhs.checked_div(rhs_i256)?;
        i256_to_i128(result).map(Self::from_repr)
    }
}

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// assert_eq!((Dec19x19::MAX - Dec19x19!(10)).saturating_div(Dec19x19!(0.1)), Dec19x19::MAX);
/// assert_eq!((Dec19x19::MAX - Dec19x19!(10)).saturating_div(Dec19x19!(-0.1)), Dec19x19::MIN);
/// assert_eq!(Dec19x19::MIN.saturating_div(Dec19x19!(-1)), Dec19x19::MAX);
/// ```
impl SaturatingDiv for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn saturating_div(self, rhs: Self) -> Self {
        self.checked_div(rhs).unwrap_or_else(||
            if self.signum_i128() * rhs.signum_i128() >= 0 { Self::MAX } else { Self::MIN },
        )
    }
}

impl DivAssign for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl_op_for_refs!(Div::div);

// =============
// === Trunc ===
// =============

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::trunc_to] {
///     (Dec19x19::MAX, 0) => Dec19x19::MAX_INT,
///     (Dec19x19!( 3.9), 0) => Dec19x19!( 3.0),
///     (Dec19x19!( 3.1), 0) => Dec19x19!( 3.0),
///     (Dec19x19!( 3.0), 0) => Dec19x19!( 3.0),
///     (Dec19x19!(-3.9), 0) => Dec19x19!(-3.0),
///     (Dec19x19!(-3.1), 0) => Dec19x19!(-3.0),
///     (Dec19x19!(-3.0), 0) => Dec19x19!(-3.0),
///     (Dec19x19::MIN, 0) => Dec19x19::MIN_INT,
///
///     // Border `to` values.
///     (Dec19x19::MAX,  18) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572),
///     (Dec19x19::MAX,  19) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572_7),
///     (Dec19x19::MAX,  99) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572_7),
///     (Dec19x19::MAX, -18) => Dec19x19!(17_000_000_000_000_000_000),
///     (Dec19x19::MAX, -19) => Dec19x19!(10_000_000_000_000_000_000),
///     (Dec19x19::MAX, -99) => Dec19x19!(10_000_000_000_000_000_000),
/// });
/// ```
impl Dec19x19 {
    #[track_caller]
    #[inline(always)]
    const fn trunc_impl(self, scale: i128) -> Self {
        let int_part = self.repr / scale;
        Self { repr: int_part * scale }
    }
}

const_impl!{ impl Trunc for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn trunc(self) -> Self {
        self.trunc_impl(FRAC_SCALE_I128)
    }
}}

const_impl!{ impl TruncTo for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn trunc_to(self, digits: i64) -> Self {
        let scale = crate::i128_ops::scale_for(digits);
        self.trunc_impl(scale)
    }
}}

// =============
// === Floor ===
// =============

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::floor_to] {
///     (Dec19x19::MAX,     0) => Dec19x19::MAX_INT,
///     (Dec19x19!(3.9),    0) => Dec19x19!(3.0),
///     (Dec19x19!(3.1),    0) => Dec19x19!(3.0),
///     (Dec19x19!(3.0),    0) => Dec19x19!(3.0),
///     (Dec19x19!(-3.9),   0) => Dec19x19!(-4.0),
///     (Dec19x19!(-3.1),   0) => Dec19x19!(-4.0),
///     (Dec19x19!(-3.0),   0) => Dec19x19!(-3.0),
///     (Dec19x19::MIN_INT, 0) => Dec19x19::MIN_INT,
///
///     // No flooring below MIN_INT
///     ((Dec19x19::MIN_INT + Dec19x19::SMALLEST_STEP), 0) => Dec19x19::MIN_INT,
///     ((Dec19x19::MIN_INT - Dec19x19::SMALLEST_STEP), 0) => Dec19x19::MIN_INT - Dec19x19::SMALLEST_STEP,
///     (Dec19x19::MIN, 0) => Dec19x19::MIN,
///
///     // Border `to` values.
///     (Dec19x19::MAX,  18) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572),
///     (Dec19x19::MAX,  19) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572_7),
///     (Dec19x19::MAX,  99) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572_7),
///     (Dec19x19::MAX, -18) => Dec19x19!(17_000_000_000_000_000_000),
///     (Dec19x19::MAX, -19) => Dec19x19!(10_000_000_000_000_000_000),
///     (Dec19x19::MAX, -99) => Dec19x19!(10_000_000_000_000_000_000),
/// });
/// ```
impl Dec19x19 {
    #[track_caller]
    #[inline(always)]
    const fn floor_impl(self, scale: i128) -> Self {
        let frac = self.repr % scale;
        let has_fraction = frac != 0;
        let is_negative = self.repr < 0;
        let subtract_one = has_fraction & is_negative;
        let truncated = (self.repr / scale) * scale;
        let repr = if subtract_one {
            if let Some(result) = truncated.checked_sub(scale) {
                result
            } else {
                self.repr
            }
        } else {
            truncated
        };
        Self { repr }
    }
}

const_impl!{ impl Floor for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn floor(self) -> Self {
        self.floor_impl(FRAC_SCALE_I128)
    }
}}

const_impl!{ impl FloorTo for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn floor_to(self, digits: i64) -> Self {
        let scale = crate::i128_ops::scale_for(digits);
        self.floor_impl(scale)
    }
}}

// ============
// === Ceil ===
// ============

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::ceil_to] {
///     (Dec19x19::MAX,   0) => Dec19x19::MAX,
///     (Dec19x19!( 3.9), 0) => Dec19x19!( 4.0),
///     (Dec19x19!( 3.1), 0) => Dec19x19!( 4.0),
///     (Dec19x19!( 3.0), 0) => Dec19x19!( 3.0),
///     (Dec19x19!(-3.9), 0) => Dec19x19!(-3.0),
///     (Dec19x19!(-3.1), 0) => Dec19x19!(-3.0),
///     (Dec19x19!(-3.0), 0) => Dec19x19!(-3.0),
///     (Dec19x19::MIN,   0) => Dec19x19::MIN_INT,
///
///     // No ceiling above MAX_INT
///     ((Dec19x19::MAX - Dec19x19::SMALLEST_STEP), 0) => Dec19x19::MAX - Dec19x19::SMALLEST_STEP,
///     (Dec19x19::MAX_INT, 0) => Dec19x19::MAX_INT,
///     ((Dec19x19::MAX_INT - Dec19x19::SMALLEST_STEP), 0) => Dec19x19::MAX_INT,
///     ((Dec19x19::MAX_INT + Dec19x19::SMALLEST_STEP), 0) => Dec19x19::MAX_INT + Dec19x19::SMALLEST_STEP,
///
///     // Border `to` values.
///     (Dec19x19::MIN,  18) => Dec19x19!(-17_014_118_346_046_923_173.168_730_371_588_410_572),
///     (Dec19x19::MIN,  19) => Dec19x19!(-17_014_118_346_046_923_173.168_730_371_588_410_572_8),
///     (Dec19x19::MIN,  99) => Dec19x19!(-17_014_118_346_046_923_173.168_730_371_588_410_572_8),
///     (Dec19x19::MIN, -18) => Dec19x19!(-17_000_000_000_000_000_000),
///     (Dec19x19::MIN, -19) => Dec19x19!(-10_000_000_000_000_000_000),
///     (Dec19x19::MIN, -99) => Dec19x19!(-10_000_000_000_000_000_000),
/// });
/// ```
impl Dec19x19 {
    #[track_caller]
    #[inline(always)]
    const fn ceil_impl(self, scale: i128) -> Self {
        let frac = self.repr % scale;
        let has_fraction = frac != 0;
        let is_positive = self.repr > 0;
        let add_one = has_fraction & is_positive;
        let truncated = (self.repr / scale) * scale;
        let repr = if add_one {
            if let Some(result) = truncated.checked_add(scale) {
                result
            } else {
                self.repr
            }
        } else {
            truncated
        };
        Self { repr }
    }
}

const_impl!{ impl Ceil for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn ceil(self) -> Self {
        self.ceil_impl(FRAC_SCALE_I128)
    }
}}

const_impl!{ impl CeilTo for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn ceil_to(self, digits: i64) -> Self {
        let scale = crate::i128_ops::scale_for(digits);
        self.ceil_impl(scale)
    }
}}

// =============
// === Round ===
// =============

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::round_to] {
///     (Dec19x19!(3.9),  0) => Dec19x19!(4.0),
///     (Dec19x19!(3.6),  0) => Dec19x19!(4.0),
///     (Dec19x19!(3.5),  0) => Dec19x19!(4.0),
///     (Dec19x19!(3.4),  0) => Dec19x19!(3.0),
///     (Dec19x19!(3.0),  0) => Dec19x19!(3.0),
///     (Dec19x19!(-3.9), 0) => Dec19x19!(-4.0),
///     (Dec19x19!(-3.6), 0) => Dec19x19!(-4.0),
///     (Dec19x19!(-3.5), 0) => Dec19x19!(-4.0),
///     (Dec19x19!(-3.4), 0) => Dec19x19!(-3.0),
///     (Dec19x19!(-3.0), 0) => Dec19x19!(-3.0),
///
///     (Dec19x19!(0.39),  1) => Dec19x19!(0.4),
///     (Dec19x19!(0.36),  1) => Dec19x19!(0.4),
///     (Dec19x19!(0.35),  1) => Dec19x19!(0.4),
///     (Dec19x19!(0.34),  1) => Dec19x19!(0.3),
///     (Dec19x19!(0.30),  1) => Dec19x19!(0.3),
///     (Dec19x19!(-0.39), 1) => Dec19x19!(-0.4),
///     (Dec19x19!(-0.36), 1) => Dec19x19!(-0.4),
///     (Dec19x19!(-0.35), 1) => Dec19x19!(-0.4),
///     (Dec19x19!(-0.34), 1) => Dec19x19!(-0.3),
///     (Dec19x19!(-0.30), 1) => Dec19x19!(-0.3),
///
///     (Dec19x19!(39.0),  -1) => Dec19x19!(40),
///     (Dec19x19!(36.0),  -1) => Dec19x19!(40),
///     (Dec19x19!(35.0),  -1) => Dec19x19!(40),
///     (Dec19x19!(34.0),  -1) => Dec19x19!(30),
///     (Dec19x19!(30.0),  -1) => Dec19x19!(30),
///     (Dec19x19!(-39.0), -1) => Dec19x19!(-40),
///     (Dec19x19!(-36.0), -1) => Dec19x19!(-40),
///     (Dec19x19!(-35.0), -1) => Dec19x19!(-40),
///     (Dec19x19!(-34.0), -1) => Dec19x19!(-30),
///     (Dec19x19!(-30.0), -1) => Dec19x19!(-30),
///
///     // Possible to round values up.
///     ((Dec19x19::MAX - Dec19x19!(1)), 0) => Dec19x19::MAX_INT - Dec19x19!(1),
///     ((Dec19x19::MAX - Dec19x19!(1)), 1) => Dec19x19!(17_014_118_346_046_923_172.2),
///     ((Dec19x19::MAX - Dec19x19!(1)), 2) => Dec19x19!(17_014_118_346_046_923_172.17),
///     ((Dec19x19::MAX - Dec19x19!(1)), 3) => Dec19x19!(17_014_118_346_046_923_172.169),
///
///     // Impossible to round values up.
///     (Dec19x19::MAX, 0) => Dec19x19::MAX_INT,
///     (Dec19x19::MAX, 1) => Dec19x19!(17_014_118_346_046_923_173.1),
///     (Dec19x19::MAX, 2) => Dec19x19!(17_014_118_346_046_923_173.16),
///     (Dec19x19::MAX, 3) => Dec19x19!(17_014_118_346_046_923_173.168),
///
///     // Possible to round values down.
///     ((Dec19x19::MIN + Dec19x19!(1)), 0) => Dec19x19::MIN_INT + Dec19x19!(1),
///     ((Dec19x19::MIN + Dec19x19!(1)), 1) => Dec19x19!(-17_014_118_346_046_923_172.2),
///     ((Dec19x19::MIN + Dec19x19!(1)), 2) => Dec19x19!(-17_014_118_346_046_923_172.17),
///     ((Dec19x19::MIN + Dec19x19!(1)), 3) => Dec19x19!(-17_014_118_346_046_923_172.169),
///
///     // Impossible to round values down.
///     (Dec19x19::MIN, 0) => Dec19x19::MIN_INT,
///     (Dec19x19::MIN, 1) => Dec19x19!(-17_014_118_346_046_923_173.1),
///     (Dec19x19::MIN, 2) => Dec19x19!(-17_014_118_346_046_923_173.16),
///     (Dec19x19::MIN, 3) => Dec19x19!(-17_014_118_346_046_923_173.168),
///
///     // Rounding at the border of precision.
///     (Dec19x19!(1.123_456_789_012_345_678_0), 19) => Dec19x19!(1.123_456_789_012_345_678_0),
///     (Dec19x19!(1.123_456_789_012_345_678_4), 19) => Dec19x19!(1.123_456_789_012_345_678_4),
///     (Dec19x19!(1.123_456_789_012_345_678_5), 19) => Dec19x19!(1.123_456_789_012_345_678_5),
///     (Dec19x19!(1.123_456_789_012_345_678_6), 19) => Dec19x19!(1.123_456_789_012_345_678_6),
///     (Dec19x19!(1.123_456_789_012_345_678_9), 19) => Dec19x19!(1.123_456_789_012_345_678_9),
///     (Dec19x19!(1.123_456_789_012_345_678_0), 18) => Dec19x19!(1.123_456_789_012_345_678),
///     (Dec19x19!(1.123_456_789_012_345_678_4), 18) => Dec19x19!(1.123_456_789_012_345_678),
///     (Dec19x19!(1.123_456_789_012_345_678_5), 18) => Dec19x19!(1.123_456_789_012_345_679),
///     (Dec19x19!(1.123_456_789_012_345_678_6), 18) => Dec19x19!(1.123_456_789_012_345_679),
///     (Dec19x19!(1.123_456_789_012_345_678_9), 18) => Dec19x19!(1.123_456_789_012_345_679),
///
///     (Dec19x19::MAX,  18) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572),
///     (Dec19x19::MAX,  19) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572_7),
///     (Dec19x19::MAX,  99) => Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572_7),
///     (Dec19x19::MAX, -18) => Dec19x19!(17_000_000_000_000_000_000),
///     (Dec19x19::MAX, -19) => Dec19x19!(10_000_000_000_000_000_000),
///     (Dec19x19::MAX, -99) => Dec19x19!(10_000_000_000_000_000_000),
/// });
/// ```
///
/// # Validation
///
/// Note that the rounding here behaves slightly differently than `BigDecimal` crate. It might
/// fail if we change the scope or seed.
/// [Bug report](https://github.com/akubera/bigdecimal-rs/issues/149).
/// ```
/// // # use fixed_num::*;
/// // # use validator::*;
/// // for i in -7 ..= 7 {
/// //     fuzzy::<Dec19x19, BigDecimal>(Series::new(0..=19, 0..=19), Series::new(0..=19, 0..=19),
/// //         |(f1, b1), (f2, b2)| should_eq(f1.round_to(i), b1.round(i))
/// //     );
/// // }
/// ```
impl Dec19x19 {
    #[track_caller]
    #[inline(always)]
    const fn round_impl(self, scale: i128, scale_half: i128) -> Self {
        let sign = self.repr >> 127; // 0 for +, -1 for -
        let bias = (scale_half ^ sign) - sign; // HALF or -HALF without branches
        let rounded = if let Some(t) = self.repr.checked_add(bias) {
            t / scale
        } else {
            self.repr / scale
        };
        Self { repr: rounded * scale }
    }
}

const_impl!{ impl Round for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn round(self) -> Self {
        self.round_impl(FRAC_SCALE_I128, FRAC_SCALE_I128_HALF)
    }
}}

const_impl!{ impl RoundTo for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn round_to(self, digits: i64) -> Self {
        let scale = crate::i128_ops::scale_for(digits);
        let scale_half = scale / 2;
        self.round_impl(scale, scale_half)
    }
}}

// ============
// === Sqrt ===
// ============

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::unchecked_sqrt, Dec19x19::checked_sqrt] {
///     (Dec19x19!(0)) => Dec19x19!(0),
///     (Dec19x19::MAX) => Dec19x19!(4_124_817_371.235_594_858_790_322_117_5),
///     (-Dec19x19::SMALLEST_STEP) => FAIL,
/// });
/// // Precision test.
/// assert_eq!(Dec19x19!(1e-18).unchecked_sqrt() * Dec19x19!(1e-18).unchecked_sqrt(), Dec19x19!(1e-18));
/// ```
///
/// # Validation
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// fuzzy1::<Dec19x19, BigDecimal>(Series::new(0..=19, 0..=19),
///     |f1, b1| should_eq(f1.abs().unchecked_sqrt(), b1.abs().sqrt().unwrap())
/// );
/// ```
impl UncheckedSqrt for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn unchecked_sqrt(self) -> Self {
        assert!(self.repr >= 0, "sqrt: negative number");
        if self.repr == 0 {
            return Self::from_repr(0);
        }
        let initial_guess = {
            let self_f64 = self.repr as f64 / FRAC_SCALE_F64;
            let approx_sqrt = self_f64.sqrt();
            i256_from_i128((approx_sqrt * FRAC_SCALE_F64) as i128)
        };
        let x = i256_from_i128(self.repr);
        let scale = FRAC_SCALE_I256;
        let mut guess = initial_guess;
        let mut last;

        // Newton-Raphson loop
        loop {
            last = guess;
            guess = (guess + (x * scale) / guess) / I256_TWO;
            if (last - guess).wrapping_abs() <= i256::ONE {
                break;
            }
        }
        Self::from_repr(guess.as_i128())
    }
}

impl CheckedSqrt for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn checked_sqrt(self) -> Option<Self> {
        if self.repr < 0 {
            None
        } else {
            Some(self.unchecked_sqrt())
        }
    }
}

// ==================
// === Log10Floor ===
// ==================

const_impl!{
/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::unchecked_log10_floor, Dec19x19::checked_log10_floor] {
///     (Dec19x19::MAX)   => Dec19x19!(19),
///     (Dec19x19!(10.1)) => Dec19x19!(1),
///     (Dec19x19!(10.0)) => Dec19x19!(1),
///     (Dec19x19!(9.99)) => Dec19x19!(0),
///     (Dec19x19!(1.17)) => Dec19x19!(0),
///     (Dec19x19!(1.0))  => Dec19x19!(0),
///     (Dec19x19!(0.9))  => Dec19x19!(-1),
///     (Dec19x19!(0.11)) => Dec19x19!(-1),
///     (Dec19x19!(0.1))  => Dec19x19!(-1),
///     (Dec19x19!(0.09)) => Dec19x19!(-2),
///     (-Dec19x19::SMALLEST_STEP) => FAIL,
/// });
/// ```
impl UncheckedLog10Floor for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn unchecked_log10_floor(self) -> Self {
        assert!(self.repr > 0);
        // log10(repr / 10^19) = digit_count - 1 - 19
        Self::from_i32(crate::i128_ops::digit_count(self.repr) - 20)
    }
}}

const_impl!{ impl CheckedLog10Floor for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn checked_log10_floor(self) -> Option<Self> {
        if self.repr >= 0 {
            Some(self.unchecked_log10_floor())
        } else {
            None
        }
    }
}}

// ==========
// === Ln ===
// ==========

// sqrt(2) * 10^19   = 1.4142135623730950488e19
const SQRT2_UP_I128: i128 = 14_142_135_623_730_950_488;
// (10^19 / sqrt(2)) = 7.071067811865475244e18
const SQRT2_DN_I128: i128 =  7_071_067_811_865_475_244;

const SQRT2_UP_I256: i256 = i256_from_i128(SQRT2_UP_I128);
const SQRT2_DN_I256: i256 = i256_from_i128(SQRT2_DN_I128);

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// let trunc = |t: Dec19x19| t.trunc_to(17);
/// check!( [|t| trunc(Dec19x19::unchecked_ln(t)), |t| Dec19x19::checked_ln(t).map(trunc)] {
///     (Dec19x19::MAX) =>  trunc(Dec19x19!(44.280_575_164_226_186_298_3)),
///     (Dec19x19!(10)) =>  trunc(Dec19x19!(2.302_585_092_994_045_684_0)),
///     (Dec19x19!(100)) => trunc(Dec19x19!(4.605_170_185_988_091_367_8)),
///     (Dec19x19!(0.1)) => trunc(Dec19x19!(-2.302_585_092_994_045_683_7)),
///     (Dec19x19!(2.718281828459045239)) => Dec19x19!(1),
///     (-Dec19x19::SMALLEST_STEP) => FAIL,
/// });
/// ```
impl UncheckedLn for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn unchecked_ln(self) -> Self {
        debug_assert!(self.repr > 0);

        // 1) lift into i256
        let mut v      = i256_from_i128(self.repr);
        let scale      = FRAC_SCALE_I256;  // = 10^19 in i256
        let two        = I256_TWO;
        let ln2        = LN_2_I256;
        let sqrt2_up   = SQRT2_UP_I256;    // = scale*√2
        let sqrt2_dn   = SQRT2_DN_I256;    // = scale/√2

        // 2) range‑reduce v so that v ∈ [scale/√2, scale*√2]
        let mut exp = 0i128;
        while v > sqrt2_up {
            v /= two;
            exp += 1;
        }
        while v < sqrt2_dn {
            v *= two;
            exp -= 1;
        }

        // 3) atanh trick: u = (v−scale)/(v+scale), scaled by `scale`
        let num = v - scale;
        let den = v + scale;
        let u = (num * scale) / den;

        // 4) atanh-series: ln(v/scale) = 2·Σₖ [ u^(2k+1) / (2k+1) ]
        let mut u_pow = u;
        let mut sum   = u;
        let mut k     = 1i128;
        loop {
            // u_pow ← u_pow · u² / scale²
            u_pow = (u_pow * u / scale) * u / scale;
            k += 2;
            let term = u_pow / i256_from_i128(k);
            if term == i256::ZERO {
                break;
            }
            sum += term;
        }
        let ln_mant = sum * i256_from_i128(2);

        // 5) add back exponent·ln(2)
        let result = ln_mant + ln2 * i256_from_i128(exp);

        // 6) to Dec19x19, preserving your overflow‑checks cfg
        #[cfg(inherit_overflow_checks)]
        { Self::from_repr(i256_to_i128(result).expect("Overflow")) }
        #[cfg(not(inherit_overflow_checks))]
        { Self::from_repr(result.as_i128()) }
    }
}

impl CheckedLn for Dec19x19 {
    #[track_caller]
    #[inline(always)]
    fn checked_ln(self) -> Option<Self> {
        (self.repr > 0).then(|| self.unchecked_ln())
    }
}

// ===========
// === Pow ===
// ===========

/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// check! ( [Dec19x19::unchecked_pow, Dec19x19::checked_pow] {
///     // Identity and basic powers
///     (Dec19x19!(2), 0_i32) => Dec19x19!(1),
///     (Dec19x19!(2), 1_i32) => Dec19x19!(2),
///     (Dec19x19!(2), 3_i32) => Dec19x19!(8),
///     (Dec19x19!(2), 4_i32) => Dec19x19!(16),
///     (Dec19x19!(2), 5_i32) => Dec19x19!(32),
///     (Dec19x19!(2), 6_i32) => Dec19x19!(64),
///     (Dec19x19!(2), 7_i32) => Dec19x19!(128),
///     (Dec19x19!(2), 8_i32) => Dec19x19!(256),
///     (Dec19x19!(2), 9_i32) => Dec19x19!(512),
///     (Dec19x19!(2), 10_i32) => Dec19x19!(1024),
///     (Dec19x19!(2), 11_i32) => Dec19x19!(2048),
///     (Dec19x19!(2), 12_i32) => Dec19x19!(4096),
///     (Dec19x19!(2), 13_i32) => Dec19x19!(8192),
///     (Dec19x19!(2), 14_i32) => Dec19x19!(16384),
///     (Dec19x19!(2), 15_i32) => Dec19x19!(32768),
///     (Dec19x19!(2), 16_i32) => Dec19x19!(65536),
///
///     // Zero exponent
///     (Dec19x19!(20), 0) => Dec19x19!(1),
///
///     // Negative exponents
///     (Dec19x19!(2), -1_i32) => Dec19x19!(0.5),
///     (Dec19x19!(2), -2_i32) => Dec19x19!(0.25),
///
///     // Fractional bases
///     (Dec19x19!(0.5), 2_i32) => Dec19x19!(0.25),
///     (Dec19x19!(0.5), 3_i32) => Dec19x19!(0.125),
///     (Dec19x19!(0.5), -1_i32) => Dec19x19!(2.0),
///
///     // Fractional result rounding
///     (Dec19x19!(1.1), 2_i32) => Dec19x19!(1.21),
///     (Dec19x19!(1.5), 2_i32) => Dec19x19!(2.25),
///
///     // Larger integer base
///     (Dec19x19!(10), 3_i32) => Dec19x19!(1000),
///
///     (Dec19x19::MAX, -1_i32) => Dec19x19!(0),
///
///     (Dec19x19!(2), 63_i32) => Dec19x19!(9_223_372_036_854_775_808),
///     (Dec19x19!(2), 64) => FAIL,
///
///     (Dec19x19!(0), -1_i32) => FAIL,
///     (Dec19x19::MAX, 2_i32) => FAIL,
///     (Dec19x19::MIN, 2_i32) => FAIL,
/// });
///```
impl UncheckedPow<i32> for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn unchecked_pow(self, exp: i32) -> Self::Output {
        let mut result = Dec19x19!(1);
        let mut base   = if exp >= 0 { self } else { Dec19x19!(1) / self };
        let mut e      = exp.unsigned_abs();
        macro_rules! step {() => {
            let e2 = e / 2;
            let f2 = e % 2;
            if f2 == 1 {
                result *= base;
            }
            e = e2;
        };}
        if e > 0 { step!(); }
        while e > 0 {
            base = base * base;
            step!();
        }
        result
    }
}

impl CheckedPow<i32> for Dec19x19 {
    type Output = Self;
    #[track_caller]
    #[inline(always)]
    fn checked_pow(self, exp: i32) -> Option<Self::Output> {
        let mut result = Dec19x19!(1);
        let mut base   = if exp >= 0 { self } else { Dec19x19!(1) / self };
        let mut e      = exp.unsigned_abs();
        macro_rules! step {() => {
            let e2 = e / 2;
            let f2 = e % 2;
            if f2 == 1 {
                result = result.checked_mul(base)?;
            }
            e = e2;
        };}
        if e > 0 { step!(); }
        while e > 0 {
            base = base.checked_mul(base)?;
            step!();
        }
        Some(result)
    }
}

// =================================
// === Conversions X -> Dec19x19 ===
// =================================

macro_rules! gen_from_x_for_fix128 {
    ($($i:ident),* $(,)?) => { paste! {
        $(
            impl From<$i> for Dec19x19 {
                #[track_caller]
                #[inline(always)]
                fn from(value: $i) -> Self {
                    Self::[<from_ $i>](value)
                }
            }

            impl Dec19x19 {
                #[track_caller]
                #[inline(always)]
                pub const fn [<from_ $i>](value: $i) -> Self {
                    Self { repr: value as i128 * FRAC_SCALE_I128 }
                }
            }
        )*
    }};
}

macro_rules! gen_fn_try_from_x_for_fix128 {
    ($($i:ident),* $(,)?) => { paste! {
        $(
            impl Dec19x19 {
                #[track_caller]
                #[inline(always)]
                pub fn [<try_from_ $i>](value: $i) -> Result<Self, <Self as TryFrom<$i>>::Error> {
                    value.try_into()
                }
            }
        )*
    }};
}

// Creates a new `Dec19x19` from the given `i64` integer, assuming it has no fractional part.
// It is safe, as i64 has at most 19 digits.
gen_from_x_for_fix128! { i64, i32, i16, i8, u32, u16, u8 }
gen_fn_try_from_x_for_fix128!{ i128, u64, u128, f32, f64 }

impl TryFrom<i128> for Dec19x19 {
    type Error = &'static str;
    #[track_caller]
    #[inline(always)]
    fn try_from(value: i128) -> Result<Self, Self::Error> {
        let err = "Overflow: Value too large to store in Dec19x19.";
        let repr = value.checked_mul(FRAC_SCALE_I128).ok_or(err)?;
        Ok(Self { repr })
    }
}

impl TryFrom<u64> for Dec19x19 {
    type Error = &'static str;
    #[track_caller]
    #[inline(always)]
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        (value as i128).try_into()
    }
}

impl TryFrom<u128> for Dec19x19 {
    type Error = &'static str;
    #[track_caller]
    #[inline(always)]
    fn try_from(value: u128) -> Result<Self, Self::Error> {
        let err = "Overflow: Value too large to store in Dec19x19.";
        i128::try_from(value).ok().ok_or(err)?.try_into()
    }
}

impl TryFrom<f64> for Dec19x19 {
    type Error = &'static str;
    #[track_caller]
    #[inline(always)]
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let err_nan = "Cannot convert NaN or infinite value to Dec19x19.";
        let err_overflow = "Overflow: Value too large to store in Dec19x19.";
        let err_underflow = "Underflow: Value too small to store in Dec19x19.";
        let scaled = value * FRAC_SCALE_I128 as f64;
        let repr_f64 = scaled.round();
        if !repr_f64.is_finite() { return Err(err_nan); }
        if repr_f64 > i128::MAX as f64 { return Err(err_overflow); }
        if repr_f64 < i128::MIN as f64 { return Err(err_underflow); }
        Ok(Self { repr: repr_f64 as i128 })
    }
}

impl TryFrom<f32> for Dec19x19 {
    type Error = &'static str;
    #[track_caller]
    #[inline(always)]
    fn try_from(value: f32) -> Result<Self, Self::Error>  {
        (value as f64).try_into()
    }
}

// =================================
// === Conversions Dec19x19 -> X ===
// =================================

macro_rules! gen_from_fix128_for_x {
    ($($i:ident),* $(,)?) => {
        $(
            impl From<Dec19x19> for $i {
                #[track_caller]
                #[inline(always)]
                #[allow(trivial_numeric_casts)]
                fn from(value: Dec19x19) -> Self {
                    (value.repr / FRAC_SCALE_I128) as Self
                }
            }
        )*
    };
}

macro_rules! gen_fn_from_fix128_for_x {
    ($($i:ident),* $(,)?) => { paste! {
        $(
            impl Dec19x19 {
                #[track_caller]
                #[inline(always)]
                pub fn [<into_ $i>](self) -> $i {
                    self.into()
                }
            }
        )*
    }};
}

gen_from_fix128_for_x! { u64, i128, u128 }
gen_fn_from_fix128_for_x! { u64, i128, u128, f64, f32 }

macro_rules! gen_try_from_fix128_for_x {
    ($($i:ident),* $(,)?) => {
        $(
            impl TryFrom<Dec19x19> for $i {
                type Error = &'static str;
                #[track_caller]
                #[inline(always)]
                fn try_from(value: Dec19x19) -> Result<Self, Self::Error> {
                    let val = value.repr / FRAC_SCALE_I128;
                    if val > $i::MAX as i128 || val < $i::MIN as i128 {
                        return Err("Overflow: Dec19x19 too large or too small.");
                    }
                    Ok(val as $i)
                }
            }
        )*
    };
}

macro_rules! gen_fn_try_from_fix128_for_x {
    ($($i:ident),* $(,)?) => { paste! {
        $(
            impl Dec19x19 {
                #[track_caller]
                #[inline(always)]
                pub fn [<try_into_ $i>](self) -> Result<$i, <$i as TryFrom<Self>>::Error> {
                    self.try_into()
                }
            }
        )*
    }};
}

gen_try_from_fix128_for_x! { i64, u32, i32, u16, i16, u8, i8 }
gen_fn_try_from_fix128_for_x! { i64, u32, i32, u16, i16, u8, i8 }

impl From<Dec19x19> for f64 {
    #[track_caller]
    #[inline(always)]
    fn from(value: Dec19x19) -> Self {
        let int_part = (value.repr / FRAC_SCALE_I128) as Self;
        let frac_part = (value.repr % FRAC_SCALE_I128) as Self / FRAC_SCALE_I128 as Self;
        int_part + frac_part
    }
}

impl From<Dec19x19> for f32 {
    #[track_caller]
    #[inline(always)]
    fn from(value: Dec19x19) -> Self {
        f64::from(value) as Self
    }
}

// ===========================
// === Parsing and Display ===
// ===========================

/// # Tests
///
/// ```
/// # use fixed_num::*;
/// # use validator::*;
/// use std::str::FromStr;
/// assert_eq!(Dec19x19!(17_014_118_346_046_923_173.168_730_371_588_410_572_7).repr, i128::MAX);
/// assert_eq!(Dec19x19!(-17_014_118_346_046_923_173.168_730_371_588_410_572_8).repr, i128::MIN);
/// assert_eq!(Dec19x19!(987e-19), Dec19x19!(0.000_000_000_000_000_098_7));
/// assert_eq!(Dec19x19!(987e-2), Dec19x19!(9.87));
/// assert_eq!(Dec19x19!(987e-1), Dec19x19!(98.7));
/// assert_eq!(Dec19x19!(987e-0), Dec19x19!(987));
/// assert_eq!(Dec19x19!(987e0), Dec19x19!(987));
/// assert_eq!(Dec19x19!(987e+0), Dec19x19!(987));
/// assert_eq!(Dec19x19!(987e+1), Dec19x19!(9_870));
/// assert_eq!(Dec19x19!(987e+2), Dec19x19!(98_700));
/// assert_eq!(Dec19x19!(987e16), Dec19x19!(9_870_000_000_000_000_000));
/// assert_eq!(Dec19x19!(1_000_000_000_000_000e-34), Dec19x19::SMALLEST_STEP);
/// assert_eq!(Dec19x19!(0.000_000_000_000_000e34), Dec19x19!(0));
/// assert!(Dec19x19::from_str("17_014_118_346_046_923_173.168_730_371_588_410_572_8").is_err());
/// assert!(Dec19x19::from_str("-17_014_118_346_046_923_173.168_730_371_588_410_572_9").is_err());
/// assert!(Dec19x19::from_str("987e+17").is_err());
/// assert!(Dec19x19::from_str("987e-20").is_err());
/// ```
impl FromStr for Dec19x19 {
    type Err = ParseDec19x19Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let repr = parse_dec19x19_internal(s)?;
        Ok(Self { repr })
    }
}

impl<'t> TryFrom<&'t str> for Dec19x19 {
    type Error = ParseDec19x19Error;
    fn try_from(s: &'t str) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl<'t> TryFrom<&'t String> for Dec19x19 {
    type Error = ParseDec19x19Error;
    fn try_from(s: &'t String) -> Result<Self, Self::Error> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for Dec19x19 {
    type Error = ParseDec19x19Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::from_str(&s)
    }
}

impl std::fmt::Display for Dec19x19 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let separator = f.alternate().then_some('_');
        let mut formatter = Formatter {
            separator,
            precision: f.precision(),
            width: f.width(),
            align: f.align(),
            fill: f.fill(),
            sign_plus: f.sign_plus(),
        };
        write!(f, "{}", self.format(&mut formatter))
    }
}

impl std::fmt::Debug for Dec19x19 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

// Tested in README.md.
impl Format for Dec19x19 {
    fn format(&self, f: &mut Formatter) -> String {
        let this = f.precision.map_or(*self, |p| self.round_to(p.min(19) as i64));
        let int_part = this.repr / FRAC_SCALE_I128;
        let frac_part = (this.repr % FRAC_SCALE_I128).abs();

        let mut frac_str = format!("{:0width$}", frac_part, width = FRAC_PLACES as usize)
            .trim_end_matches('0')
            .to_string();

        if let Some(prec) = f.precision {
            if frac_str.len() < prec {
                let zeros_needed = prec - frac_str.len();
                frac_str.push_str(&"0".repeat(zeros_needed));
            }
        }

        let int_str = int_part.abs().to_string();
        let sign_len = 1;
        let int_str_len = int_str.len();
        let int_len = int_str_len + int_str_len / 3;
        let frac_len = frac_str.len() + frac_str.len() / 3;
        let mut result = String::with_capacity(sign_len + int_len + frac_len + 1);
        if this.repr < 0 {
            result.push('-');
        } else if f.sign_plus {
            result.push('+');
        }

        for (i, c) in int_str.chars().enumerate() {
            let j = int_str_len - i;
            if i != 0 && j > 0 && j % 3 == 0 {
                if let Some(sep) = f.separator {
                    result.push(sep);
                }
            }
            result.push(c);
        }

        if !frac_str.is_empty() {
            result.push('.');
            for (i, c) in frac_str.chars().enumerate() {
                if i > 0 && i % 3 == 0 {
                    if let Some(sep) = f.separator {
                        result.push(sep);
                    }
                }
                result.push(c);
            }
        }

        if let Some(width) = f.width {
            let fill = f.fill.to_string();
            let padding = width.saturating_sub(result.len());
            match f.align {
                Some(std::fmt::Alignment::Right) => result.push_str(&fill.repeat(padding)),
                Some(std::fmt::Alignment::Center) => {
                    let left_padding = padding / 2;
                    let right_padding = padding - left_padding;
                    result.insert_str(0, &fill.repeat(left_padding));
                    result.push_str(&fill.repeat(right_padding));
                }
                _ => result.insert_str(0, &fill.repeat(padding)),
            }
        }

        result
    }
}
