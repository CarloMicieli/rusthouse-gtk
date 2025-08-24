//! The common library crate.
//!
//! This crate will contain shared types and utilities for the RustHouse application.

/// Returns a greeting from the common crate.
pub fn common_greeting() -> &'static str {
    "Hello from common!"
}
