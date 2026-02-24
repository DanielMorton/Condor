use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Place {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub admin_level: Option<i32>,
    pub location: Option<String>,
    pub bbox_area: Option<f64>,
}