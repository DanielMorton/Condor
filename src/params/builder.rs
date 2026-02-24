use std::collections::HashMap;
use crate::error::{ClientError, Result};

/// Generic parameter builder with validation
pub struct ParamBuilder {
    params: HashMap<String, String>,
}

impl ParamBuilder {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    /// Add a string parameter
    pub fn add<K: Into<String>, V: ToString>(mut self, key: K, value: V) -> Self {
        self.params.insert(key.into(), value.to_string());
        self
    }

    /// Add an optional parameter
    pub fn add_optional<K: Into<String>, V: ToString>(
        mut self,
        key: K,
        value: Option<V>,
    ) -> Self {
        if let Some(v) = value {
            self.params.insert(key.into(), v.to_string());
        }
        self
    }

    /// Add a boolean parameter
    pub fn add_bool<K: Into<String>>(mut self, key: K, value: bool) -> Self {
        self.params.insert(key.into(), value.to_string());
        self
    }

    /// Add multiple values as comma-separated
    pub fn add_array<K: Into<String>, V: ToString>(
        mut self,
        key: K,
        values: Vec<V>,
    ) -> Self {
        if !values.is_empty() {
            let joined = values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            self.params.insert(key.into(), joined);
        }
        self
    }

    /// Add pagination parameters
    pub fn paginate(self, page: u32, per_page: u32) -> Self {
        self.add("page", page).add("per_page", per_page)
    }

    /// Add date range parameters
    pub fn date_range(self, start: &str, end: &str) -> Self {
        self.add("d1", start).add("d2", end)
    }

    /// Add created date range
    pub fn created_date_range(self, start: &str, end: &str) -> Self {
        self.add("created_d1", start).add("created_d2", end)
    }

    /// Build the parameter map
    pub fn build(self) -> HashMap<String, String> {
        self.params
    }

    /// Validate parameters (can be extended with custom validators)
    pub fn validate(&self) -> Result<()> {
        // Add validation logic here
        if let Some(per_page) = self.params.get("per_page") {
            let per_page_val: u32 = per_page.parse()
                .map_err(|_| ClientError::Parse("Invalid per_page value".to_string()))?;
            if per_page_val > 200 {
                return Err(ClientError::InvalidResponse(
                    "per_page cannot exceed 200".to_string()
                ));
            }
        }
        Ok(())
    }
}

impl Default for ParamBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_builder() {
        let params = ParamBuilder::new()
            .add("place_id", "6803")
            .add("taxon_id", "47126")
            .add_bool("verifiable", true)
            .paginate(1, 50)
            .build();

        assert_eq!(params.get("place_id"), Some(&"6803".to_string()));
        assert_eq!(params.get("taxon_id"), Some(&"47126".to_string()));
        assert_eq!(params.get("verifiable"), Some(&"true".to_string()));
        assert_eq!(params.get("per_page"), Some(&"50".to_string()));
    }

    #[test]
    fn test_array_params() {
        let params = ParamBuilder::new()
            .add_array("place_id", vec![6803, 6804, 6805])
            .build();

        assert_eq!(params.get("place_id"), Some(&"6803,6804,6805".to_string()));
    }
}