use std::collections::HashMap;
use crate::client::INaturalistClient;
use crate::error::Result;

/// Builder for generic GET requests to any iNaturalist API endpoint
#[derive(Debug, Clone)]
pub struct GenericGetBuilder {
    client: INaturalistClient,
    pub path: String,
    pub params: HashMap<String, String>,
}

impl GenericGetBuilder {
    pub fn new(client: INaturalistClient) -> Self {
        Self {
            client,
            path: String::new(),
            params: HashMap::new(),
        }
    }

    /// Set the API endpoint path (e.g., "observations", "taxa/123")
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Add a parameter to the request
    pub fn add_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Add multiple parameters at once
    pub fn with_params(mut self, params: HashMap<String, String>) -> Self {
        self.params.extend(params);
        self
    }

    /// Execute the GET request
    pub async fn fetch(self) -> Result<serde_json::Value> {
        self.client
            .request_handler
            .get_with_params(&self.path, &self.params, &self.client.rate_limiter)
            .await
    }
}
