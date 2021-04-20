#[cfg(feature = "http")]
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

mod database;
mod models;

use database::{csv::CSVDatabase, Database};
use models::recommender::RecommenderBuilder;

#[cfg(feature = "http")]
use models::recommender::Recommender;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Request {
    user_id: u32,
}

#[cfg(feature = "http")]
#[get("/{user_id}/content_based")]
async fn content_based(
    request: web::Path<Request>,
    recommender: web::Data<Recommender>,
) -> impl Responder {
    HttpResponse::Ok().json(recommender.content_based(request.user_id, 20, None))
}
#[cfg(feature = "http")]
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {

    let dataset_dir = r"../dataset/8Kratings100users500alerts";
    let users = format!("{}/users.csv", dataset_dir);
    let ratings = format!("{}/ratings.csv", dataset_dir);
    let alerts = format!("{}/../alerts.csv", dataset_dir);

    let csv = CSVDatabase::new(users.as_str(), ratings.as_str(), alerts.as_str());

    let (users, ratings, mut alerts) =
        futures::join!(csv.get_users(), csv.get_ratings(), csv.get_alerts());

    // println!("users: {:?}", users[0]);
    // println!("ratings: {:?}", ratings[0]);
    // println!("alerts: {:?}", alerts[0]);

    let recommender = RecommenderBuilder::build(users, &mut alerts, ratings).await;

    println!("Listen on http://localhost:8080");

    HttpServer::new(move || App::new().data(recommender.clone()).service(content_based))
        .bind("127.0.0.1:8080")?
        .run()
        .await?;

    Ok(())
}

#[cfg(not(feature = "http"))]
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    let dataset_dir = r"../dataset/8Kratings100users500alerts";
    let users = format!("{}/users.csv", dataset_dir);
    let ratings = format!("{}/ratings.csv", dataset_dir);
    let alerts = format!("{}/../alerts.csv", dataset_dir);

    let csv = CSVDatabase::new(users.as_str(), ratings.as_str(), alerts.as_str());

    let (users, ratings, mut alerts) =
        futures::join!(csv.get_users(), csv.get_ratings(), csv.get_alerts());

    // println!("users: {:?}", users[0]);
    // println!("ratings: {:?}", ratings[0]);
    // println!("alerts: {:?}", alerts[0]);

    let recommender = RecommenderBuilder::build(users, &mut alerts, ratings).await;

    // println!("users {:#?}", recommender.users.get(&1u32));
    println!("content based: {:#?}", recommender.content_based(1 as u32, 10 as u16, None));

    Ok(())
}