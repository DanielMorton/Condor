use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identification {
    pub id: i64,
    pub body: Option<String>,
    pub category: Option<String>,
    pub created_at: String,
    pub current: bool,
    pub taxon: Option<IdentificationTaxon>,
    pub user: Option<IdentificationUser>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdentificationTaxon {
    pub id: i64,
    pub name: String,
    pub rank: String,
    pub preferred_common_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdentificationUser {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
}