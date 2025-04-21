<img width="698" alt="Image" src="https://github.com/user-attachments/assets/d7628fc6-4d32-446d-9c2c-37e371cf3c0c" />

<br/>

# ⚙️ Introduction

`Dec19x19` is a high-precision, high-performance fixed-point decimal type for Rust.

It is designed for environments where accuracy, determinism, and raw speed are non-negotiable:
financial systems, trading engines, technical analysis, backtesting, and anywhere floating-point
drift or arbitrary-precision overhead are unacceptable.

Internally, values are stored as `i128` integers with the last 19 digits interpreted as the
fractional part. This allows all operations to perform without rounding or approximations within
the full range of exactly 19 fractional and 19 integer digits:

```text
±9_999_999_999_999_999_999.999_999_999_999_999_999_9
```

The actual representable bounds are wider:

- Max: `+17_014_118_346_046_923_173.168_730_371_588_410_572_7`
- Min: `−17_014_118_346_046_923_173.168_730_371_588_410_572_8`

Overflow is safely handled via `checked_*` and `saturating_*` variants.

<br/>

# 🧭 When to Use `Dec19x19`

`Dec19x19` is built for applications where exact decimal precision, deterministic behavior, and
high performance are essential. This includes financial software, trading platforms,
simulations, and systems where rounding errors are unacceptable.

In many domains—especially finance and crypto—values are represented using fixed decimal
precision. For example, most cryptocurrencies (including Ethereum) define token values using 18
decimal places, meaning the smallest unit is `10^-18`. Floating-point numbers (`f64`, `f32`)
can't accurately represent these values without precision loss, and results can vary across
architectures due to non-deterministic behavior.

Unlike floating-point types, `Dec19x19` guarantees:

