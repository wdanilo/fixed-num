use criterion::{Criterion, criterion_group};
use paste::paste;
use std::fmt::Debug;
use std::hint::black_box;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use ::bigdecimal::Signed;
use ::fixed_num::traits::*;
use ::rust_decimal::MathematicalOps;

use ::bigdecimal::BigDecimal as bigdecimal;
use ::decimal::d128 as decimal;
use ::decimal_rs::Decimal as decimal_rs;
use ::fastnum::D128 as fastnum;
use ::fixed::FixedI128;
use ::fixed::types::extra::U64;
use ::fixed_num::Dec19x19 as fixed_num;
use ::rust_decimal::Decimal as rust_decimal;
use validator::Series;

#[expect(non_camel_case_types)]
type _fixed = FixedI128<U64>; // Not used as it panics in many operations.

// ========================
// === Criterion Config ===
// ========================

const WORKSPACE_ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn out_dir() -> PathBuf {
    Path::new(WORKSPACE_ROOT).join("target").join("criterion")
}

fn config() -> Criterion {
    Criterion::default()
        .noise_threshold(1.0)
        .output_directory(&out_dir())
}

// =========================
// === Output Generation ===
// =========================

#[derive(Debug, Default)]
struct Buffer {
    ident: usize,
    str: String,
}

impl Buffer {
    fn line(&mut self, s: &str) {
        self.str.push_str(&"  ".repeat(self.ident));
        self.str.push_str(s);
        self.str.push('\n');
    }

    fn group_start(&mut self, s: &str) {
        self.line(s);
        self.ident += 1;
    }

    fn group_end(&mut self, s: &str) {
        self.ident -= 1;
        self.line(s);
    }
}

fn normalize_by(input: Vec<Option<f64>>, ix: usize) -> Vec<Option<f64>> {
    let base = input[ix].unwrap();
    input
        .into_iter()
        .map(|opt| opt.map(|val| base / val))
        .collect()
}

