use models::recommender::Recommender;
use services::{
    database::{alerts::Alerts, users::Users, Database},
    recommender::RecommenderService,
};
use tonic::transport::Server;

mod config;
mod models;
mod recommender;
mod redis;
mod resources;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let config = config::Config::from_env().unwrap();

    let database = Database::connect(config.db.get_connection_url().as_str()).await?;
    let redis = redis::Redis::new(config.redis);

    let mut recommender = recommender::Recommender::new(redis, database);
    recommender.load_data().await.unwrap();
    let recommender = RecommenderService::new(recommender);

    Server::builder()
        .add_service(recommender.service())
        .serve(config.server.get_url().parse().unwrap())
        .await?;

    Ok(())
}
