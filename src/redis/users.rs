use crate::models::recommender::Recommender;
use crate::redis::Redis;
use crate::resources::errors::Internal;
use futures::future::join_all;
use redis::AsyncCommands;
use std::collections::HashMap;

impl super::Redis {
    fn user_preferences_key<U: std::fmt::Display>(id: U) -> String {
        format!(r#"users.{}.preferences"#, id)
    }

    fn user_ratings_key<U: std::fmt::Display>(id: U) -> String {
        format!(r#"users.{}.ratings"#, id)
    }

    fn user_similarity_key<U: std::fmt::Display>(id: U) -> String {
        format!(r#"users.{}.similarity"#, id)
    }

    pub async fn add_user_preferences(
        &self,
        user_id: u32,
        preferences: Vec<String>,
    ) -> Result<(), Internal> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        let hash_key = Redis::user_preferences_key(user_id);

        connection
            .del(hash_key.clone())
            .await
            .map_err(Internal::from)?;

        connection
            .lpush(hash_key, preferences)
            .await
            .map_err(Internal::from)?;

        Ok(())
    }

    pub async fn get_user_preferences(&self, user_id: u32) -> Result<Vec<String>, Internal> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        let hash_key = Redis::user_preferences_key(user_id);

        Ok(connection
            .lrange(hash_key, 0, -1)
            .await
            .map_err(Internal::from)?)
    }

    pub async fn add_user_ratings(
        &self,
        user_id: u32,
        ratings: Vec<(String, (bool, bool, bool))>,
    ) -> Result<(), Internal> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        let hash_key = Redis::user_ratings_key(user_id);

        connection
            .hset_multiple(
                hash_key,
                &ratings
                    .iter()
                    .map(|(alert_id, (like, dislike, critical))| {
                        (alert_id, format!("{}:{}:{}", like, dislike, critical))
                    })
                    .collect::<Vec<_>>(),
            )
            .await
            .map_err(Internal::from)?;

        Ok(())
    }

    pub async fn get_user_ratings(
        &self,
        user_id: u32,
    ) -> Result<HashMap<String, (bool, bool, bool)>, Internal> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        let hash_key = Redis::user_ratings_key(user_id);

        let keys_and_values: Vec<String> =
            connection.hgetall(hash_key).await.map_err(Internal::from)?;

        let ret = Redis::zip_key_value(keys_and_values)
            .iter()
            .map(|(alert_id, rating)| {
                let rating: Vec<_> = rating.split(':').collect();

                (
                    alert_id.to_owned(),
                    (
                        rating[0].parse().unwrap(),
                        rating[1].parse().unwrap(),
                        rating[2].parse().unwrap(),
                    ),
                )
            })
            .collect();

        Ok(ret)
    }

    pub async fn calculate_user_similarities(&self, user_id: u32) -> Result<(), Internal> {
        let preferences = self.get_user_preferences(user_id).await?;

        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        let keys: Vec<String> = connection
            .keys(Redis::user_preferences_key('*'))
            .await
            .map_err(Internal::from)?;

        let users_ids = keys
            .iter()
            .filter_map(|key| {
                let splitted = key.split('.').collect::<Vec<_>>();
                let id = splitted[1].parse::<u32>().unwrap();

                (id != user_id).then(|| id)
            })
            .collect::<Vec<_>>();

        if users_ids.len() == 0 {
            return Ok(());
        }

        let users_preferences = join_all(
            users_ids
                .iter()
                .map(|&id| async move { (id, self.get_user_preferences(id).await) }),
        )
        .await;

        let similarities = users_preferences
            .iter()
            .filter_map(|(id, pref)| {
                if let Ok(pref) = pref {
                    Some((
                        Recommender::users_similarity(&preferences, pref.to_vec()),
                        id,
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let hash_key = Redis::user_similarity_key(user_id);

        connection
            .zadd_multiple(hash_key, &similarities)
            .await
            .map_err(Internal::from)?;

        Ok(())
    }

    pub async fn get_user_similarities(
        &self,
        user_id: u32,
        number: Option<isize>,
    ) -> Result<HashMap<u32, f32>, Internal> {
        let mut connection = self
            .client
            .get_async_connection()
            .await
            .map_err(Internal::from)?;

        let hash_key = Redis::user_similarity_key(user_id);

        let keys_and_values: Vec<String> = connection
            .zrevrangebyscore_limit_withscores(hash_key, "+inf", 0, 0, number.unwrap_or(20))
            .await
            .map_err(Internal::from)?;

        Ok(Redis::zip_key_value(keys_and_values)
            .iter()
            .map(|(id, similarity)| {
                (
                    id.parse::<u32>().unwrap(),
                    similarity.parse::<f32>().unwrap(),
                )
            })
            .collect())
    }
}
