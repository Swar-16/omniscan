//! OMNISCAN: Local-first, configurable Optical Mark Recognition (OMR) engine.
//!
//! This library crate exposes the foundational domain types, errors, CLI definitions,
//! normalized layout templates, dataset manifests, and ground-truth parsers.

pub mod cli;
pub mod core;
pub mod error;
pub mod labels;
pub mod layout;
pub mod manifest;

pub use error::{OmniscanError, Result};
pub use layout::Template;
