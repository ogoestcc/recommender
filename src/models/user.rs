
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "user_id")]
    id: u32,
    #[serde(with = "preferences")]
    preferences: Vec<String>,
    ratings: Option<Vec<super::rating::Rating>>,
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

