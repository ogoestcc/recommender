use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    models::rating::UserRatings,
    services::types::users::{contents, ratings},
};

use super::alert::Alert;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "user_id")]
    pub id: u32,
    #[serde(with = "preferences")]
    pub preferences: Vec<String>,
    pub ratings: Option<UserRatings>,
    pub similarity: Option<Vec<(u32, f32)>>,
    // pub prefs: Vec<Contents>,
}

impl User {
    pub fn alert_rating(&self, alert_id: &String) -> i32 {
        self.ratings
            .as_ref()
            .map_or(0, |r| r.alert_rating(alert_id))
    }

    pub fn alert_score(&self, alert: &Alert) -> f32 {
        match &self.ratings {
            Some(rating) => rating.relevance(&alert.id) * alert.score.unwrap_or(0.),
            None => alert.score.unwrap_or(0.),
        }
    }

    pub fn alert_score_by_id(&self, alert: &String, score: f32) -> f32 {
        match &self.ratings {
            Some(rating) => rating.relevance(alert) * score,
            None => score,
        }
    }
}

mod preferences {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?
            .replace(&['[', ']', '\'', ' '][..], "")
            .split(',')
            .map(|s| s.into())
            .collect())
    }
}

impl From<ratings::UsersRatings> for User {
    fn from(base: ratings::UsersRatings) -> Self {
        let mut ratings = (!base.ratings.is_empty()).then(|| UserRatings::default());

        for rating in base.ratings {
            match &mut ratings {
                Some(r) => r.push(rating.into()),
                None => 0,
            };
        }

        Self {
            id: base.user.id as u32,
            preferences: vec![],
            ratings,
            similarity: None,
        }
    }
}

impl From<contents::UsersContents> for User {
    fn from(base: contents::UsersContents) -> Self {
        let user = base.user;
        Self {
            id: user.id as u32,
            preferences: base.preferences.iter().map(|c| c.id.clone()).collect(),
            ratings: None,
            similarity: None,
        }
    }
}
