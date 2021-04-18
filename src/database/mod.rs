pub use async_trait::async_trait;

use super::models::{alert::Alert, rating::Rating, user::User};

pub mod csv;
#[async_trait]
pub trait Database {
    async fn get_users(&self) -> Vec<User>;
    async fn get_alerts(&self) -> Vec<Alert>;
    async fn get_ratings(&self) -> Vec<Rating>;
}
