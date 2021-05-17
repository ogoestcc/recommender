use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::services::types::alerts;

#[derive(Debug, Deserialize, Serialize, Clone, PartialOrd)]
pub struct Alert {
    #[serde(rename = "cveid")]
    pub id: String,
    pub cvss_score: f32,
    pub provider: String,
    pub product: String,
    pub description: String,
    pub score: Option<f32>,
}

impl Alert {
    pub fn filter_content(&self, content: &String) -> bool {
        &self.product == content || &self.provider == content
    }
}

impl PartialEq for Alert {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Alert {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Alert {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score > other.score {
            Ordering::Greater
        } else if self.score < other.score {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl From<alerts::Alert> for Alert {
    fn from(alert: alerts::Alert) -> Self {
        Self {
            id: alert.id,
            cvss_score: alert.cvss_score.unwrap_or(0.),
            provider: alert.provider,
            product: alert.product,
            description: alert.description,
            score: None,
        }
    }
}
