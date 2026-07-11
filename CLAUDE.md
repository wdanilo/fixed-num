# fixed-num

A high-precision, high-performance fixed-point decimal library for Rust. The primary type is `Dec19x19` -- a 128-bit fixed-point decimal with exactly 19 integer digits and 19 fractional digits, stored as a scaled `i128`.

## Purpose

Designed for financial systems, trading engines, technical analysis, and any domain where floating-point drift, non-determinism, or arbitrary-precision overhead are unacceptable. Every operation is exact within the supported range (no rounding, no allocations). Published to crates.io as `fixed-num = "0.2"`.

## Project Structure

Cargo workspace with 4 published crates, 1 benchmark crate, and 2 test crates:

```
fixed-num/
  Cargo.toml                      Workspace root (edition 2024, resolver 3)
  crates/
    lib/                          Main crate: `fixed-num`
      build.rs                      Detects nightly + overflow-checks at build time
      src/
        lib.rs                      Re-exports Dec19x19, ops traits, UnwrapAll
        dec19x19.rs                 Core type + all arithmetic implementations (~2050 lines)
        ops.rs                      Trait definitions (HasMax, Abs, Checked*, Saturating*, etc.)
        i128_ops.rs                 Power-of-10 constants, digit_count(), scale_for()
        serde.rs                    Serialize as string, deserialize from string/int/float
        main.rs                     Dev scratch binary
    macro/                        Proc-macro crate: `fixed-num-macro`
      src/lib.rs                    Dec19x19!() compile-time literal macro
    helper/                       Shared helper crate: `fixed-num-helper`
      src/lib.rs                    Parsing logic, Formatter, Rand trait, ParseDec19x19Error
    validator/                    Test helper crate: `fixed-num-validator`
      src/lib.rs                    Fuzzy testing (series_pair, should_eq, check! macro)
    bench/                        Benchmarks: `fixed-num-bench`
      benches/ops.rs                Criterion benchmarks vs rust_decimal, bigdecimal, etc.
  tests/
    stable/                       Compilation test for stable Rust
    nightly/                      Compilation test for nightly Rust
```

## Internal Representation

`Dec19x19` is `#[repr(transparent)]` over `i128`. The last 19 digits are the fractional part. The scale factor is `10^19 = 10_000_000_000_000_000_000`.

- Range: `-17_014_118_346_046_923_173.168_730_371_588_410_572_8` to `+17_014_118_346_046_923_173.168_730_371_588_410_572_7` (i.e., `i128::MIN` to `i128::MAX`)
- Exact precision within the full 19+19 digit range (no rounding on add/sub)

## Public API

### Core Type

- `Dec19x19` -- the fixed-point decimal. `Copy`, `Eq`, `Ord`, `Default` (zero).
- `Dec19x19::from_repr(i128)` -- construct from raw scaled representation.
- `Dec19x19::is_zero(self) -> bool`

### Construction

- `Dec19x19!(literal)` -- compile-time parsing proc macro. Supports underscores, scientific notation (`987e-2`), negative values.
- `FromStr` / `TryFrom<&str>` / `TryFrom<String>` -- runtime parsing.
- `From<i64|i32|i16|i8|u32|u16|u8>` -- infallible integer conversion.
- `TryFrom<i128|u64|u128|f64|f32>` -- fallible conversion from wider types.
- `Dec19x19::from_i64(value)`, `from_i32(value)`, etc. -- `const` named constructors.

### Conversion Out

- `Into<f64>`, `Into<f32>`, `Into<u64>`, `Into<i128>`, `Into<u128>` -- infallible.
- `TryInto<i64|i32|u32|i16|u16|i8|u8>` -- fallible narrowing.
- `into_f64()`, `into_f32()`, `try_into_i64()`, etc. -- named method equivalents.

### Arithmetic Operators

Standard `Add`, `Sub`, `Mul`, `Div`, `Rem`, `Neg` with `+=`, `-=`, `*=`, `/=` and all `&`-ref combinations.

### Arithmetic Traits (in `fixed_num::ops`)

Each operation comes in three variants where applicable:

| Category | Unchecked (panics) | Checked (returns Option) | Saturating (clamps) |
|---|---|---|---|
| Add | `unchecked_add` | `checked_add` | `saturating_add` |
| Sub | `unchecked_sub` | `checked_sub` | `saturating_sub` |
| Mul | `unchecked_mul` | `checked_mul` | `saturating_mul` |
| Div | `unchecked_div` | `checked_div` | `saturating_div` |
| Sqrt | `unchecked_sqrt` | `checked_sqrt` | -- |
| Pow | `unchecked_pow(i32)` | `checked_pow(i32)` | -- |
| Ln | `unchecked_ln` | `checked_ln` | -- |
| Log10Floor | `unchecked_log10_floor` | `checked_log10_floor` | -- |