fn after_benchmarks(ops: &[&str], libs: &[&str]) {
    let out_dir = out_dir();
    let results = &ops
        .iter()
        .map(|op| {
            let results = libs
                .iter()
                .map(|lib| {
                    let path = out_dir
                        .join(format!("{op} {lib}"))
                        .join("new")
                        .join("estimates.json");
                    path.exists()
                        .then(|| {
                            let content = std::fs::read_to_string(&path).unwrap();
                            let json: serde_json::Value = serde_json::from_str(&content).unwrap();
                            // `json["slope"]` is a little bit closer to what Criterion reports in terminal,
                            // but it often doesn't exist in the generated json.
                            json["median"]["point_estimate"].as_f64()
                        })
                        .flatten()
                })
                .collect::<Vec<_>>();
            normalize_by(results, 1)
        })
        .collect::<Vec<_>>();

    let mut out = Buffer::default();
    out.group_start("<table>");
    out.group_start("<thead>");
    out.group_start("<tr>");
    out.line("<th></th>");
    for lib in libs {
        out.line(&format!("<th>{lib}</th>"));
    }
    out.group_end("</tr>");
    out.group_end("</thead>");
    out.group_start("<tbody>");
    for (op, results) in ops.iter().zip(results) {
        out.group_start("<tr>");
        out.line(&format!("<td>{op}</td>"));
        let max = results
            .iter()
            .skip(1)
            .filter_map(|x| *x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        for result in results {
            let norm = result.map(|x| x / max).unwrap_or(0.01);
            let coeff = ((1.0 + norm.log10()).max(0.0).min(1.0) * 100.0).round();
            let bg = format!("color-mix(in lch, #58760b {coeff}%, #c41c0d)");
            let fg_opacity = if norm == 1.0 || result.is_none() {
                1.0
            } else {
                0.5
            };
            let fg = format!("rgba(255, 255, 255, {fg_opacity})");
            let font = if norm == 1.0 { "bold" } else { "normal" };
            let style = format!(
                "\
                style=\"color: {fg};\
                background-color: {bg};\
                font-weight: {font};\"\
            "
            )
            .replace("  ", " ");
            match result {
                Some(value) => out.line(&format!("<td {style}>{value:.2}</td>")),
                None => out.line(&format!("<td {style}>⚠️</td>")),
            }
        }
        out.group_end("</tr>");
    }
    out.group_end("</tbody>");
    out.group_end("</table>");

    let out_path = Path::new(WORKSPACE_ROOT).join("results.html");
    let mut file = std::fs::File::create(&out_path).unwrap();
    file.write_all(out.str.as_bytes()).unwrap();
}

// ===================
// === Bench Utils ===
// ===================

macro_rules! def_benches {
    ([$($all_tgt:tt)*] $( $op:ident for [$($tgt:tt)*] { $f:ident $args:tt })* ) => {
        def_benches! { @1 [f64, fixed_num, $($all_tgt)*]
            $($op for [f64, fixed_num, $($tgt)*] $f $args ;)*
        }
    };

    (@1 [$($tgts:ident),* $(,)?] $( $op:ident for [$($t:ident),* $(,)?] $f:ident $args:tt ;)* ) => {
        $( def_bench! { @0 $op for [$($t,)*] $f $args } )*

        fn main() {
            $($op();)*
            criterion::Criterion::default()
                .configure_from_args()
                .final_summary();

            let ops = &[$(stringify!($op)),*];
            let targets = &[$(stringify!($tgts)),*];
            after_benchmarks(ops, &targets[..]);
        }
    };
}

macro_rules! def_bench {
    (@0 $op:ident for [$($t:ty),* $(,)?] $f:ident $args:tt) => { paste! {
        $(
            def_bench! { @1 $op for [$t] $f $args }
        )*

        criterion_group! {
            name = $op;
            config = config();
            targets = $([< bench_ $op _ $t >],)*
        }
    }};

    (@1 $op:ident for [$t:ty] $f:ident ($($args:tt)*)) => { paste! {
        #[allow(non_snake_case)]
        fn [< bench_ $op _ $t >](c: &mut Criterion) {
            $f::<$t>(c, &format!("{} {}", stringify!($op), stringify!($t)), $($args)*);
        }
    }};
}

#[allow(non_snake_case)]
fn bench1<T>(c: &mut Criterion, label: &str, mut series1: Series, f: impl Fn(&T) -> T)
where
    T: FromStr<Err: Debug>,
{
    series1.seed = 7;
    let a_series = validator::series_str::<fixed_num>(series1);
    let a_vec: Vec<_> = a_series
        .iter()
        .map(|s| black_box(T::from_str(s).unwrap()))
        .collect();
    c.bench_function(label, |bencher| {
        bencher.iter(|| {
            for a in a_vec.iter() {
                black_box(f(a));
            }
        })
    });
}

#[allow(non_snake_case)]
fn bench2<T>(
    c: &mut Criterion,
    label: &str,
    mut series1: Series,
    mut series2: Series,
    f: impl Fn(&T, &T) -> T,
) where
    T: FromStr<Err: Debug>,
{
    series1.seed = 7;
    series2.seed = 17;
    let a_series = validator::series_str::<fixed_num>(series1);
    let b_series = validator::series_str::<fixed_num>(series2);
    let a_vec: Vec<_> = a_series
        .iter()
        .map(|s| black_box(T::from_str(s).unwrap()))
        .collect();
    let b_vec: Vec<_> = b_series
        .iter()
        .map(|s| black_box(T::from_str(s).unwrap()))
        .collect();
    c.bench_function(label, |bencher| {
        bencher.iter(|| {
            for (a, b) in a_vec.iter().zip(b_vec.iter()) {
                black_box(f(a, b));
            }
        })
    });
}

trait RollingWindowBounds: SubWrapper + AddWrapper + DivWrapper + MulWrapper + From<u32> {}
impl<T> RollingWindowBounds for T where
    T: SubWrapper + AddWrapper + DivWrapper + MulWrapper + From<u32>
{
}

fn rolling_window_avg<T>(values: &[T], window_size: usize) -> Vec<T>
where
    T: RollingWindowBounds,
{
    let window_size_t = T::from(window_size as u32);
    let one_over_window_size_t = T::from(1).div_wrapper(&window_size_t);
    if values.len() < window_size || window_size == 0 {
        return vec![];
    }

    let mut result = Vec::with_capacity(values.len() - window_size + 1);
    let mut sum: T = 0.into();
    for i in 0..window_size {
        sum = sum.add_wrapper(&values[i]);
    }
    result.push(sum.mul_wrapper(&one_over_window_size_t));

    for i in window_size..values.len() {
        sum = (sum.add_wrapper(&values[i])).sub_wrapper(&values[i - window_size]);
        result.push(sum.mul_wrapper(&one_over_window_size_t));
    }

    result
}

#[allow(non_snake_case)]
fn bench_rolling_window<T>(c: &mut Criterion, label: &str)
where
    T: RollingWindowBounds + FromStr<Err: Debug>,
{
    let series = validator::series_str::<fixed_num>(Series::new(0..=15, 0..=19));
    let values = series
        .iter()
        .map(|s| T::from_str(&s).unwrap())
        .collect::<Vec<T>>();
    c.bench_function(label, |bencher| {
        bencher.iter(|| black_box(rolling_window_avg(&values, 10)))
    });
}

// ==================
// === Benchmarks ===
// ==================

def_benches! { [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum]
    eq for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=9, 0..=19), Series::new(0..=9, 0..=19),
            |a, b| if a.eq_wrapper(b) { a.clone() } else { b.clone() }
        )
    }
    ord for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=9, 0..=19), Series::new(0..=9, 0..=19),
            |a, b| if a > b { a.clone() } else { b.clone() }
        )
    }
    signum for [rust_decimal, bigdecimal, fastnum] {
        bench1(Series::new(0..=9, 0..=19),
            |a| a.signum_wrapper()
        )
    }
    neg for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench1(Series::new(0..=9, 0..=19),
            |a| a.neg()
        )
    }
    abs for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench1(Series::new(0..=19, 0..=19),
            |a| a.abs()
        )
    }
    rem for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=19, 0..=19), Series::new(0..=19, 0..=19),
            |a, b| a.rem(b.clone())
        )
    }
    add for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=18, 0..=19), Series::new(0..=18, 0..=19),
            |a, b| a.add_wrapper(b)
        )
    }
    sub for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=18, 0..=19), Series::new(0..=18, 0..=19),
            |a, b| a.sub_wrapper(b)
        )
    }
    mul_fxf for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=9, 19), Series::new(0..=9, 19),
            |a, b| a.mul_wrapper(b)
        )
    }
    mul_fxi for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=9, 19), Series::new(0..=9, 0),
            |a, b| a.mul_wrapper(b)
        )
    }
    mul_ixi for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=9, 0), Series::new(0..=9, 0),
            |a, b| a.mul_wrapper(b)
        )
    }
    div for [rust_decimal, bigdecimal, decimal, decimal_rs, fastnum] {
        bench2(Series::new(0..=9, 0..=9), Series::new(0..=9, 0..=9),
            |a, b| a.div_wrapper(b)
        )
    }
    checked_add for [rust_decimal, bigdecimal, decimal_rs] {
        bench2(Series::new(0..=18, 0..=19), Series::new(0..=18, 0..=19),
            |a, b| a.checked_add_wrapper(b)
        )
    }
    checked_sub for [rust_decimal, bigdecimal, decimal_rs] {
        bench2(Series::new(0..=18, 0..=19), Series::new(0..=18, 0..=19),
            |a, b| a.checked_sub_wrapper(b)
        )
    }
    checked_mul_fxf for [rust_decimal, bigdecimal, decimal_rs] {
        bench2(Series::new(0..=9, 19), Series::new(0..=9, 19),
            |a, b| a.checked_mul_wrapper(b)
        )
    }
    checked_mul_fxi for [rust_decimal, bigdecimal, decimal_rs] {
        bench2(Series::new(0..=9, 0..=19), Series::new(0..=9, 0),
            |a, b| a.checked_mul_wrapper(b)
        )
    }
    checked_mul_ixi for [rust_decimal, bigdecimal, decimal_rs] {
        bench2(Series::new(0..=9, 0), Series::new(0..=9, 0),
            |a, b| a.checked_mul_wrapper(b)
        )
    }
    checked_div for [rust_decimal, bigdecimal, decimal_rs] {
        bench2(Series::new(0..=9, 0..=9), Series::new(0..=9, 0..=9),
            |a, b| a.checked_div_wrapper(b)
        )
    }
    trunc for [rust_decimal, decimal_rs] {
        bench1(Series::new(0..=19, 0..=19),
            |a| a.trunc_wrapper()
        )
    }
    floor for [rust_decimal, decimal_rs, fastnum] {
        bench1(Series::new(0..=19, 0..=19),
            |a| a.floor()
        )
    }
    ceil for [rust_decimal, decimal_rs, fastnum] {
        bench1(Series::new(0..=19, 0..=19),
            |a| a.ceil()
        )
    }
    round for [rust_decimal, bigdecimal, decimal_rs, fastnum] {
        bench1(Series::new(0..=19, 0..=19),
            |a| { for i in -7 .. 7 { a.round_to_wrapper(i); } a.clone() }
        )
    }
    powi for [rust_decimal, decimal, decimal_rs, fastnum] {
        bench1(Series::new(0..=1, 0..=19),
            |a| { for i in 2 .. 16 { a.powi_wrapper(i); } a.clone() }
        )
    }
    sqrt for [rust_decimal, bigdecimal, decimal_rs, fastnum] {
        bench1(Series::new(0..=19, 0..=19),
            |a| a.abs().sqrt_wrapper()
        )
    }
    ln for [rust_decimal, decimal, decimal_rs] {
        bench1(Series::new(0..=19, 0..=19),
            |a| a.abs().ln_wrapper()
        )
    }
    log10_floor for [rust_decimal, decimal, fastnum] {
        bench1(Series::new(0..=19, 0..=19),
            |a| a.abs().log10_floor_wrapper()
        )
    }
    rolling_window for [rust_decimal, bigdecimal, decimal_rs, fastnum] {
        bench_rolling_window()
    }
}

