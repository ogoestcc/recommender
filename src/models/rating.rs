use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rating {
    #[serde(rename = "userid")]
    user_id: u32,
    #[serde(rename = "cveid")]
    alert_id: String,
    #[serde(with = "int_to_bool")]
    like: bool,
    #[serde(with = "int_to_bool")]
    dislike: bool,
    #[serde(with = "int_to_bool")]
    critical: bool,
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
