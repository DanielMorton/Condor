use serde::{Deserialize, Serialize};

use crate::models::photo::Photo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Taxon {
    pub id: u32,
    pub name: String,
    pub rank: String,
    pub preferred_common_name: Option<String>,
    pub default_photo: Option<Photo>,
    pub iconic_taxon_name: Option<String>,
    pub ancestor_ids: Option<Vec<u32>>,
    pub observations_count: Option<u32>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonSuggestion {
    pub taxon: Taxon,
    pub frequency: Option<f64>,
    pub vision_score: Option<f64>,
}
