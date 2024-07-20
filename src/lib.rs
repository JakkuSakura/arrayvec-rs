//! **arrayvec** provides the types [`ArrayVec`] and [`ArrayString`]:
//! array-backed vector and string types, which store their contents inline.
//!
//! The arrayvec package has the following cargo features:
//!
//! - `std`
//!   - Optional, enabled by default
//!   - Use libstd; disable to use `no_std` instead.
//!
//! - `serde`
//!   - Optional
//!   - Enable serialization for ArrayVec and ArrayString using serde 1.x
//!
//! - `zeroize`
//!   - Optional
//!   - Implement `Zeroize` for ArrayVec and ArrayString
//!
//! ## Rust Version
//!
//! This version of arrayvec requires Rust 1.51 or later.
//!
#![doc(html_root_url = "https://docs.rs/arrayvec/0.7/")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(specialization))]

#[cfg(feature = "serde")]
extern crate serde;

#[cfg(not(feature = "std"))]
extern crate core as std;

mod array_string;
mod arrayvec;
mod arrayvec_impl;
mod char;
mod errors;
mod len_type;
mod utils;

pub use crate::array_string::ArrayString;
pub use crate::errors::CapacityError;
pub use len_type::LenUint;

pub use crate::arrayvec::{ArrayVec, Drain, IntoIter};
