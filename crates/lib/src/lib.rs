//! <style>
//!   table {
//!     --border-color: var(--main-background-color)
//!   }
//! </style>
//!
//! <style>
//!   .benchmark-table td:nth-child(2),
//!   .benchmark-table th:nth-child(2),
//!   .benchmark-table td:nth-child(3),
//!   .benchmark-table th:nth-child(3) {
//!     border-right: 4px solid var(--border-color)
//!   }
//! </style>
#![doc = include_str!("../README.md")]
#![cfg_attr(nightly, feature(const_trait_impl))]
#![cfg_attr(nightly, feature(step_trait))]


#[allow(unused_extern_crates)]
extern crate self as fixed_num;

pub mod ops;
pub mod dec19x19;
pub mod i128_ops;

pub use dec19x19::Dec19x19;

// ==============
// === Traits ===
// ==============

pub mod traits {
    pub use crate::ops::*;
    pub use fixed_num_helper::Rand;
}
pub use traits::*;

// =================
// === UnwrapAll ===
// =================

pub trait UnwrapAll {
    type Output;
    fn unwrap_all(self) -> Self::Output;
}

impl<T> UnwrapAll for Option<T> {
    type Output = T;
    fn unwrap_all(self) -> Self::Output {
        #[expect(clippy::unwrap_used)]
        self.unwrap()
    }
}

impl UnwrapAll for Dec19x19 {
    type Output = Self;
    fn unwrap_all(self) -> Self::Output {
        self
    }
}
