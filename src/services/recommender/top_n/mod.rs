use recommender::top_n;

use crate::services::recommender::types::recommender;

pub async fn handler(
    recommender: &crate::models::recommender::Recommender,
    request: top_n::Input,
) -> top_n::Output {
    let request = request.into_inner();

    let alerts =
        recommender.top_n(request.alerts_number.unwrap_or(20), request.content);
    Ok(tonic::Response::new(top_n::Response {
        alerts: alerts.iter().map(Into::into).collect(),
    }))
}
