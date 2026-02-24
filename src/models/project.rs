use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub slug: String,
    pub project_type: Option<String>,
    pub icon: Option<String>,
}