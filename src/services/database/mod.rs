use super::protos::database;

pub mod alerts;
pub mod users;

pub mod services {
    use super::database;

    pub mod users {
        use super::database;
        pub use database::users_client::UsersClient as Client;

        pub use database::{get_users, get_users_and_ratings};

        pub mod get_users_and_contents {
            use super::database;

            pub use database::{get_alerts_and_ratings, get_users::Request};
        }
    }

    pub mod alerts {
        use super::database;

        pub use database::alerts_client::AlertsClient as Client;

        pub use database::{get_alerts, get_alerts_and_ratings};
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pub users: services::users::Client<tonic::transport::Channel>,
    pub alerts: services::alerts::Client<tonic::transport::Channel>,
    endpoint: String,
}

impl Database {
    pub async fn connect<U: ToString>(url: U) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = url.to_string();
        Ok(Self {
            users: services::users::Client::connect(url.to_string()).await?,
            alerts: services::alerts::Client::connect(url.to_string()).await?,
            endpoint,
        })
    }
}
