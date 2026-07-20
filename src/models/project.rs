use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: Option<String>,
    pub project_type: Option<String>,
    pub slug: String,
    pub icon_url: Option<String>,
    pub banner_url: Option<String>,
    pub observed_taxa_count: Option<u32>,
    pub observations_count: Option<u32>,
    pub members_count: Option<u32>,
    pub user: Option<User>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct ProjectSearchParams {
    pub q: Option<String>,
    pub place_id: Option<Vec<u32>>,
    pub taxon_id: Option<Vec<u32>>,
    pub user_id: Option<Vec<u32>>,
    pub project_type: Option<String>,
    pub member_id: Option<u32>,
    pub rule_details: Option<bool>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub extra_params: HashMap<String, String>,
}

impl ProjectSearchParams {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(ref q) = self.q {
            map.insert("q".to_string(), q.clone());
        }
        if let Some(ref ids) = self.place_id {
            map.insert(
                "place_id".to_string(),
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(ref ids) = self.taxon_id {
            map.insert(
                "taxon_id".to_string(),
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(ref ids) = self.user_id {
            map.insert(
                "user_id".to_string(),
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(ref t) = self.project_type {
            map.insert("type".to_string(), t.clone());
        }
        if let Some(id) = self.member_id {
            map.insert("member_id".to_string(), id.to_string());
        }
        if let Some(details) = self.rule_details {
            map.insert("rule_details".to_string(), details.to_string());
        }
        if let Some(page) = self.page {
            map.insert("page".to_string(), page.to_string());
        }
        if let Some(per_page) = self.per_page {
            map.insert("per_page".to_string(), per_page.to_string());
        }

        for (k, v) in &self.extra_params {
            map.insert(k.clone(), v.clone());
        }

        map
    }
}
