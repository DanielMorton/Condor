use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Observation {
    pub id: i64,
    pub observed_on: Option<String>,
    pub quality_grade: Option<String>,
    pub taxon: Option<ObservationTaxon>,
    pub user: Option<ObservationUser>,
    pub place_guess: Option<String>,
    pub location: Option<String>,
    pub positional_accuracy: Option<i32>,
    pub photos: Vec<ObservationPhoto>,
    pub sounds: Vec<ObservationSound>,
    pub identifications_count: Option<i32>,
    pub comments_count: Option<i32>,
    pub faves_count: Option<i32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationTaxon {
    pub id: i64,
    pub name: String,
    pub rank: String,
    pub preferred_common_name: Option<String>,
    pub iconic_taxon_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationUser {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationPhoto {
    pub id: i64,
    pub url: String,
    pub attribution: Option<String>,
    pub license_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObservationSound {
    pub id: i64,
    pub attribution: Option<String>,
    pub license_code: Option<String>,
}