use csv::Reader;
use serde::Deserialize;

use crate::models::{alert, rating, user};

use super::{async_trait, Database};

pub struct CSVDatabase<'f> {
    users_filepath: &'f str,
    ratings_filepath: &'f str,
    alerts_filepath: &'f str,
}

impl<'f> CSVDatabase<'f> {
    pub fn new(
        users_filepath: &'f str,
        ratings_filepath: &'f str,
        alerts_filepath: &'f str,
    ) -> Self {
        Self {
            users_filepath,
            ratings_filepath,
            alerts_filepath,
        }
    }

    pub fn get_data<D: for<'de> Deserialize<'de>>(&self, file: &'f str) -> Vec<D> {
        let mut rdr = Reader::from_path(file).unwrap();
        let iter = rdr.deserialize();
        let mut vec = vec![];
        iter.fold(&mut vec, |acc, u| {
            if let Ok(d) = u {
                acc.push(d)
            }

            acc
        });

        vec
    }
}

#[async_trait]
impl Database for CSVDatabase<'_> {
    async fn get_users(&self) -> Vec<user::User> {
        self.get_data(self.users_filepath)
    }

    async fn get_alerts(&self) -> Vec<alert::Alert> {
        self.get_data(self.alerts_filepath)
    }

    async fn get_ratings(&self) -> Vec<rating::Rating> {
        self.get_data(self.ratings_filepath)
    }
}
