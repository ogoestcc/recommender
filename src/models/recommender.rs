use array_tool::vec::Intersect;
use std::{cmp::Ordering, collections::HashMap};

use super::{alert::Alert, user::User};

#[derive(Debug, Clone)]
pub struct Recommender {
    pub users: HashMap<u32, User>,
    pub alerts: HashMap<String, Alert>,
    // alerts: &'b mut HashMap<&'b str, &'b Alert>,
    // ratings: &'b Vec<Rating>,
}

/// PERSONALIZED RECOMMENDATION
impl Recommender {
    pub fn users_similarity(left: &Vec<String>, right: Vec<String>) -> f32 {
        let right_size = right.len();
        let intersection: Vec<_> = left.intersect(right);

        intersection.len() as f32
            / (left.len() as f32 + right_size as f32 - intersection.len() as f32)
    }

    fn not_viewed(user: &User, alert: &Alert) -> bool {
        if let Some(ratings) = &user.ratings {
            ratings.get(&alert.id).is_none()
        } else {
            true
        }
    }

    fn include_preferences(user: &User, alert: &Alert) -> bool {
        let preferences = &user.preferences;

        preferences
            .iter()
            .any(|pref| (*pref == alert.product || *pref == alert.provider))
    }

    fn sort_alerts<'u, T: 'u>(
        user: &'u User,
        get_id: for<'a> fn(&'a T) -> &'a String,
        get_score: for<'a> fn(&'a T) -> f32,
    ) -> impl FnMut(&T, &T) -> Ordering + 'u {
        move |left, right| {
            let left = user.alert_score_by_id(get_id(left), get_score(left));
            let right = user.alert_score_by_id(get_id(right), get_score(right));

            let order = if left > right {
                Ordering::Greater
            } else if left < right {
                Ordering::Less
            } else {
                Ordering::Equal
            };

            order.reverse()
        }
    }

    pub fn content_based(
        &self,
        user_id: u32,
        alert_number: u16,
        exclude_clause: Option<for<'r, 's> fn(&'r User, &'s Alert) -> bool>,
    ) -> Vec<&Alert> {
        let viewed_method = if let Some(method) = exclude_clause {
            method
        } else {
            Recommender::not_viewed
        };

        let user = self.users.get(&user_id);

        if user.is_none() {
            return vec![];
        }

        let user = user.unwrap();

        let mut alerts = self.alerts.values().collect::<Vec<_>>();

        // let mut alerts = alerts
        //     .iter()
        //     .filter(|alert| {
        //         let alert = alert.to_owned().to_owned();

        //         viewed_method(user, alert)
        //     })
        //     .collect::<Vec<_>>();

        alerts.sort_by(Recommender::sort_alerts::<&Alert>(
            user,
            |a| &a.id,
            |a| a.score.unwrap_or(0.),
        ));

        let limit = if alert_number as usize > alerts.len() {
            alerts.len()
        } else {
            alert_number as usize
        };

        let slice = &alerts[..limit];
        slice.iter().map(|alert| alert.to_owned()).collect()
    }

    pub fn collaborative_filtering(&self, user_id: u32, alert_number: u16) -> Vec<&Alert> {
        let user = self.users.get(&user_id).unwrap();

        let alerts: Vec<_> = self
            .alerts
            .iter()
            .filter(|(_, alert)| Recommender::not_viewed(user, alert))
            .collect();

        let alerts = alerts.clone();

        let mut alerts: Vec<_> = alerts
            .iter()
            .filter_map(|(alert_id, alert)| {
                let mut score = 0.;
                let mut similarity_total_mod = 0.;

                if let Some(similarity) = &user.similarity {
                    for (similar_id, similarity) in similarity {
                        let similar_user = self.users.get(similar_id).unwrap();

                        score += similar_user.alert_rating(alert_id) as f32 * similarity;

                        similarity_total_mod += similarity.abs();
                    }

                    Some((alert, score / similarity_total_mod))
                } else {
                    None
                }
            })
            .collect();

        alerts.sort_by(Recommender::sort_alerts::<(&&Alert, f32)>(
            user,
            |(a, _)| &a.id,
            |(_, s)| *s,
        ));

        let limit = if alert_number as usize > alerts.len() {
            alerts.len()
        } else {
            alert_number as usize
        };

        let slice = &alerts[..limit];
        slice
            .iter()
            .map(|(alert, _)| alert.to_owned().to_owned())
            .collect()
    }
}

/// NON-PERSONALIZED RECOMMENDATION
impl Recommender {
    pub fn top_n(&self, alert_number: u32, content: Option<String>) -> Vec<&Alert> {
        let alerts = self.alerts.values().collect::<Vec<_>>();

        let mut alerts = if let Some(content) = &content {
            alerts
                .iter()
                .filter_map(|a| {
                    if a.filter_content(content) {
                        Some(a.to_owned())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            alerts
        };

        alerts.sort_by(|a, b| a.cmp(b).reverse());

        let limit = if alert_number as usize > alerts.len() {
            alerts.len()
        } else {
            alert_number as usize
        };

        let slice = &alerts[..limit];
        slice.iter().map(|alert| alert.to_owned()).collect()
    }
}
