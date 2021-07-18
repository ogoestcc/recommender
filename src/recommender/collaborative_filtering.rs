use crate::models::rating::Ratings;
use futures::future;
use std::cmp::Ordering;
use std::collections::HashMap;

use crate::resources::errors::Internal;

impl super::Recommender {
    const USERS_NUMBER: isize = 10;

    pub async fn collaborative_filtering(
        &self,
        user_id: u32,
        alert_number: u16,
    ) -> Result<Vec<String>, Internal> {
        let (alerts, similarities, ratings) = futures::join!(
            self.redis.get_alerts(None),
            self.redis
                .get_user_similarities(user_id, Some(Self::USERS_NUMBER)),
            self.redis.get_user_ratings(user_id)
        );
        let (alerts, similarities, ratings) = (
            alerts?,
            similarities?,
            Ratings::from(
                ratings?
                    .iter()
                    .map(|(id, r)| (id.to_owned(), *r))
                    .collect::<Vec<_>>(),
            ),
        );

        let preferences = future::join_all(
            similarities
                .iter()
                .map(|(id, _)| self.redis.get_user_preferences(*id)),
        )
        .await;

        let preferences = similarities
            .iter()
            .zip(preferences.iter())
            .filter_map(|((id, simi), pref)| match pref {
                Ok(preferences) => Some((*id, (*simi, preferences.to_owned()))),
                Err(_) => None,
            })
            .collect::<HashMap<u32, (f32, Vec<String>)>>();

        let mut alerts = alerts
            .iter()
            .map(|((id, provider, product), _)| {
                let mut score = 0.;
                let mut similarity_total_mod = 0.;

                for (id, (similarity, pref)) in &preferences {
                    let content_score =
                        Self::tag_weight(pref, &provider) + Self::tag_weight(pref, &product);

                    score += content_score * similarity;
                    similarity_total_mod += similarity.abs();
                }

                (id, (score / similarity_total_mod))
            })
            .collect::<Vec<_>>();

        alerts.sort_by(|(l_id, l_score), (r_id, r_score)| {
            let left = ratings.relevance(l_id) * l_score;
            let right = ratings.relevance(r_id) * r_score;

            let order = if left > right {
                Ordering::Greater
            } else if left < right {
                Ordering::Less
            } else {
                Ordering::Equal
            };

            order.reverse()
        });

        Ok(Self::slice(alerts, alert_number as usize)
            .iter()
            .map(|(id, _)| id.to_owned().to_owned())
            .collect())
    }
}
