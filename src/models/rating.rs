use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::services::types::ratings;

const CRITICAL: i32 = 4;
const LIKE: i32 = 1;
const DISLIKE: i32 = 2;

#[derive(Debug, Deserialize, Clone)]
pub struct Rating {
    #[serde(rename = "userid")]
    pub user_id: u32,
    #[serde(rename = "cveid")]
    pub alert_id: String,
    #[serde(with = "int_to_bool")]
    pub like: bool,
    #[serde(with = "int_to_bool")]
    pub dislike: bool,
    #[serde(with = "int_to_bool")]
    pub critical: bool,
}

mod int_to_bool {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(u16::deserialize(deserializer)? == 1u16)
    }
}

impl Rating {
    pub fn votes(&self) -> i8 {
        self.like.then(|| 1).unwrap_or(0) + self.critical.then(|| 1).unwrap_or(0)
            - self.dislike.then(|| 1).unwrap_or(0)
    }

    pub fn score(&self) -> i32 {
        let mut sum = self.like.then(|| LIKE).unwrap_or(0);
        sum += self.dislike.then(|| DISLIKE).unwrap_or(0);
        sum + self.critical.then(|| CRITICAL).unwrap_or(0)
    }

    pub fn score_mut<'l, 'r>(&self, score: &'l mut (i32, i32, i32)) -> &'l mut (i32, i32, i32) {
        if self.critical {
            score.0 += CRITICAL;
        }

        if self.like {
            score.1 += LIKE;
        }

        if self.dislike {
            score.2 += DISLIKE;
        }

        score
    }

    pub fn split_score<'l, 'r>(
        score: &'l mut (i32, i32, i32),
        rating: Self,
    ) -> &'l mut (i32, i32, i32) {
        if rating.critical {
            score.0 += CRITICAL;
        }

        if rating.like {
            score.1 += LIKE;
        }

        if rating.dislike {
            score.2 -= DISLIKE;
        }

        score
    }
}

impl From<ratings::Rating> for Rating {
    fn from(base: ratings::Rating) -> Self {
        Self {
            user_id: base.user_id as u32,
            alert_id: base.alert_id,
            like: base.like,
            dislike: base.dislike,
            critical: base.critical,
        }
    }
}

impl<T: Into<Rating> + Clone> From<&T> for Rating {
    fn from(base: &T) -> Self {
        base.clone().into()
    }
}

#[derive(Debug)]
pub enum RatingEval {
    Liked { critical: bool },
    Disliked { critical: bool },
}

impl RatingEval {
    pub fn score(&self) -> i32 {
        match self {
            RatingEval::Liked { critical } => critical.then(|| CRITICAL).unwrap_or(0) + LIKE,
            RatingEval::Disliked { critical } => critical.then(|| CRITICAL).unwrap_or(0) - DISLIKE,
        }
    }
}

#[derive(Debug)]
pub struct Ratings {
    ratings: HashMap<String, RatingEval>,
    liked: usize,
    disliked: usize,
}

impl From<Vec<(String, (bool, bool, bool))>> for Ratings {
    fn from(values: Vec<(String, (bool, bool, bool))>) -> Self {
        let mut ratings = HashMap::with_capacity(values.len());

        let mut liked = 0usize;
        let mut disliked = 0usize;

        for (alert, (l, _, critical)) in values {
            let eval = if l {
                RatingEval::Liked { critical }
            } else {
                RatingEval::Disliked { critical }
            };

            match eval {
                RatingEval::Liked { .. } => liked += 1,
                RatingEval::Disliked { .. } => disliked += 1,
            }

            ratings.insert(alert, eval);
        }

        Self {
            ratings,
            liked,
            disliked,
        }
    }
}

impl Ratings {
    fn normalize(max: f32, value: f32) -> f32 {
        (value - 1.) / (max - 1.)
    }

    fn like_relevance(&self, ratings_len: f32) -> f32 {
        let relevance = ratings_len / self.liked as f32;

        Self::normalize(ratings_len, relevance) / LIKE as f32
    }

    fn dislike_relevance(&self, ratings_len: f32) -> f32 {
        let relevance = ratings_len / self.disliked as f32;

        Self::normalize(ratings_len, relevance) / DISLIKE as f32
    }

    pub fn relevance(&self, alert_id: &String) -> f32 {
        match self.ratings.get(alert_id) {
            Some(RatingEval::Liked { .. }) => self.like_relevance(self.ratings.len() as f32),
            Some(RatingEval::Disliked { .. }) => self.dislike_relevance(self.ratings.len() as f32),
            _ => 2.0,
        }
    }

    pub fn alert_rating(&self, alert_id: &String) -> i32 {
        self.ratings.get(alert_id).map_or(0, RatingEval::score)
    }

    pub fn get(&self, alert_id: &String) -> Option<i32> {
        self.ratings.get(alert_id).map(RatingEval::score)
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct UserRatings {
    ratings: Mutex<HashMap<String, Rating>>,
    liked: usize,
    disliked: usize,
}

impl Clone for UserRatings {
    fn clone(&self) -> Self {
        let ratings = self.ratings.lock().unwrap();

        Self {
            ratings: Mutex::new(ratings.clone()),
            liked: 0,
            disliked: 0,
        }
    }
}

impl UserRatings {
    fn normalize(max: f32, value: f32) -> f32 {
        (value - 1.) / (max - 1.)
    }

    fn like_relevance(&self, ratings_len: f32) -> f32 {
        let relevance = ratings_len / self.liked as f32;

        UserRatings::normalize(ratings_len, relevance) / LIKE as f32
    }

    fn dislike_relevance(&self, ratings_len: f32) -> f32 {
        let relevance = ratings_len / self.disliked as f32;

        UserRatings::normalize(ratings_len, relevance) / DISLIKE as f32
    }

    pub fn push(&mut self, rating: Rating) -> usize {
        let mut ratings = self.ratings.lock().unwrap();

        if rating.like {
            self.liked += 1usize;
        } else if rating.dislike {
            self.disliked += 1usize;
        }

        ratings.insert(rating.alert_id.to_owned(), rating);

        let new_size = ratings.len();

        new_size
    }

    pub fn relevance(&self, alert_id: &String) -> f32 {
        let ratings = self.ratings.lock().unwrap();

        match ratings.get(alert_id) {
            Some(Rating { like, .. }) if *like => self.like_relevance(ratings.len() as f32),
            Some(Rating { dislike, .. }) if *dislike => {
                self.dislike_relevance(ratings.len() as f32)
            }
            _ => 2.0,
        }
    }

    pub fn alert_rating(&self, alert_id: &String) -> i32 {
        let ratings = self.ratings.lock().unwrap();

        ratings.get(alert_id).map_or(0, |rating| rating.score())
    }

    pub fn get(&self, alert_id: &String) -> Option<i32> {
        let ratings = self.ratings.lock().unwrap();

        ratings.get(alert_id).map(|rating| rating.score())
    }
}
