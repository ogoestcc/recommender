use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    models::rating::Rating,
    services::types::users::{contents, ratings},
};

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "user_id")]
    pub id: u32,
    #[serde(with = "preferences")]
    pub preferences: Vec<String>,
    pub ratings: Option<HashMap<String, i32>>,
    pub similarity: Option<Vec<(u32, f32)>>,
    // pub prefs: Vec<Contents>,
}

impl User {
    pub fn alert_rating(&self, alert_id: &String) -> i32 {
        if let Some(ratings) = &self.ratings {
            if let Some(rating) = ratings.get(alert_id) {
                *rating
            } else {
                0
            }
        } else {
            0
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
        let mut ratings = HashMap::with_capacity(base.ratings.len());

        for rating in base.ratings {
            let rating = Rating::from(rating);
            ratings.insert(rating.alert_id.clone(), rating.score());
        }

        Self {
            id: base.user.id as u32,
            preferences: vec![],
            ratings: Some(ratings),
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
