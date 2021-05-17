use serde::Deserialize;

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
    pub fn votes(&self) -> u8 {
        let mut sum = 0;

        if self.like {
            sum += 1;
        }

        if self.dislike {
            sum -= 1;
        }

        if self.critical {
            sum += 1;
        }

        sum as u8
    }

    pub fn score(&self) -> i32 {
        let mut sum = 0i32;

        if self.like {
            sum += LIKE;
        }

        if self.dislike {
            sum -= DISLIKE;
        }

        if self.critical {
            sum += CRITICAL;
        }

        sum
    }

    pub fn split_score<'l, 'r>(score: &'l mut (i32, i32, i32), rating: Self) -> &'l mut (i32, i32, i32) {
        if rating.critical {
            score.0 += CRITICAL;
        }

        if rating.like {
            score.1 += LIKE;
        }

        if rating.dislike {
            score.2 += DISLIKE;
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