- **No rounding drift.** Every operation is exact within the supported range.
- **Deterministic results.** Same input always produces the same output.
- **Consistent decimal scale.** Precisely 19 digits after the decimal, matching or exceeding
  most domain requirements (e.g. Ethereum's 18).
- **No hidden costs.** All operations are performed using fast, predictable `i128` arithmetic
  with zero allocations.

Use `Dec19x19` when:

- You need to model money, token balances, or prices with exact decimal behavior.
- Performance matters, and you can't afford the overhead of heap-based decimal libraries.
- You require deterministic and audit-friendly results (e.g. blockchain logic, financial
  reconciliation).
- You want a wide range type, e.g. for time. If you interpret `Dec19x19!(1)` as 1 millisecond,
  it covers the range of 100 million years with precision down to the zeptosecond (travel time
  of a photon across a hydrogen molecule = 247 zeptoseconds).

In short, `Dec19x19` provides the precision of big-decimal, performance close to primitive
types, and the reliability that floating-point types can't offer.

<br/>

# ✅ Features

- 🔬 **Exact Decimal Precision**<br/>
  Fixed 19 integer + 19 fractional digits. No approximations.

- 🛡️ **Safety by Default**<br/>
  Checked and saturating arithmetic built in. No panics, no surprises.

- 🚀 **High Performance**<br/>
  Lean `i128` math optimized for speed. Compiles to minimal instructions.

- 🧪 **Proven Correctness**<br/>
  Verified via extensive tests, fuzzing, and comparison with other crates.

- 🧱 **Clean and simple implementation**<br/>
  Internally just a scaled `i128`. Easy to audit, maintain, and extend.

<br/>

# 🛠️ Usage

Construct values via:
- `Dec19x19!(...)` macro to parse a decimal literal at compile time,
- `Dec19x19::from_str(...)` method to parse a decimal literal at runtime,
- `Dec19x19::from(...)` method to convert from other narrower types,
- `Dec19x19::try_from(...)` method to convert from other wider types.

```rust
use fixed_num::Dec19x19 as Dec;
use std::str::FromStr;

let price = Dec::from(123_u8) + Dec!(0.456);
let fee = Dec!(1e-3);
let total = price + fee;

assert_eq!(Ok(total), Dec::from_str("123.457"));
assert_eq!(format!("{total}"), "123.457");
```

<br/>

You can print `Dec19x19` values using the `Display` trait with all the usual formatting options:

```rust
use fixed_num::Dec19x19 as Dec;

let dec = Dec!(7_654_321.123_456_7);
assert_eq!(&format!("{dec}"),          "7654321.1234567");

// Human-readable form.
assert_eq!(&format!("{dec:#}"),        "7_654_321.123_456_7");

// Precision rounding.
assert_eq!(&format!("{dec:.0}"),       "7654321");
assert_eq!(&format!("{dec:.1}"),       "7654321.1");
assert_eq!(&format!("{dec:.2}"),       "7654321.12");
assert_eq!(&format!("{dec:.3}"),       "7654321.123");
assert_eq!(&format!("{dec:.4}"),       "7654321.1235");
assert_eq!(&format!("{dec:#.0}"),      "7_654_321");
assert_eq!(&format!("{dec:#.4}"),      "7_654_321.123_5");
assert_eq!(&format!("{dec:#.19}"),     "7_654_321.123_456_700_000_000_000_0");

// Padding and alignment.
assert_eq!(&format!("{dec:#24}"),      "     7_654_321.123_456_7");
assert_eq!(&format!("{dec:<#24}"),     "     7_654_321.123_456_7");
assert_eq!(&format!("{dec:>#24}"),     "7_654_321.123_456_7     ");
assert_eq!(&format!("{dec:^#24}"),     "  7_654_321.123_456_7   ");
assert_eq!(&format!("{dec:^#24.4}"),   "    7_654_321.123_5     ");
assert_eq!(&format!("{dec:_^#24.4}"),  "____7_654_321.123_5_____");

// Explicit + sign display.
assert_eq!(&format!("{dec:^+#24.4}"),  "    +7_654_321.123_5    ");
```

<br/>

We highly recommend setting `overflow-checks = true` in your `Cargo.toml`, especially when 
developing financial or precision-critical libraries. This option introduces minimal overhead in 
release builds, but it can be invaluable for catching arithmetic bugs during development. For best 
results, we suggest the following configuration:

```toml
# Cargo.toml
[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
overflow-checks = true
strip = true
```

```toml
# .cargo/config.toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

<br/>

# 🛠️ Operations

Dec19x19 implements most standard arithmetic operations. The list of supported operations can be
found [here](https://docs.rs/fixed-num/latest/fixed_num/ops/index.html). Operations that never panic
are marked with the ✅ symbol.

<br/>

# 📊 Comparison to Other Libraries

|                                              | `fixed_num`                           | `rust_decimal`          | `bigdecimal` | `decimal`                                                   | `decimal_rs`             | `fixed`                                        | `fastnum`                            |
|----------------------------------------------|---------------------------------------|-------------------------|--------------|-------------------------------------------------------------|--------------------------|------------------------------------------------|--------------------------------------|
| 100% Rust                                    | ✅                                     | ✅                       | ✅            | ❌                                                           | ✅                        | ✅                                              | ✅                                    |
| Size (bits)                                  | 128                                   | 128                     | dynamic      | 128                                                         | 160                      | Configurable                                   | 64/128/256/512/..                    |
| Underlying repr                              | `i128`                                | 4 x `u32`               | `Vec<u64>`   | `[u8; 16]`                                                  | `(u128, i16, bool, u8)`  | Configurable                                   | Configurable                         |
| Arbitrary precision                          | ❌                                     | ❌                       | ✅            | ❌                                                           | ❌                        | ❌                                              | ⚠️ (chosen during compilation)       |
| Decimal fixed-point precision                | ✅                                     | ✅                       | ✅            | ✅                                                           | ✅                        | ❌                                              | ✅       |
| Precision                                    | 38 digits, 19 before and 19 after dot | 28 digits, dot anywhere | Infinite     | 34 digits, dot anywhere. More digits with round-off errors. | 38 digits, dot anywhere. | Configurable decimal and fractional bit count. | Infinite (chosen during compilation) |
| Copyable                                     | ✅                                     | ✅                       | ❌            | ✅                                                           | ✅                        | ✅                                              | ✅                                    |
| Const exprs                                  | ✅                                     | ❌                       | ❌            | ❌                                                           | ❌                        | ❌                                              | ✅                                    |
| No round-off errors (e.g. `0.1 + 0.2 ≠ 0.3`) | ✅                                     | ✅                       | ✅            | ✅ Up to 34 digits.                                          | ✅ Up to 38 digits.       | ❌                                              | ✅                                    |
| `±0`, `±Infinity`, `NaN`                     | ❌                                     | ❌                       | ❌            | ✅                                                           | ❌                        | ❌                                              | ✅                                    |


### Max Precision
- [rust_decimal]: `Decimal` represents a 128 bit representation of a fixed-precision decimal number. 
  The finite set of values of type Decimal are of the form `m / 10^e`, where `m` is an integer such 
  that `-2^96 < m < 2^96`, and `e` is an integer between `0` and `28` inclusive. So, `m` is in range
  `-79_228_162_514_264_337_593_543_950_336` to `79_228_162_514_264_337_593_543_950_336`, which gives
  28 full digits and the dot can be placed anywhere in the number.
- [decimal](https://github.com/alkis/decimal?tab=readme-ov-file): The library provides d128 which is 
  a [128-bit decimal floating point number](https://en.wikipedia.org/wiki/Decimal128_floating-point_format). 
  

<br/>

# 🚀 Benchmarks (higher is better)

Benchmarks measure normalized throughput. `1.00` = `Dec19x19` baseline.

- `1.25` means 25% faster
- `0.50` means 2× slower
- `⚠️` indicates unsupported or panicking behavior

<table class="benchmark-table">
  <thead>
  <tr>
    <th></th>
    <th>f64</th>
    <th>fixed_num</th>
    <th>rust_decimal</th>
    <th>bigdecimal</th>
    <th>decimal</th>
    <th>decimal_rs</th>
    <th>fastnum</th>
  </tr>
  </thead>
  <tbody>
  <tr>
    <td>eq</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 91%, #c41c0d); font-weight: normal;">0.82</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 2%, #c41c0d); font-weight: normal;">0.10</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.01</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 19%, #c41c0d); font-weight: normal;">0.15</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.09</td>
  </tr>
  <tr>
    <td>ord</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">1.48</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 8%, #c41c0d); font-weight: normal;">0.12</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 30%, #c41c0d); font-weight: normal;">0.20</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.09</td>
  </tr>
  <tr>
    <td>signum</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">1.40</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 59%, #c41c0d); font-weight: normal;">0.39</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.03</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 70%, #c41c0d); font-weight: normal;">0.50</td>
  </tr>
  <tr>
    <td>neg</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">1.59</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 96%, #c41c0d); font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 88%, #c41c0d); font-weight: normal;">0.83</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.03</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.05</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 78%, #c41c0d); font-weight: normal;">0.66</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.10</td>
  </tr>
  <tr>
    <td>abs</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">2.01</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 96%, #c41c0d); font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.10</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.03</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.06</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 87%, #c41c0d); font-weight: normal;">0.82</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 95%, #c41c0d); font-weight: normal;">0.98</td>
  </tr>
  <tr>
    <td>rem</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 63%, #c41c0d); font-weight: normal;">0.42</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 83%, #c41c0d); font-weight: normal;">0.67</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.04</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 87%, #c41c0d); font-weight: normal;">0.74</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 39%, #c41c0d); font-weight: normal;">0.24</td>
  </tr>
  <tr>
    <td>add</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">1.24</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.05</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.01</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
  </tr>
  <tr>
    <td>sub</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">1.19</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.05</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.01</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
  </tr>
  <tr>
    <td>mul_fxf</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">63.30</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 97%, #c41c0d);font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.07</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 43%, #c41c0d);font-weight: normal;">0.29</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 61%, #c41c0d);font-weight: normal;">0.44</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 41%, #c41c0d);font-weight: normal;">0.28</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 46%, #c41c0d);font-weight: normal;">0.31</td>
  </tr>
  <tr>
    <td>mul_fxi</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">15.72</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 88%, #c41c0d);font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.32</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.08</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.12</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 26%, #c41c0d);font-weight: normal;">0.24</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 42%, #c41c0d);font-weight: normal;">0.34</td>
  </tr>
  <tr>
    <td>mul_ixi</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">15.94</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 56%, #c41c0d);font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 99%, #c41c0d);font-weight: normal;">2.71</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.09</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.17</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">2.76</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 9%, #c41c0d);font-weight: normal;">0.34</td>
  </tr>
  <tr>
    <td>div</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">61.03</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 97%, #c41c0d);font-weight: normal;">0.93</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.01</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 27%, #c41c0d);font-weight: normal;">0.19</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 30%, #c41c0d);font-weight: normal;">0.20</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.04</td>
  </tr>
  <tr>
    <td>checked_add</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">1.05</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.05</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>checked_sub</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">1.01</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.05</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.02</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>checked_mul_fxf</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">51.52</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 94%, #c41c0d);font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.15</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 41%, #c41c0d);font-weight: normal;">0.30</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 38%, #c41c0d);font-weight: normal;">0.28</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>checked_mul_fxi</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">14.29</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 61%, #c41c0d);font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">2.46</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.08</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 3%, #c41c0d);font-weight: normal;">0.27</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>checked_mul_ixi</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">14.25</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 43%, #c41c0d);font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">3.69</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.10</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 93%, #c41c0d);font-weight: normal;">3.15</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>checked_div</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">46.74</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 85%, #c41c0d);font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.42</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.01</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 23%, #c41c0d);font-weight: normal;">0.24</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>trunc</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">19.19</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 81%, #c41c0d); font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 45%, #c41c0d); font-weight: normal;">0.43</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.54</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>floor</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">23.40</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 78%, #c41c0d); font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 24%, #c41c0d); font-weight: normal;">0.29</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.67</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.14</td>
  </tr>
  <tr>
    <td>ceil</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">23.20</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 80%, #c41c0d); font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 26%, #c41c0d); font-weight: normal;">0.28</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.57</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.13</td>
  </tr>
  <tr>
    <td>round</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">60.69</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 37%, #c41c0d); font-weight: normal;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.16</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">4.26</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 45%, #c41c0d); font-weight: normal;">1.19</td>
  </tr>
  <tr>
    <td>powi</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">3260.83</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 77%, #c41c0d);font-weight: normal;">0.59</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 33%, #c41c0d);font-weight: normal;">0.21</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 55%, #c41c0d);font-weight: normal;">0.36</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">0.99</td>
  </tr>
  <tr>
    <td>sqrt</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: normal;">245.36</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.05</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.05</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.03</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
  </tr>
  <tr>
    <td>ln</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">361.46</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 15%, #c41c0d);font-weight: normal;">0.14</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.01</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 26%, #c41c0d);font-weight: normal;">0.18</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
  </tr>
  <tr>
    <td>log10_floor</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 88%, #c41c0d); font-weight: normal;">0.75</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 100%, #c41c0d); font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
    <td style="color: rgba(255, 255, 255, 1.0); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5); background-color: color-mix(in lch, #58760b 0%, #c41c0d); font-weight: normal;">0.00</td>
  </tr>
  <tr>
    <td>rolling_window</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: normal;">15.51</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 100%, #c41c0d);font-weight: bold;">1.00</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 66%, #c41c0d);font-weight: normal;">0.46</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">0.10</td>
    <td style="color: rgba(255, 255, 255, 1);background-color: color-mix(in lch, #58760b 0%, #c41c0d);font-weight: normal;">⚠️</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 26%, #c41c0d);font-weight: normal;">0.18</td>
    <td style="color: rgba(255, 255, 255, 0.5);background-color: color-mix(in lch, #58760b 58%, #c41c0d);font-weight: normal;">0.38</td>
  </tr>
  </tbody>
</table>

⚠️ **Note:** The `fixed` crate was excluded due to frequent panics during arithmetic operations
in benchmarks.
