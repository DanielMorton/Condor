use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledTerm {
    pub id: u32,
    pub ontology_uri: String,
    pub uri: String,
    pub is_value: bool,
    pub labels: Vec<ControlledTermLabel>,
    pub values: Option<Vec<ControlledTermValue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledTermLabel {
    pub locale: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlledTermValue {
    pub id: u32,
    pub ontology_uri: String,
    pub uri: String,
    pub labels: Vec<ControlledTermLabel>,
}
