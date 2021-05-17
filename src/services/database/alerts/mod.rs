use std::collections::HashMap;

use crate::{
    models,
    services::database::services::alerts::{self, get_alerts_and_ratings},
};

#[derive(Debug)]
pub struct Alerts {
    client: alerts::Client<tonic::transport::Channel>,
    endpoint: String,
}

impl Alerts {
    pub async fn connect<U: ToString>(url: U) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = url.to_string();
        Ok(Alerts {
            client: alerts::Client::connect(url.to_string()).await?,
            endpoint,
        })
    }
}

impl Alerts {
    pub async fn get_all_alerts(&mut self) -> HashMap<String, models::alert::Alert> {
        let alerts = self
            .client
            .get_alerts_and_ratings(get_alerts_and_ratings::Request::default())
            .await
            .unwrap();
        let alerts = alerts.into_inner().alerts;
        let mut hash = HashMap::<String, models::alert::Alert>::with_capacity(alerts.len());

        let mut a = vec![1];

        a.extend(vec![2, 3]);

        let mut ratings = vec![];

        for rats in alerts.iter().map(|a| a.ratings.clone()) {
            ratings.extend(rats);
        }

        // let ratings = ratings.iter().map(models::rating::Rating::from).collect::<Vec<_>>();

        let mut score = (0i32, 0i32, 0i32);
        let rating_avg = ratings
            .iter()
            .map(models::rating::Rating::from)
            .fold(0f32, |sum, rating| sum + rating.votes() as f32)
            / (ratings.len() as f32);
        let score = ratings
            .iter()
            .map(models::rating::Rating::from)
            .fold(&mut score, models::rating::Rating::split_score);

        let ratings_score = score.0 + score.1 - score.2;

        for alert in alerts {
            let alert_ratings = alert.ratings;
            let mut alert: models::alert::Alert = alert.alert.into();

            let mut score = (0i32, 0i32, 0i32);
            // let score = models::rating::Rating::split_score(&mut score, rating);
            let score = alert_ratings
                .iter()
                .map(models::rating::Rating::from)
                .fold(&mut score, models::rating::Rating::split_score);

            let alert_score = score.0 + score.1 - score.2;

            let alerts_votes = alert_ratings
                .iter()
                .map(models::rating::Rating::from)
                .fold(0f32, |sum, rating| sum + rating.votes() as f32);

            alert.score = Some(
                ((rating_avg * ratings_score as f32) + (alerts_votes * alert_score as f32))
                    / rating_avg
                    + alert_score as f32,
            );

            hash.insert(alert.id.clone(), alert);
        }

        hash
    }
}
