//! # driftwood
//! Some logs on the [tide](https://github.com/http-rs/tide)

#![allow(clippy::print_literal)]
#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_docs,
    unreachable_pub,
    missing_copy_implementations,
    unused_qualifications
)]

pub(crate) struct LogMiddlewareHasBeenRun;

mod apache_combined;
mod apache_common;
mod dev;

pub use apache_combined::ApacheCombinedLogger;
pub use apache_common::ApacheCommonLogger;
pub use dev::DevLogger;
