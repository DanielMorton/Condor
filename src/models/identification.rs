use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::taxon::Taxon;
use crate::models::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identification {
    pub id: u64,
    pub uuid: String,
    pub observation_id: u64,
    pub taxon_id: u32,
    pub taxon: Taxon,
    pub user: User,
    pub body: Option<String>,
    pub current: bool,
    pub category: Option<String>,
    pub disagreement: Option<bool>,
    pub previous_observation_taxon_id: Option<u32>,
    pub vision: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct IdentificationSearchParams {
    pub observation_id: Option<Vec<u64>>,
    pub taxon_id: Option<Vec<u32>>,
    pub user_id: Option<Vec<u32>>,
    pub current: Option<bool>,
    pub category: Option<Vec<String>>,
    pub own_observation: Option<bool>,
    pub is_change: Option<bool>,
    pub order: Option<String>,
    pub order_by: Option<String>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
    pub extra_params: HashMap<String, String>,
}

impl IdentificationSearchParams {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(ref ids) = self.observation_id { map.insert("observation_id".to_string(), ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")); }
        if let Some(ref ids) = self.taxon_id { map.insert("taxon_id".to_string(), ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")); }
        if let Some(ref ids) = self.user_id { map.insert("user_id".to_string(), ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",")); }
        if let Some(current) = self.current { map.insert("current".to_string(), current.to_string()); }
        if let Some(ref categories) = self.category { map.insert("category".to_string(), categories.join(",")); }
        if let Some(own) = self.own_observation { map.insert("own_observation".to_string(), own.to_string()); }
        if let Some(change) = self.is_change { map.insert("is_change".to_string(), change.to_string()); }
        if let Some(ref order) = self.order { map.insert("order".to_string(), order.clone()); }
        if let Some(ref order_by) = self.order_by { map.insert("order_by".to_string(), order_by.clone()); }
        if let Some(page) = self.page { map.insert("page".to_string(), page.to_string()); }
        if let Some(per_page) = self.per_page { map.insert("per_page".to_string(), per_page.to_string()); }

        for (k, v) in &self.extra_params {
            map.insert(k.clone(), v.clone());
        }

        map
    }
}
