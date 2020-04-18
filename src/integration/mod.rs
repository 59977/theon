//! Integration of external crates and foreign types.
//!
//! This module provides implementations of traits in Theon for foreign types.
//! Integrated crates are re-exported within a sub-module, which can be used to
//! avoid versioning errors.
//!
//! Re-exported types are hidden in Theon's documentation. Refer to the
//! documentation for integrated crates at the corresponding version.

// Feature modules. These are empty unless Cargo features are enabled.
pub mod cgmath;
pub mod mint;
pub mod nalgebra;