Other traits: `HasMax`, `HasMin`, `Signum`, `Abs`, `Trunc`, `TruncTo`, `Floor`, `FloorTo`, `Ceil`, `CeilTo`, `Round`, `RoundTo`.

The `*To` variants accept a `digits: i64` parameter for rounding to a specific number of fractional digits (negative values round integer digits).

All traits are re-exported as anonymous (`_ as`) via `fixed_num::traits`.

### Constants

- `Dec19x19::MAX`, `MIN` -- full i128 range.
- `Dec19x19::MAX_INT`, `MIN_INT` -- largest/smallest representable integers.
- `Dec19x19::SMALLEST_STEP` -- `0.000_000_000_000_000_000_1` (1 ULP).
- `Dec19x19::LN_2` -- natural log of 2, full 19-digit precision.

### Display

`Display` and `Debug` both produce decimal strings. Supports:
- Precision: `{:.4}` rounds to 4 fractional digits.
- Alternate: `{:#}` inserts underscore separators.
- Width/alignment/fill/sign: standard Rust formatting.

### Serde (feature-gated)

- Serializes as a string.
- Deserializes from strings, all integer types, and (with `serde_float` feature) from `f32`/`f64`.

## How It Works Internally

### Addition / Subtraction
Direct `i128` add/sub on the raw repr. Overflow checked via `i128::checked_add/sub`.

### Multiplication
Splits each operand into integer and fractional parts (using `/ SCALE` and `% SCALE`), performs cross-multiplication on `u128`, then reassembles. The `mul_opt` feature (default) adds fast paths when one operand has no fractional or no integer part.

### Division
Widens the dividend to `i256` (via `ethnum` or `arrow-buffer`), multiplies by `SCALE`, then divides by the `i128` divisor. Two backend options:
- `i256_ethnum` (default, slightly faster) -- `ethnum::I256`
- `i256_arrow` -- `arrow_buffer::i256`

### Square Root
Newton-Raphson in `i256` space, seeded from `f64::sqrt()`.

### Natural Logarithm
Range-reduces to `[1/sqrt(2), sqrt(2)]` by halving/doubling, then uses the atanh series `ln(x) = 2 * sum(u^(2k+1) / (2k+1))` where `u = (x-1)/(x+1)`, computed in `i256`.

### Power
Binary exponentiation (squaring). Negative exponents computed as `pow(1/base, |exp|)`.

### Log10 Floor
Uses `digit_count()` -- a branchless binary-tree comparison over the 39 powers of 10.

### Rounding (trunc/floor/ceil/round)
All operate on the raw `i128` repr using integer division by the appropriate power-of-10 scale. `round` uses a branchless sign-aware bias trick. Overflow on floor/ceil returns the original value unchanged.

### Parsing
Two parsers exist:
- `parse_dec19x19_internal` (primary) -- splits on `.` and `e`, applies exponent via `shift_decimal()`, pads/truncates the fractional part to 19 digits, combines via `int * SCALE + frac`.
- `parse_dec19x19_internal2` (experimental, WIP) -- single-pass byte parser with exponent support, still has debug output.

### Build Script
`build.rs` detects:
- `nightly` -- enables `const_trait_impl`, `step_trait` features.
- `inherit_overflow_checks` -- detects runtime overflow checking, controls whether division/ln use checked or unchecked i256 narrowing.

## Cargo Features

- `mul_opt` (default) -- optimized multiplication paths for integer-only/fraction-only operands.
- `i256_ethnum` (default) -- use `ethnum` for i256. Mutually exclusive with `i256_arrow`.
- `i256_arrow` -- use `arrow-buffer` for i256.
- `serde` -- enable serde Serialize/Deserialize.
- `serde_float` -- allow deserializing from `f32`/`f64` (implies `serde`). Not recommended; prefer arbitrary-precision string deserialization.

## Dependencies

Runtime (non-optional): `paste` (macro helper), `rand` (deterministic random generation for testing/fuzzing), `fixed-num-helper`, `fixed-num-macro`.

Optional: `ethnum` (i256), `arrow-buffer` (i256), `serde`.

Dev/test: `fixed-num-validator` (wraps `bigdecimal` for fuzzy correctness verification).

Benchmarks compare against: `rust_decimal`, `bigdecimal`, `decimal` (d128), `decimal-rs`, `fixed`, `fastnum`.

## Testing Approach

- Doc tests with the `check!` macro: tests both checked and unchecked variants, with `FAIL` for expected panics/None.
- Fuzzy validation: generates 10,000 random `Dec19x19` values via deterministic `Rand`, converts to `BigDecimal`, performs the same operation on both, and asserts string equality to 19 decimal places.
- Stable/nightly compilation tests in `tests/`.
