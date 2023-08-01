use axum::response::IntoResponse;

#[utoipa::path(
        post,
        path = "/auth/logout",
        responses(
            (status = 200, description = "Success"),
        ),
        tag = "Auth"
    )]
pub async fn logout() -> impl IntoResponse {
    axum::http::StatusCode::OK
}
