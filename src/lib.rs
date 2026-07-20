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
    ControlledTerm, Identification, IdentificationSearchParams, Observation,
    ObservationHistogramStats, ObservationIdentifierStats, ObservationObserverStats,
    ObservationSearchParams, ObservationSpeciesStats, Place, PlaceSearchParams, Project,
    ProjectSearchParams, SearchParams, SearchResult, Taxon, TaxonSuggestion, User,
};

// Re-export commonly used types
pub use std::collections::HashMap;
