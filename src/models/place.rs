use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    pub id: u32,
    pub name: String,
    pub display_name: String,
    pub place_type: Option<String>,
    pub bbox_area: Option<f64>,
    pub ancestor_place_ids: Option<Vec<u32>>,
    pub geometry_geojson: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default)]
pub struct PlaceSearchParams {
    pub q: Option<String>,
    pub place_type: Option<Vec<String>>,
    pub ancestor_place_id: Option<u32>,
    pub per_page: Option<usize>,
    pub extra_params: HashMap<String, String>,
}

impl PlaceSearchParams {
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        if let Some(ref q) = self.q {
            map.insert("q".to_string(), q.clone());
        }
        if let Some(ref types) = self.place_type {
            map.insert("place_type".to_string(), types.join(","));
        }
        if let Some(id) = self.ancestor_place_id {
            map.insert("ancestor_place_id".to_string(), id.to_string());
        }
        if let Some(n) = self.per_page {
            map.insert("per_page".to_string(), n.to_string());
        }

        for (k, v) in &self.extra_params {
            map.insert(k.clone(), v.clone());
        }

        map
    }
}
