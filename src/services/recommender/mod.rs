mod content_based;
use crate::models::recommender;

use content_based::handler as content_based_handler;

use super::protos::recommender as protos;

#[derive(Debug)]
pub struct RecommenderService {
    recommender_data: recommender::Recommender,
}

impl RecommenderService {
    pub fn new(recommender_data: recommender::Recommender) -> Self {
        Self { recommender_data }
    }

    pub fn service(self) -> types::recommender::content_based::Server<Self> {
        types::recommender::content_based::Server::new(self)
    }
}

#[async_trait::async_trait]
impl types::recommender::content_based::Service for RecommenderService {
    async fn content_based(
        &self,
        request: types::recommender::content_based::Input,
    ) -> types::recommender::content_based::Output {
        content_based_handler(&self.recommender_data, request).await
    }
}

pub mod types {
    use super::protos;

    // pub mod alerts {
    //     use super::{proto, types};

    //     pub use types::Alert;

    //     impl<T: Into<Alert> + Clone> From<&T> for Alert {
    //         fn from(value: &T) -> Self {
    //             value.clone().into()
    //         }
    //     }

    //     impl From<crate::models::alert::Alert> for Alert {
    //         fn from(alert: crate::models::alert::Alert) -> Self {
    //             Self {
    //                 id: alert.id,
    //                 cvss_score: Some(alert.cvss_score),
    //                 provider: alert.provider,
    //                 product: alert.product,
    //                 // published_at: alert.published_at,
    //                 description: alert.description,
    //                 ..Default::default()
    //             }
    //         }
    //     }

    //     // impl Into<crate::models::alerts::Alert> for &Alert {
    //     //     fn into(self) -> crate::services::recommender::types::alerts::Alert {
    //     //         crate::services::recommender::types::alerts::Alert {
    //     //             id: self.id,
    //     //             cvss_score: Some(self.cvss_score),
    //     //             provider: self.provider,
    //     //             product: self.product,
    //     //             published_at: "".into(),
    //     //             description: self.description,
    //     //         }
    //     //     }
    //     // }
    // }

    pub mod recommender {
        use super::protos;

        pub mod content_based {
            use super::protos;

            pub use protos::{
                content_based::{Request, Response},
                recommender_server::Recommender as Service,
                recommender_server::RecommenderServer as Server,
            };

            pub type Input = tonic::Request<Request>;
            pub type Output = Result<tonic::Response<Response>, tonic::Status>;
        }
    }
}
