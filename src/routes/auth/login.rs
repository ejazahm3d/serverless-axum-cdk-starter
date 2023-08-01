use crate::io::error::AppError;

use axum::{response::IntoResponse, Json};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(serde::Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct LoginRequest {
    pub(crate) email: String,
    pub(crate) password: String,
}

#[derive(serde::Serialize, ToSchema)]
pub struct LoginResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
}

#[utoipa::path(
        post,
        path = "/auth/login",
        request_body = LoginRequest,
        responses(
            (status = 200, description = "Successfully Logged In", body = LoginResponse),
        ),
        tag = "Auth"
    )]
pub async fn login() -> Result<impl IntoResponse, AppError> {
    Ok((
        axum::http::StatusCode::OK,
        Json(LoginResponse {
            id: Uuid::new_v4(),
            email: "".into(),
            username: "".into(),
        }),
    ))
}
