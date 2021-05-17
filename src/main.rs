
use tonic::transport::Server;
// mod database;
mod models;
pub mod services;

#[cfg(not(feature = "http"))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut users = services::database::users::Users::connect("http://[::1]:9092").await?;

    let mut alerts = services::database::alerts::Alerts::connect("http://[::1]:9092").await?;

    let (users, alerts) = futures::join!(users.get_all_users(), alerts.get_all_alerts());

    let recommender = models::recommender::Recommender { users, alerts };

    let recommender = services::recommender::RecommenderService::new(recommender);

    Server::builder()
        .add_service(recommender.service())
        .serve(format!("[::1]:{}", 10000).parse().unwrap())
        .await?;

    Ok(())
}

// #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
