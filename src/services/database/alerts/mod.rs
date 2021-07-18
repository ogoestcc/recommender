use crate::services::database::services::alerts::get_alerts_and_ratings::AlertsRatings;
use futures::future::join_all;

use crate::resources::errors::Internal;
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

impl super::Database {
    pub async fn get_alerts_scores(
        &mut self,
    ) -> Result<HashMap<(String, String, String), f32>, Internal> {
        let alerts = self
            .alerts
            .get_alerts(alerts::get_alerts::Request::default())
            .await
            .map_err(Internal::from)?;

        let alerts = alerts.into_inner().alerts;
        let mut hash = alerts
            .iter()
            .map(|a| {
                (
                    a.id.to_owned(),
                    (0., a.provider.to_owned(), a.product.to_owned()),
                )
            })
            .collect::<HashMap<String, (f32, String, String)>>();

        let ratings = self
            .alerts
            .get_alerts_and_ratings(get_alerts_and_ratings::Request::default())
            .await
            .map_err(Internal::from)?;
        let ratings = ratings.into_inner().alerts;
        let ratings_size = ratings.len();

        let ratings = join_all(
            ratings
                .iter()
                .map(|AlertsRatings { alert, ratings }| async move {
                    ratings
                        .iter()
                        .map(|rating| (alert.id.to_owned(), models::rating::Rating::from(rating)))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
        .await;

        let (rating_avg, score) = ratings.iter().flatten().fold(
            (0f32, (0i32, 0i32, 0i32)),
            |(sum, mut score), (_, rating)| {
                (
                    (sum + rating.votes() as f32) / (ratings_size as f32),
                    *rating.score_mut(&mut score),
                )
            },
        );
        let ratings_score = score.0 + score.1 - score.2;

        for ratings in ratings {
            if ratings.len() > 0 {
                let (alert_id, _) = &ratings[0];

                let (alert_votes, score) = ratings.iter().fold(
                    (0f32, (0i32, 0i32, 0i32)),
                    |(sum, mut score), (_, rating)| {
                        ((sum + rating.votes() as f32), *rating.score_mut(&mut score))
                    },
                );

                let alert_score = score.0 + score.1 - score.2;

                let score = ((rating_avg * ratings_score as f32)
                    + (alert_votes * alert_score as f32))
                    / rating_avg
                    + alert_score as f32;

                if let Some(s) = hash.get_mut(alert_id) {
                    s.0 = score;
                }
            }
        }

        Ok(hash
            .iter()
            .map(|(id, (score, provider, product))| {
                (
                    (id.to_owned(), provider.to_owned(), product.to_owned()),
                    *score,
                )
            })
            .collect())
    }

    pub async fn get_all_alerts(&mut self) -> HashMap<String, models::alert::Alert> {
        let alerts = self
            .alerts
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
