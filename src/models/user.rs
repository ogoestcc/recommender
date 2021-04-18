use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "user_id")]
    pub id: u32,
    #[serde(with = "preferences")]
    pub preferences: Vec<String>,
    pub ratings: Option<HashMap<String, i32>>,
    pub similarity: Option<Vec<(u32, f32)>>,
}

mod preferences {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?
            .replace(&['[', ']', '\'', ' '][..], "")
            .split(',')
            .map(|s| s.into())
            .collect())
    }
}
