//! orloglausa-io provides generic input/output support.

#![no_std]

#[cfg(feature="standard-rw")]
extern crate std;

#[cfg(feature="standard-rw")]
mod standard;

#[cfg(feature="standard-rw")]
pub use standard::*;

#[cfg(feature="binary-rw")]
mod binary;

#[cfg(feature="binary-rw")]
pub use binary::*;