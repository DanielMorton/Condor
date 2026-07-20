//! iNaturalist API Client Library
//!
//! A Rust client for interacting with the iNaturalist API with rate limiting,
//! retry logic, parallel processing, and auto-pagination capabilities.

pub mod api;
pub mod cli;
pub mod client;
pub mod error;
pub mod models;
pub mod utils;

// Re-export main types for easier use
pub use client::INaturalistClient;
pub use error::{ClientError, Result};
pub use models::{
    ObservationHistogramStats, ObservationObserverStats, ObservationSpeciesStats,
    ObservationIdentifierStats, Observation, ObservationSearchParams,
    Identification, IdentificationSearchParams,
    Taxon, TaxonSuggestion, Place, PlaceSearchParams,
    Project, ProjectSearchParams, User, SearchResult, SearchParams,
    ControlledTerm,
};

// Re-export commonly used types
pub use std::collections::HashMap;
