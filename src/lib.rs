#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]
#![deny(clippy::all)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

mod errors;
mod new_id;

pub use errors::IdError;

pub use new_id::NewId;
