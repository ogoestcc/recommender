use recommender::content_based;

use crate::services::recommender::types::recommender;

pub async fn handler(
    recommender: &crate::models::recommender::Recommender,
    request: content_based::Input,
) -> content_based::Output {
    let request = request.into_inner();

    let alerts =
        recommender.content_based(request.user_id as u32, request.alerts_number.unwrap_or(20) as u16, None);
    Ok(tonic::Response::new(content_based::Response {
        alerts: alerts.iter().map(Into::into).collect(),
    }))
}
