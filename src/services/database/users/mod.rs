use crate::resources::errors::Internal;
use crate::services::types::users::WhereClause;

use std::collections::HashMap;

use users::{get_users_and_contents, get_users_and_ratings};

use crate::{
    models::{self, user::User},
    services::database::services::users,
};

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

impl super::Database {
    pub async fn get_user_preferences_and_ratings(
        &mut self,
        user_id: u32,
    ) -> Result<(Vec<String>, Vec<(String, (bool, bool, bool))>), Internal> {
        let mut user_w = WhereClause::default();
        user_w.id = Some(user_id as i32);

        let mut content_req = get_users_and_contents::Request::default();
        let mut rating_req = get_users_and_ratings::Request::default();

        content_req.r#where = Some(user_w.clone());
        rating_req.user_where = Some(user_w.clone());

        let (contents, ratings) = (
            self.users.get_users_and_contents(content_req).await,
            self.users.get_users_and_ratings(rating_req).await,
        );

        let content = contents?.into_inner().users;
        let ratings = ratings?.into_inner().users;

        let content = futures::future::join_all(content.iter().map(|content| async move {
            content
                .preferences
                .iter()
                .map(|content| content.id.to_owned())
                .collect::<Vec<_>>()
        }))
        .await;

        let ratings = futures::future::join_all(ratings.iter().map(|rating| async move {
            rating
                .ratings
                .iter()
                .map(|rating| {
                    (
                        rating.alert_id.to_owned(),
                        (rating.like, rating.dislike, rating.critical),
                    )
                })
                .collect::<Vec<_>>()
        }))
        .await;

        Ok((
            content.iter().flatten().map(|c| c.to_owned()).collect(),
            ratings.iter().flatten().map(|r| r.to_owned()).collect(),
        ))
    }

    pub async fn get_users_contents(&mut self) -> Result<Vec<(u32, Vec<String>)>, Internal> {
        let mut request = get_users_and_contents::Request::default();

        let users_contents = self
            .users
            .get_users_and_contents(request)
            .await
            .unwrap()
            .into_inner()
            .users;

        Ok(users_contents
            .iter()
            .map(|response| {
                let user = &response.user;
                let contents = response
                    .preferences
                    .iter()
                    .map(|content| content.id.to_owned())
                    .collect::<Vec<String>>();

                (user.id as u32, contents)
            })
            .collect())
    }

    pub async fn get_all_users(&mut self) -> HashMap<u32, User> {
        let user_ratings = self
            .users
            .get_users_and_ratings(get_users_and_ratings::Request::default())
            .await
            .unwrap();

        let users_contents = self
            .users
            .get_users_and_contents(get_users_and_contents::Request::default())
            .await
            .unwrap()
            .into_inner()
            .users;

        let ratings = user_ratings.into_inner().users;
        let mut hash = HashMap::<u32, User>::with_capacity(users_contents.len());

        for content in users_contents {
            let mut user: models::user::User = content.into();

            for (id, inserted) in &mut hash {
                let intersection: Vec<&String> = user
                    .preferences
                    .iter()
                    .filter(|p| inserted.preferences.iter().find(|pref| pref == p).is_some())
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
