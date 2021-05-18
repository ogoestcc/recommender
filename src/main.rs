use models::recommender::Recommender;
use services::{
    database::{alerts::Alerts, users::Users},
    recommender::RecommenderService,
};
use tonic::transport::Server;

mod config;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();

    let mut users = Users::connect(config.db.get_connection_url().as_str()).await?;
    let mut alerts = Alerts::connect(config.db.get_connection_url().as_str()).await?;

    let (users, alerts) = futures::join!(users.get_all_users(), alerts.get_all_alerts());

    let recommender = RecommenderService::new(Recommender { users, alerts });

    Server::builder()
        .add_service(recommender.service())
        .serve(config.server.get_url().parse().unwrap())
        .await?;

    Ok(())
}
