use super::protos::database;

pub mod alerts;
pub mod users;

pub mod services {
    use super::database;

    pub mod users {
        use super::database;
        pub use database::users_client::UsersClient as Client;

        pub use database::{get_users_and_ratings, get_users};

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