// ================
// === Wrappers ===
// ================
// Wrappers for operations to unify the API between libraries. Should be zero-cost (confirmed
// manually).

macro_rules! wrapper {
    (
        trait $trait:ident {
            fn $fn:ident $fn_args:tt -> $fn_out:ty {
                $(
                    $target:ident => $expr:expr
                ),*
                $(,)*
            }
        }
    ) => {
        pub trait $trait {
            fn $fn $fn_args -> $fn_out;
        }

        $(
            impl $trait for $target {
                #[inline(always)]
                fn $fn $fn_args -> $fn_out {
                    $expr
                }
            }
        )*
    };
}

wrapper! {
    trait AddWrapper {
        fn add_wrapper(&self, other: &Self) -> Self {
            f64          => self.add(other),
            fixed_num    => self.add(other),
            rust_decimal => self.add(other),
            bigdecimal   => self.add(other),
            decimal      => self.add(other),
            decimal_rs   => self.add(other),
            fastnum      => self.add(*other),
        }
    }
}

wrapper! {
    trait SubWrapper {
        fn sub_wrapper(&self, other: &Self) -> Self {
            f64          => self.sub(other),
            fixed_num    => self.sub(other),
            rust_decimal => self.sub(other),
            bigdecimal   => self.sub(other),
            decimal      => self.sub(other),
            decimal_rs   => self.sub(other),
            fastnum      => self.sub(*other),
        }
    }
}

