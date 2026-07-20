use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub login: String,
    pub name: Option<String>,
    pub icon_url: Option<String>,
    pub observations_count: Option<u32>,
    pub identifications_count: Option<u32>,
    pub species_count: Option<u32>,
}
