mod collaborative_filtering;
mod content_based;
mod top_n;
use crate::recommender;

use collaborative_filtering::handler as collaborative_filtering_handler;
use content_based::handler as content_based_handler;
use top_n::handler as top_n_handler;

use super::protos::recommender as protos;

#[derive(Debug)]
pub struct RecommenderService {
    recommender_data: recommender::Recommender,
}

impl RecommenderService {
    pub fn new(recommender_data: recommender::Recommender) -> Self {
        Self { recommender_data }
    }

    pub fn service(self) -> types::recommender::Server<Self> {
        types::recommender::Server::new(self)
    }
}

#[async_trait::async_trait]
impl types::recommender::Service for RecommenderService {
    async fn content_based(
        &self,
        request: types::recommender::content_based::Input,
    ) -> types::recommender::content_based::Output {
        content_based_handler(&self.recommender_data, request).await
    }

    async fn collaborative_filtering(
        &self,
        request: types::recommender::collaborative_filtering::Input,
    ) -> types::recommender::collaborative_filtering::Output {
        collaborative_filtering_handler(&self.recommender_data, request).await
    }

    async fn top_n(
        &self,
        request: types::recommender::top_n::Input,
    ) -> types::recommender::top_n::Output {
        top_n_handler(&self.recommender_data, request).await
    }

    async fn load_user_data(
        &self,
        request: types::recommender::load_data::Input,
    ) -> types::recommender::load_data::Output {
        self.recommender_data
            .clone()
            .load_user_data(request.into_inner().user_id)
            .await
            .map_err(|e| tonic::Status::internal(e.to_string()))?;

        Ok(tonic::Response::new(Default::default()))
    }
}

pub mod types {
    use super::protos;

    pub mod recommender {
        use super::protos;

        pub use protos::{
            recommender_server::Recommender as Service,
            recommender_server::RecommenderServer as Server,
        };

        pub mod content_based {
            use super::protos;

            pub use protos::content_based::{Request, Response};

            pub type Input = tonic::Request<Request>;
            pub type Output = Result<tonic::Response<Response>, tonic::Status>;
        }

        pub mod collaborative_filtering {
            use super::protos;

            pub use protos::collaborative_filtering::{Request, Response};

            pub type Input = tonic::Request<Request>;
            pub type Output = Result<tonic::Response<Response>, tonic::Status>;
        }

        pub mod top_n {
            use super::protos;

            pub use protos::top_n::{Request, Response};

            pub type Input = tonic::Request<Request>;
            pub type Output = Result<tonic::Response<Response>, tonic::Status>;
        }

        pub mod load_data {
            use super::protos;

            pub use protos::load_data::{Request, Response};

            pub type Input = tonic::Request<Request>;
            pub type Output = Result<tonic::Response<Response>, tonic::Status>;
        }
    }
}
