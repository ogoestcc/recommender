use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Alert {
    #[serde(rename = "cveid")]
    id: String,
    cvss_score: f32,
    provider: String,
    product: String,
    description: String,
}