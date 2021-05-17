use models::alert;

use crate::models;

pub use super::types::{Alert, AlertWhereClause as WhereClause};

impl<T: Into<Alert> + Clone> From<&T> for Alert {
    fn from(value: &T) -> Self {
        value.clone().into()
    }
}

impl From<alert::Alert> for Alert {
    fn from(alert: alert::Alert) -> Self {
        Self {
            id: alert.id,
            cvss_score: Some(alert.cvss_score),
            provider: alert.provider,
            product: alert.product,
            // published_at: alert.published_at,
            description: alert.description,
            ..Default::default()
        }
    }
}
