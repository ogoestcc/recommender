use std::collections::HashMap;

use super::{alert::Alert, rating::Rating, user::User};

#[derive(Debug)]
pub struct Recommender {
    pub users: HashMap<u32, User>,
    pub alerts: HashMap<String, Alert>,
    // alerts: &'b mut HashMap<&'b str, &'b Alert>,
    // ratings: &'b Vec<Rating>,
}

pub struct RecommenderBuilder {}

impl RecommenderBuilder {
    fn rating_score<'l>(acc: &'l mut (i32, i32, i32), rating: &Rating) -> &'l mut (i32, i32, i32) {
        if rating.critical {
            acc.0 += 1;
        }

        if rating.like {
            acc.1 += 1;
        }

        if rating.dislike {
            acc.2 += 1;
        }

        acc
    }

    async fn build_users(users: Vec<User>, ratings: &Vec<Rating>) -> HashMap<u32, User> {
        let mut hash: HashMap<u32, User> = HashMap::new();

        for user in users.iter() {
            let user = &mut user.to_owned();
            for (id, inserted_user) in &mut hash {
                let intersection: Vec<&String> = user
                    .preferences
                    .iter()
                    .filter(|p| {
                        inserted_user
                            .preferences
                            .iter()
                            .find(|pref| pref == p)
                            .is_some()
                    })
                    .collect();

                let similarity = intersection.len() as f32
                    / (user.preferences.len() as f32 + inserted_user.preferences.len() as f32
                        - intersection.len() as f32);

                if let None = user.similarity {
                    user.similarity = Some(vec![]);
                }

                user.similarity
                    .as_mut()
                    .unwrap()
                    .push((id.to_owned(), similarity));

                if let None = inserted_user.similarity {
                    inserted_user.similarity = Some(vec![]);
                }

                inserted_user
                    .similarity
                    .as_mut()
                    .unwrap()
                    .push((user.id.to_owned(), similarity));
            }

            let mut ratings_scores_by_alert_id = HashMap::new();

            let alpha = 1;
            let beta = 1;
            let phi = 1;

            for rating in ratings {
                if rating.user_id == user.id {
                    let score = RecommenderBuilder::rating_score(&mut (0, 0, 0), rating).to_owned();

                    ratings_scores_by_alert_id.insert(
                        rating.alert_id.clone(),
                        (alpha * score.0) + (beta * score.1) + (phi * score.2),
                    );
                }
            }

            user.ratings = Some(ratings_scores_by_alert_id);

            hash.insert(user.id, user.to_owned());
        }

        hash
    }

    async fn build_alerts(alerts: Vec<Alert>, ratings: &Vec<Rating>) -> HashMap<String, Alert> {
        let mut hash: HashMap<String, Alert> = HashMap::new();

        let rating_votes = |rating: &Rating| {
            let mut votes = 0;

            if rating.like {
                votes += 1;
            }

            if rating.dislike {
                votes += 1;
            }

            if rating.critical {
                votes += 1;
            }

            votes
        };

        let rating_votes_sum = |ratings: &Vec<Rating>| {
            ratings
                .iter()
                .fold(0f32, |sum, rating| sum + rating_votes(rating) as f32)
        };

        let ranking_score = |ratings: &Vec<Rating>| {
            let mut alert_score = (0, 0, 0);

            let score = ratings
                .iter()
                .fold(&mut alert_score, RecommenderBuilder::rating_score);

            let alpha = 1;
            let beta = 1;
            let phi = 1;

            (alpha * score.0) + (beta * score.1) + (phi * score.2)
        };

        let rating_avg = rating_votes_sum(&ratings) / ratings.len() as f32;
        let ratings_score = ranking_score(&ratings);

        for alert in alerts.iter() {
            let alert = &mut alert.to_owned();
            let alert_ratings = ratings
                .iter()
                .filter(|rating| rating.alert_id == alert.id)
                .map(|r| r.to_owned())
                .collect::<Vec<_>>();

            let alert_score: i32 = ranking_score(&alert_ratings);
            let alerts_votes = rating_votes_sum(&alert_ratings);

            alert.score = Some(
                ((rating_avg * ratings_score as f32) + (alerts_votes * alert_score as f32))
                    / rating_avg
                    + alert_score.abs() as f32,
            );

            hash.insert(alert.id.clone(), alert.to_owned());
        }

        hash
    }

    pub async fn build(users: Vec<User>, alerts: Vec<Alert>, ratings: Vec<Rating>) -> Recommender {
        let (users, alerts) = futures::join!(
            RecommenderBuilder::build_users(users, &ratings),
            RecommenderBuilder::build_alerts(alerts, &ratings)
        );

        Recommender {
            users,
            alerts,
            // ratings: &ratings,
        }
    }
}
