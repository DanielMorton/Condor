use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Taxon {
    pub id: i64,
    pub name: String,
    pub rank: String,
    pub rank_level: Option<f64>,
    pub preferred_common_name: Option<String>,
    pub iconic_taxon_name: Option<String>,
    pub is_active: bool,
    pub ancestor_ids: Vec<i64>,
    pub observations_count: Option<i64>,
    pub default_photo: Option<TaxonPhoto>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaxonPhoto {
    pub id: i64,
    pub url: String,
    pub attribution: Option<String>,
    pub license_code: Option<String>,
}