use serde::{Deserialize, Deserializer, Serializer};
use serde::de::Error;

/// Deserialize a string that might be a number
pub fn string_or_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde_json::Value;
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(s),
        Value::Number(n) => Ok(n.to_string()),
        _ => Err(D::Error::custom("expected string or number")),
    }
}

/// Deserialize an optional string that might be null or missing
pub fn optional_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde_json::Value;
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::Null => Ok(None),
        Value::String(s) if s.is_empty() => Ok(None),
        Value::String(s) => Ok(Some(s)),
        _ => Ok(None),
    }
}

/// Serialize optional values, skipping if None
pub fn skip_if_none<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: serde::Serialize,
{
    match value {
        Some(v) => v.serialize(serializer),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct TestStruct {
        #[serde(deserialize_with = "string_or_number")]
        value: String,
    }

    #[test]
    fn test_string_or_number_with_string() {
        let json = r#"{"value": "test"}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "test");
    }

    #[test]
    fn test_string_or_number_with_number() {
        let json = r#"{"value": 123}"#;
        let result: TestStruct = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, "123");
    }
}