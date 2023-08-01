use std::usize;

use axum::response::IntoResponse;
use axum::Json;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::io::error::AppError;

#[derive(serde::Deserialize, ToSchema)]
#[allow(dead_code)]
pub struct SignUpRequest {
    pub(crate) email: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) avatar: Option<String>,
}

#[derive(serde::Serialize, ToSchema)]
pub struct SignUpResponse {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) username: String,
}

#[utoipa::path(
        post,
        path = "/auth/signup",
        request_body = SignUpRequest,
        responses(
            (status = 201, description = "Successfully Signed Up", body = SignUpResponse),
        ),
        tag = "Auth"
    )]
pub async fn sign_up() -> Result<impl IntoResponse, AppError> {
    Ok((
        axum::http::StatusCode::CREATED,
        Json(SignUpResponse {
            id: Uuid::new_v4(),
            email: "".into(),
            username: "".into(),
        }),
    ))
}
