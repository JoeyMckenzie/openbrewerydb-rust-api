//! Open Brewery DB API bindings for Rust.

#![forbid(unsafe_code)]
#![warn(
    dead_code,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts,
    clippy::single_char_pattern
)]

pub mod breweries;
pub mod client;
pub mod errors;
mod search;