wrapper! {
    trait MulWrapper {
        fn mul_wrapper(&self, other: &Self) -> Self {
            f64          => self.mul(other),
            fixed_num    => self.mul(other),
            rust_decimal => self.mul(other),
            bigdecimal   => self.mul(other),
            decimal      => self.mul(other),
            decimal_rs   => self.mul(other),
            fastnum      => self.mul(*other),
        }
    }
}

wrapper! {
    trait DivWrapper {
        fn div_wrapper(&self, other: &Self) -> Self {
            f64          => self.div(other),
            fixed_num    => self.div(other),
            rust_decimal => self.div(other),
            bigdecimal   => self.div(other),
            decimal      => self.div(other),
            decimal_rs   => self.div(other),
            fastnum      => self.div(*other),
        }
    }
}

wrapper! {
    trait EqWrapper {
        fn eq_wrapper(&self, other: &Self) -> bool {
            f64          => (self - other).abs() < f64::EPSILON,
            fixed_num    => self.eq(other),
            rust_decimal => self.eq(other),
            bigdecimal   => self.eq(other),
            decimal      => self.eq(other),
            decimal_rs   => self.eq(other),
            fastnum      => self.eq(other),
        }
    }
}

wrapper! {
    trait SqrtWrapper {
        fn sqrt_wrapper(&self) -> Self {
            f64          => self.sqrt(),
            fixed_num    => self.unchecked_sqrt(),
            rust_decimal => self.sqrt().unwrap(),
            bigdecimal   => self.sqrt().unwrap(),
            decimal_rs   => self.sqrt().unwrap(),
            fastnum      => self.sqrt(),
        }
    }
}

wrapper! {
    trait LnWrapper {
        fn ln_wrapper(&self) -> Self {
            f64          => self.ln(),
            fixed_num    => self.unchecked_ln(),
            rust_decimal => self.ln(),
            decimal      => self.ln(),
            decimal_rs   => self.ln().unwrap(),
            fastnum      => self.ln(),
        }
    }
}

