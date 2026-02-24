use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub total_results: Option<i64>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub results: T,
}

// Specific response types
pub type HistogramResponse = ApiResponse<serde_json::Value>;
pub type ObserversResponse = ApiResponse<Vec<crate::models::ObserverStats>>;
pub type IdentifiersResponse = ApiResponse<Vec<crate::models::ObserverStats>>;
pub type SpeciesCountsResponse = ApiResponse<Vec<crate::models::SpeciesStats>>;
pub type IdentificationsResponse = ApiResponse<Vec<serde_json::Value>>;
pub type TaxaResponse = ApiResponse<Vec<serde_json::Value>>;
pub type PlacesResponse = ApiResponse<Vec<serde_json::Value>>;
pub type ProjectsResponse = ApiResponse<Vec<serde_json::Value>>;
pub type UsersResponse = ApiResponse<Vec<serde_json::Value>>;
pub type SearchResponse = ApiResponse<serde_json::Value>;