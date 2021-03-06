use recommender::collaborative_filtering;

use crate::services::recommender::types::recommender;

pub async fn handler(
    recommender: &crate::recommender::Recommender,
    request: collaborative_filtering::Input,
) -> collaborative_filtering::Output {
    let request = request.into_inner();

    let alerts = recommender
        .collaborative_filtering(
            request.user_id as u32,
            request.alerts_number.unwrap_or(20) as u16,
        )
        .await
        .map_err(|e| tonic::Status::internal(e.to_string()))?;
    Ok(tonic::Response::new(collaborative_filtering::Response {
        alerts,
    }))
}