wrapper! {
    trait SignumWrapper {
        fn signum_wrapper(&self) -> Self {
            f64          => self.signum(),
            fixed_num    => self.signum(),
            rust_decimal => self.signum(),
            bigdecimal   => self.signum(),
            fastnum      => self.signum(),
        }
    }
}

wrapper! {
    trait CheckedAddWrapper {
        fn checked_add_wrapper(&self, other: &Self) -> Self {
            f64 => {
                let val = self + other;
                let result = if val.is_finite() {
                    Some(val)
                } else {
                    None
                };
                result.unwrap()
            },
            fixed_num    => self.checked_add(*other).unwrap(),
            rust_decimal => self.checked_add(*other).unwrap(),
            bigdecimal   => self.add(other),
            decimal_rs   => self.checked_add(other).unwrap(),
        }
    }
}

wrapper! {
    trait CheckedSubWrapper {
        fn checked_sub_wrapper(&self, other: &Self) -> Self {
            f64 => {
                let val = self - other;
                let result = if val.is_finite() {
                    Some(val)
                } else {
                    None
                };
                result.unwrap()
            },
            fixed_num    => self.checked_sub(*other).unwrap(),
            rust_decimal => self.checked_sub(*other).unwrap(),
            bigdecimal   => self.sub(other),
            decimal_rs   => self.checked_sub(other).unwrap(),
        }
    }
}

wrapper! {
    trait CheckedMulWrapper {
        fn checked_mul_wrapper(&self, other: &Self) -> Self {
            f64 => {
                let val = self * other;
                let result = if val.is_finite() {
                    Some(val)
                } else {
                    None
                };
                result.unwrap()
            },
            fixed_num    => self.checked_mul(*other).unwrap(),
            rust_decimal => self.checked_mul(*other).unwrap(),
            bigdecimal   => self.mul(other),
            decimal_rs   => self.checked_mul(other).unwrap(),
        }
    }
}

wrapper! {
    trait CheckedDivWrapper {
        fn checked_div_wrapper(&self, other: &Self) -> Self {
            f64 => {
                let val = self / other;
                let result = if val.is_finite() {
                    Some(val)
                } else {
                    None
                };
                result.unwrap()
            },
            fixed_num    => self.checked_div(*other).unwrap(),
            rust_decimal => self.checked_div(*other).unwrap(),
            bigdecimal   => self.div(other),
            decimal_rs   => self.checked_div(other).unwrap(),
        }
    }
}

wrapper! {
    trait TruncWrapper {
        fn trunc_wrapper(&self) -> Self {
            f64          => self.trunc(),
            fixed_num    => self.trunc(),
            rust_decimal => self.trunc(),
            decimal_rs   => self.trunc(0),
        }
    }
}

wrapper! {
    trait RoundWrapper {
        fn round_wrapper(&self) -> Self {
            f64          => self.round(),
            fixed_num    => self.round(),
            rust_decimal => self.round(),
            bigdecimal   => self.round(0),
            decimal_rs   => self.round(0),
            fastnum      => self.round(0),
        }
    }
}

wrapper! {
    trait RoundToWrapper {
        fn round_to_wrapper(&self, to: i64) -> Self {
            f64          => (self * 10f64.powi(to as i32).round()) / 10f64.powi(to as i32),
            fixed_num    => self.round_to(to),
            rust_decimal => self.round_dp(to as u32),
            bigdecimal   => self.round(to),
            decimal_rs   => self.round(to as i16),
            fastnum      => self.round(to as i16),
        }
    }
}

wrapper! {
    trait Log10FloorWrapper {
        fn log10_floor_wrapper(&self) -> Self {
            f64          => self.log10().floor(),
            fixed_num    => self.unchecked_log10_floor(),
            rust_decimal => self.log10().floor(),
            decimal      => self.log10(),
            fastnum      => self.log10().floor(),
        }
    }
}

wrapper! {
    trait PowiWrapper {
        fn powi_wrapper(&self, exp: i32) -> Self {
            f64          => self.powi(exp),
            fixed_num    => self.unchecked_pow(exp),
            rust_decimal => self.powi(exp as i64),
            decimal      => self.pow(decimal::from(exp)),
            decimal_rs   => self.checked_pow(&decimal_rs::from(exp)).unwrap(),
            fastnum      => self.powi(exp),
        }
    }
}
