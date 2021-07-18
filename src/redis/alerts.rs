use crate::resources::errors::Internal;
use redis::AsyncCommands;
use std::collections::HashMap;

impl<'a> super::Redis {
    const ALERTS_KEYS: &'static str = r#"alerts"#;

    pub async fn add_alerts(
        &self,
        alerts: HashMap<(String, String, String), f32>,
    ) -> Result<(), Internal> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        connection
            .zadd_multiple(
                Self::ALERTS_KEYS,
                &alerts
                    .iter()
                    .map(|((id, provider, product), score)| {
                        (score, format!("{}:{}:{}", id, provider, product))
                    })
                    .collect::<Vec<_>>(),
            )
            .await
            .map_err(Internal::from)?;

        Ok(())
    }

    pub async fn get_alerts(
        &self,
        number: Option<isize>,
    ) -> Result<Vec<((String, String, String), f32)>, Internal> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        let query = if let Some(number) = number {
            connection.zrevrangebyscore_limit_withscores(Self::ALERTS_KEYS, "+inf", 0, 0, number)
        } else {
            connection.zrevrangebyscore_withscores(Self::ALERTS_KEYS, "+inf", 0)
        };

        let keys_and_values: Vec<String> = query.await.map_err(Internal::from)?;

        Ok(Self::zip_key_value(keys_and_values)
            .iter()
            .map(|(id, score)| {
                let splitted: Vec<_> = id.split(':').collect();

                (
                    (
                        splitted[0].to_string(),
                        splitted[1].to_string(),
                        splitted[2].to_string(),
                    ),
                    score.parse::<f32>().unwrap(),
                )
            })
            .collect())
    }
}
