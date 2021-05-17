use std::collections::HashMap;

use users::{get_users_and_contents, get_users_and_ratings};

use crate::{models::{self, user::User}, services::database::services::users};

#[derive(Debug)]
pub struct Users {
    client: users::Client<tonic::transport::Channel>,
    endpoint: String,
}

impl Users {
    pub async fn connect<U: ToString>(url: U) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = url.to_string();
        Ok(Users {
            client: users::Client::connect(url.to_string()).await?,
            endpoint,
        })
    }
}

impl Users {
    pub async fn get_all_users(&mut self) -> HashMap<u32, User> {
        let user_ratings = self
            .client
            .get_users_and_ratings(get_users_and_ratings::Request::default())
            .await
            .unwrap();

        let users_contents = self
            .client
            .get_users_and_contents(get_users_and_contents::Request::default())
            .await
            .unwrap()
            .into_inner()
            .users;

        let ratings = user_ratings.into_inner().users;
        let mut hash = HashMap::<u32, User>::with_capacity(ratings.len());


        for content in users_contents {
            let mut user: models::user::User = content.into();

            for (id, inserted) in &mut hash {
                let intersection: Vec<&String> = user
                    .preferences
                    .iter()
                    .filter(|p| {
                        inserted
                            .preferences
                            .iter()
                            .find(|pref| pref == p)
                            .is_some()
                    })
                    .collect();

                let similarity = intersection.len() as f32
                    / (user.preferences.len() as f32 + inserted.preferences.len() as f32
                        - intersection.len() as f32);

                if let None = user.similarity {
                    user.similarity = Some(vec![]);
                }

                user.similarity
                    .as_mut()
                    .unwrap()
                    .push((id.to_owned(), similarity));

                if let None = inserted.similarity {
                    inserted.similarity = Some(vec![]);
                }

                inserted
                    .similarity
                    .as_mut()
                    .unwrap()
                    .push((user.id.to_owned(), similarity));
            }

            hash.insert(user.id, user.clone());
        }

        for rating in ratings {
            let user: models::user::User = rating.into();

            if let Some(usr) = hash.get_mut(&user.id) {
                usr.ratings = user.ratings;
            }
        }

        hash
    }
}
