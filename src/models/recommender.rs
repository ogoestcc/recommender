use super::{user::User, alert::Alert};

#[derive(Debug)]
pub struct Recommender<'b> {
    users: Vec<&'b User>,
    alerts: Vec<&'b Alert>,
}





