//! This crate contains tools and data structures to import, process, transform and export
//! distance fields.

/// Utility module with helper methods.
mod utils;

/// Core data structures for distance fields
pub mod data;

pub mod distance;

pub mod generator;

pub mod processor;

/// Module for export related types and functionality
pub mod export;

pub mod input;