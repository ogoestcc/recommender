mod collaborative_filtering;
mod content_based;
mod top_n;

use crate::models::rating::Ratings;
use crate::resources::errors::Internal;
use crate::{redis::Redis, services::database::Database};

#[derive(Clone)]
pub struct Recommender {
    redis: Redis,
    database: Database,
}

impl std::fmt::Debug for Recommender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Recommender {:?}", self.database)
    }
}

impl Recommender {
    pub fn new(redis: Redis, database: Database) -> Self {
        Self { redis, database }
    }

    pub async fn load_data(&mut self) -> Result<(), Internal> {
        let users = self.database.get_users_contents().await?;
        let alerts = self.database.get_alerts_scores().await?;

        let (_, alerts) = futures::join!(
            futures::future::join_all(users.iter().map(|(id, preferences)| {
                self.redis.add_user_preferences(*id, preferences.to_vec())
            }),),
            self.redis.add_alerts(alerts)
        );

        alerts
    }

    pub async fn load_user_data(&mut self, user_id: u32) -> Result<(), Internal> {
        let (contents, ratings) = self
            .database
            .get_user_preferences_and_ratings(user_id)
            .await?;

        let (contents, ratings, similarity) = futures::join!(
            self.redis.add_user_preferences(user_id, contents),
            self.redis.add_user_ratings(user_id, ratings),
            self.redis.calculate_user_similarities(user_id)
        );

        contents?;
        ratings?;
        similarity?;

        Ok(())
    }

    fn slice<T: std::clone::Clone>(alerts: Vec<T>, number: usize) -> Vec<T> {
        let limit = if number > alerts.len() {
            alerts.len()
        } else {
            number
        };

        let slice = &alerts[..limit];
        slice.to_vec()
    }

    fn tag_weight(tags: &Vec<String>, tag: &String) -> f32 {
        tags.contains(tag)
            .then(|| 1. / tags.len() as f32)
            .unwrap_or(0.)
    }
}
