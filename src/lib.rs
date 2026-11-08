//! OMNISCAN: Local-first, configurable Optical Mark Recognition (OMR) engine.
//!
//! This library crate exposes the foundational domain types, errors, CLI definitions,
//! and initial module boundaries for the OMNISCAN platform.

pub mod cli;
pub mod error;

pub use error::{OmniscanError, Result};

