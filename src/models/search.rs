use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::place::Place;
use crate::models::project::Project;
use crate::models::taxon::Taxon;
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub score: f64,
    #[serde(rename = "type")]
    pub result_type: String,
    pub matches: Vec<String>,
    pub record: SearchRecord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchRecord {
    Taxon(Taxon),
    Place(Place),
    Project(Project),
    User(User),
}

#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub q: String,
    pub sources: Option<Vec<String>>,
    pub per_page: Option<usize>,
    pub page: Option<usize>,
    pub extra_params: HashMap<String, String>,
}

impl SearchParams {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        map.insert("q".to_string(), self.q.clone());
        if let Some(ref sources) = self.sources { map.insert("sources".to_string(), sources.join(",")); }
        if let Some(n) = self.per_page { map.insert("per_page".to_string(), n.to_string()); }
        if let Some(page) = self.page { map.insert("page".to_string(), page.to_string()); }

        for (k, v) in &self.extra_params {
            map.insert(k.clone(), v.clone());
        }

        map
    }
}
