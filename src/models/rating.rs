use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rating {
    #[serde(rename = "userid")]
    pub user_id: u32,
    #[serde(rename = "cveid")]
    pub alert_id: String,
    #[serde(with = "int_to_bool")]
    pub like: bool,
    #[serde(with = "int_to_bool")]
    pub dislike: bool,
    #[serde(with = "int_to_bool")]
    pub critical: bool,
}


mod int_to_bool {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    { 
        Ok(u16::deserialize(deserializer)? == 1u16)
    }
}
