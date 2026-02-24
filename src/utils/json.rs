use std::collections::HashMap;
use serde_json::Value;

/// Flatten nested JSON objects (simplified version)
pub fn flatten_dict(obj: &Value) -> HashMap<String, Value> {
    let mut result = HashMap::new();

    if let Some(object) = obj.as_object() {
        for (key, value) in object {
            match value {
                Value::Object(_) => {
                    let nested = flatten_dict(value);
                    for (nested_key, nested_value) in nested {
                        result.insert(format!("{}_{}", key, nested_key), nested_value);
                    }
                }
                _ => {
                    result.insert(key.clone(), value.clone());
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_flatten_dict() {
        let nested_json = json!({
            "name": "test",
            "user": {
                "id": 123,
                "login": "testuser"
            }
        });

        let flattened = flatten_dict(&nested_json);

        assert_eq!(flattened.get("name"), Some(&json!("test")));
        assert_eq!(flattened.get("user_id"), Some(&json!(123)));
        assert_eq!(flattened.get("user_login"), Some(&json!("testuser")));
    }

    #[test]
    fn test_flatten_simple_object() {
        let simple_json = json!({
            "a": 1,
            "b": "test"
        });

        let flattened = flatten_dict(&simple_json);

        assert_eq!(flattened.get("a"), Some(&json!(1)));
        assert_eq!(flattened.get("b"), Some(&json!("test")));
    }
}