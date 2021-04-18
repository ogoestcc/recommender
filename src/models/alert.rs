use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Alert {
    #[serde(rename = "cveid")]
    pub id: String,
    pub cvss_score: f32,
    pub provider: String,
    pub product: String,
    pub description: String,
    pub score: Option<f32>,
}