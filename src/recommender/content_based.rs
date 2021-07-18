use crate::recommender::Ratings;
use crate::resources::errors::Internal;
use std::cmp::Ordering;

impl super::Recommender {
    pub async fn content_based(
        &self,
        user_id: u32,
        alert_number: u16,
        // exclude_clause: Option<for<'r, 's> fn(&'r User, &'s Alert) -> bool>,
    ) -> Result<Vec<String>, Internal> {
        // let viewed_method = if let Some(method) = exclude_clause {
        //     method
        // } else {
        //     Recommender::not_viewed
        // };

        let (alerts, tags, ratings) = futures::join!(
            self.redis.get_alerts(Some(alert_number as isize)),
            self.redis.get_user_preferences(user_id),
            self.redis.get_user_ratings(user_id),
        );

        let (mut alerts, tags): (Vec<((String, String, String), f32)>, _) = (alerts?, tags?);

        let ratings = Ratings::from(
            ratings?
                .iter()
                .map(|(id, rating)| (id.to_owned(), *rating))
                .collect::<Vec<_>>(),
        );

        alerts.sort_by(
            |((l_id, l_provider, l_product), l_score), ((r_id, r_provider, r_product), r_score)| {
                let left = ratings.relevance(l_id)
                    * (Self::tag_weight(&tags, l_provider) + Self::tag_weight(&tags, l_product))
                    * l_score;
                let right = ratings.relevance(r_id)
                    * (Self::tag_weight(&tags, r_provider) + Self::tag_weight(&tags, r_product))
                    * r_score;

                let order = if left > right {
                    Ordering::Greater
                } else if left < right {
                    Ordering::Less
                } else {
                    Ordering::Equal
                };

                order.reverse()
            },
        );

        Ok(Self::slice(
            alerts.iter().map(|((id, _, _), _)| id.to_owned()).collect(),
            alert_number as usize,
        ))
    }
}
