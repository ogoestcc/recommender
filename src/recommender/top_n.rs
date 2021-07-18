use crate::resources::errors::Internal;

impl super::Recommender {
    pub async fn top_n(&self, alert_number: u32) -> Result<Vec<String>, Internal> {
        let alerts = self.redis.get_alerts(Some(alert_number as isize)).await?;

        Ok(alerts.iter().map(|((id, _, _), _)| id.to_owned()).collect())
    }
}
