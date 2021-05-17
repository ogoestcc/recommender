use std::collections::HashMap;

use super::{alert::Alert, user::User};

#[derive(Debug, Clone)]
pub struct Recommender {
    pub users: HashMap<u32, User>,
    pub alerts: HashMap<String, Alert>,
    // alerts: &'b mut HashMap<&'b str, &'b Alert>,
    // ratings: &'b Vec<Rating>,
}

impl Recommender {
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

        let alerts = self.alerts.values().collect::<Vec<_>>();

        let mut alerts = alerts
            .iter()
            .filter(|alert| {
                let alert = alert.to_owned().to_owned();

                Recommender::include_preferences(user, alert) && viewed_method(user, alert)
            })
            .collect::<Vec<_>>();

        alerts.sort_by(|left, right| right.cmp(left).reverse());

        let slice = &alerts[..alert_number as usize];
        slice
            .iter()
            .map(|alert| alert.to_owned().to_owned())
            .collect()
    }
}

// pub struct RecommenderBuilder;

// impl RecommenderBuilder {
//     fn rating_score<'l>(acc: &'l mut (i32, i32, i32), rating: &Rating) -> &'l mut (i32, i32, i32) {
//         if rating.critical {
//             acc.0 += 1;
//         }

//         if rating.like {
//             acc.1 += 1;
//         }

//         if rating.dislike {
//             acc.2 += 1;
//         }

//         acc
//     }

//     async fn build_users(users: Vec<User>, ratings: &Vec<Rating>) -> HashMap<u32, User> {
//         let mut hash: HashMap<u32, User> = HashMap::with_capacity(users.len());

//         for user in users.iter() {
//             let user = &mut user.to_owned();
//             for (id, inserted_user) in &mut hash {
//                 let intersection: Vec<&String> = user
//                     .preferences
//                     .iter()
//                     .filter(|p| {
//                         inserted_user
//                             .preferences
//                             .iter()
//                             .find(|pref| pref == p)
//                             .is_some()
//                     })
//                     .collect();

//                 let similarity = intersection.len() as f32
//                     / (user.preferences.len() as f32 + inserted_user.preferences.len() as f32
//                         - intersection.len() as f32);

//                 if let None = user.similarity {
//                     user.similarity = Some(vec![]);
//                 }

//                 user.similarity
//                     .as_mut()
//                     .unwrap()
//                     .push((id.to_owned(), similarity));

//                 if let None = inserted_user.similarity {
//                     inserted_user.similarity = Some(vec![]);
//                 }

//                 inserted_user
//                     .similarity
//                     .as_mut()
//                     .unwrap()
//                     .push((user.id.to_owned(), similarity));
//             }

//             let mut ratings_scores_by_alert_id = HashMap::new();

//             let alpha = 4;
//             let beta = 1;
//             let phi = 2;

//             for rating in ratings {
//                 if rating.user_id == user.id {
//                     let score = RecommenderBuilder::rating_score(&mut (0, 0, 0), rating).to_owned();

//                     ratings_scores_by_alert_id.insert(
//                         rating.alert_id.clone(),
//                         ((alpha * score.0) + (beta * score.1) - (phi * score.2)) as i32,
//                     );
//                 }
//             }

//             user.ratings = Some(ratings_scores_by_alert_id);

//             hash.insert(user.id, user.to_owned());
//         }

//         hash
//     }

//     async fn build_alerts(alerts: &mut Vec<Alert>, ratings: &Vec<Rating>) -> HashMap<String, Alert> {
//         let mut hash: HashMap<String, Alert> = HashMap::with_capacity(alerts.len());

//         let rating_votes = |rating: &Rating| {
//             let mut votes = 0;

//             if rating.like {
//                 votes += 1;
//             }

//             if rating.dislike {
//                 votes += 1;
//             }

//             if rating.critical {
//                 votes += 1;
//             }

//             votes
//         };

//         let rating_votes_sum = |ratings: &Vec<Rating>| {
//             ratings
//                 .iter()
//                 .fold(0f32, |sum, rating| sum + rating_votes(rating) as f32)
//         };

//         let ranking_score = |ratings: &Vec<Rating>| {
//             let mut alert_score = (0, 0, 0);

//             let score = ratings
//                 .iter()
//                 .fold(&mut alert_score, RecommenderBuilder::rating_score);

//             let alpha = 4;
//             let beta = 1;
//             let phi = 2;

//             (alpha * score.0) + (beta * score.1) - (phi * score.2)
//         };

//         let rating_avg = rating_votes_sum(&ratings) / ratings.len() as f32;
//         let ratings_score = ranking_score(&ratings);

//         for alert in alerts.iter_mut() {
//             // let alert = &mut alert.to_owned();
//             let alert_ratings = ratings
//                 .iter()
//                 .filter(|rating| rating.alert_id == alert.id)
//                 .map(|r| r.to_owned())
//                 .collect::<Vec<_>>();

//             let alert_score: i32 = ranking_score(&alert_ratings);
//             let alerts_votes = rating_votes_sum(&alert_ratings);

//             alert.score = Some(
//                 ((rating_avg * ratings_score as f32) + (alerts_votes * alert_score as f32))
//                     / rating_avg
//                     + alert_score.abs() as f32,
//             );

//             hash.insert(alert.id.clone(), alert.to_owned());
//         }

//         hash
//     }

//     pub async fn build(users: Vec<User>, alerts: &mut Vec<Alert>, ratings: Vec<Rating>) -> Recommender {
//         let (users, alerts) = futures::join!(
//             RecommenderBuilder::build_users(users, &ratings),
//             RecommenderBuilder::build_alerts(alerts, &ratings)
//         );

//         Recommender {
//             users,
//             alerts,
//             // ratings: &ratings,
//         }
//     }
// }
