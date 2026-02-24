//! iNaturalist API Client Library
//!
//! A comprehensive Rust client for the iNaturalist API with macro-generated endpoints,
//! rate limiting, retry logic, and parallel processing capabilities.

pub mod client;
pub mod models;
pub mod api;
pub mod utils;
pub mod error;
pub mod cli;
pub mod macros;
pub mod params;
pub mod endpoints;

// Re-export main types for easier use
pub use client::INaturalistClient;
pub use models::stats::{LocationStats, ObserverStats, SpeciesStats};
pub use models::response::*;
pub use error::{ClientError, Result};
pub use params::common::*;

// Re-export commonly used types
pub use std::collections::HashMap;