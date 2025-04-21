pub use bigdecimal::BigDecimal;
use std::str::FromStr;
use std::fmt::{Debug, Display};
use fixed_num_helper::*;

#[derive(Clone, Debug)]
pub struct Series {
    pub seed: u64,
    pub int_prec: RandRange,
    pub frac_prec: RandRange,
}

impl Series {
    pub fn new(int_prec: impl IntoRandRange, frac_prec: impl IntoRandRange) -> Self {
        let int_prec = int_prec.into_rand_range();
        let frac_prec = frac_prec.into_rand_range();
        Self {
            seed: 0,
            int_prec,
            frac_prec,
        }
    }
}

pub fn series_str<T>(cfg: Series) -> Vec<String>
where T: Rand + Display {
    let count = 10_000;
    let seed_base = cfg.seed * 1_000_000;
    (0..count)
        .map(|i| T::rand(seed_base + i, cfg.int_prec.clone(), cfg.frac_prec.clone()))
        .map(|t| format!("{t}"))
        .collect()
}

pub fn series_pair1<A, B>(mut cfg: Series) -> Vec<(A, B)> where
A: Rand + Display + FromStr<Err:Debug>,
B: FromStr<Err:Debug> {
    if cfg.seed == 0 { cfg.seed = 7; }
    let vec_str = series_str::<A>(cfg);
    let a_vec = vec_str.iter().map(|a| A::from_str(a).unwrap()).collect::<Vec<_>>();
    let b_vec = vec_str.iter().map(|a| B::from_str(a).unwrap()).collect::<Vec<_>>();
    a_vec.into_iter().zip(b_vec.into_iter()).collect::<Vec<_>>()
}

pub fn series_pair2<A, B>(mut cfg1: Series, mut cfg2: Series) -> Vec<((A, B), (A, B))> where
A: Rand + Display + FromStr<Err:Debug>,
B: FromStr<Err:Debug> {
    if cfg1.seed == 0 { cfg1.seed = 7; }
    if cfg2.seed == 0 { cfg2.seed = 17; }
    series_pair1(cfg1).into_iter().zip(series_pair1(cfg2).into_iter()).collect()
}

pub fn fuzzy1<A, B>(cfg1: Series, f: impl Fn(A, B)) where
    A: Rand + Display + FromStr<Err:Debug>,
    B: FromStr<Err:Debug> {
    for (a, b) in series_pair1::<A, B>(cfg1) {
        f(a, b);
    }
}

pub fn fuzzy2<A, B>(cfg1: Series, cfg2: Series, f: impl Fn((A, B), (A, B))) where
A: Rand + Display + FromStr<Err:Debug>,
B: FromStr<Err:Debug> {
    for (a, b) in series_pair2::<A, B>(cfg1, cfg2) {
        f(a, b);
    }
}

pub fn should_panic<T: Debug>(f: impl FnOnce() -> T + std::panic::UnwindSafe, desc: &str) {
    let result = std::panic::catch_unwind(|| f());
    assert!(result.is_err(), "Expected panic, but got: {result:?} in {desc}");
}


pub fn cmp<T>(a: T, b: BigDecimal) -> Result<(), String>
where T: Copy + Display {
    let a_str = format!("{a:.19}");
    let b_str = format!("{:.19}", b.with_scale(19));
    if a_str == b_str {
        Ok(())
    } else {
        Err(format!("Mismatch: {a_str} != {b_str}"))
    }
}

pub trait ShouldEq<T> {
    fn should_eq(self, other: T);
}

impl<T> ShouldEq<BigDecimal> for T
where T: Display {
    fn should_eq(self, other: BigDecimal) {
        let a_str = format!("{self:.19}");
        let b_str = format!("{:.19}", other.with_scale(19));
        assert_eq!(a_str, b_str, "Mismatch: {a_str} != {b_str}");
    }
}

pub fn should_eq<A, B>(a: A, b: B)
where A: ShouldEq<B> {
    a.should_eq(b);
}

#[macro_export]
macro_rules! check {
    ( [] $cases:tt ) => {};
    ( [$f:expr $(, $($fns:tt)*)?] $cases:tt ) => {
        check! { @1 $f, $cases }
        check! { [$($($fns)*)?] $cases }
    };
    ( @1 $f:expr, {}) => {

    };
    ( @1 $f:expr, { $args:tt => FAIL $(, $($ts:tt)* )? } ) => {
        check! { @2 $f, $args => FAIL }
        check! { @1 $f, { $($($ts)*)? } }
    };
    ( @1 $f:expr, { $args:tt => $out:expr $(, $($ts:tt)* )? } ) => {
        check! { @2 $f, $args => $out }
        check! { @1 $f, { $($($ts)*)? } }
    };
    ( @2 $f:expr, ($($args:tt)*) => FAIL ) => {
        should_panic(|| $f($($args)*).unwrap_all(), stringify!($f($($args)*) != FAIL));
    };
    ( @2 $f:expr, ($($args:tt)*) => $out:expr ) => {
        assert_eq!($f($($args)*).unwrap_all(), $out, stringify!($f($($args)*) != $out));
    };
}
