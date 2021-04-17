
mod database;
mod models;

use database::{csv::CSVDatabase, Database};

#[tokio::main]
async fn main() {
    let dataset_dir = r"../dataset/8Kratings100users500alerts";
    let users = format!("{}/users.csv", dataset_dir);
    let ratings = format!("{}/ratings.csv", dataset_dir);
    let alerts = format!("{}/../alerts.csv", dataset_dir);

    let csv = CSVDatabase::new(users.as_str(), ratings.as_str(), alerts.as_str());

    let (users, ratings, alerts) = futures::join!(csv.get_users(), csv.get_ratings(), csv.get_alerts());

    println!("users: {:?}", users[0]);
    println!("ratings: {:?}", ratings[0]);
    println!("alerts: {:?}", alerts[0]);

}
