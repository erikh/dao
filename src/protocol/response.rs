use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response {
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub timestamp: NaiveDateTime,
    pub payload: HashMap<String, String>,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string(self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_to_string() -> Result<()> {
        let mut payload = HashMap::default();

        payload.insert("testing".to_string(), "payload".to_string());

        let table = vec![
            (
                Response {
                    status: true,
                    error: None,
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload: Default::default(),
                },
                "{\"status\":true,\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{}}",
                "basic response",
            ),
            (
                Response {
                    status: true,
                    error: Some(String::from("this is an error")),
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload: Default::default(),
                },
                "{\"status\":true,\"error\":\"this is an error\",\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{}}",
                "basic response with error",
            ),
            (
                Response {
                    status: true,
                    error: None,
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload: payload.clone(),
                },
                "{\"status\":true,\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{\"testing\":\"payload\"}}",
                "basic response with payload",
            ),
            (
                Response {
                    status: true,
                    error: Some(String::from("this is an error")),
                    timestamp: NaiveDateTime::UNIX_EPOCH,
                    payload,
                },
                "{\"status\":true,\"error\":\"this is an error\",\"timestamp\":\"1970-01-01T00:00:00\",\"payload\":{\"testing\":\"payload\"}}",
                "basic response with error and payload",
            ),
        ];

        for (input, result, annotation) in table {
            assert_eq!(input.to_string(), result, "{}", annotation);
        }

        Ok(())
    }
}
