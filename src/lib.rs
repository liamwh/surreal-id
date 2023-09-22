#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

mod errors;
mod new_id;

pub use errors::IdError;

pub use new_id::NewId;
