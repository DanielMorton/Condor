use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: u64,
    pub url: Option<String>,
    pub medium_url: Option<String>,
    pub large_url: Option<String>,
    pub thumb_url: Option<String>,
    pub square_url: Option<String>,
    pub original_url: Option<String>,
    pub license_code: Option<String>,
    pub attribution: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sound {
    pub id: u64,
    pub file_url: String,
    pub license_code: Option<String>,
    pub attribution: Option<String>,
}
