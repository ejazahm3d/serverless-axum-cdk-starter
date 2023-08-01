use axum::response::IntoResponse;

#[utoipa::path(
        get,
        path = "/health-check",
        responses(
            (status = 200, description = "Success"),
        ),
        tag = "Health Check"
    )]
pub async fn health_check() -> impl IntoResponse {
    axum::http::StatusCode::OK
}
