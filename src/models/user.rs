use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub icon: Option<String>,
    pub icon_url: Option<String>,
    pub observations_count: Option<i64>,
    pub identifications_count: Option<i64>,
    pub species_count: Option<i64>,
    pub created_at: Option<String>,
}