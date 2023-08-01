use std::sync::Arc;

use axum::{
    debug_handler,
    extract::State,
    response::{Html, IntoResponse},
    Json,
};

use crate::{configuration::Settings, io::error::AppError};

pub async fn rapidoc(State(config): State<Arc<Settings>>) -> Result<impl IntoResponse, AppError> {
    let stage_name = match config.application.get_stage_name().is_empty() {
        true => "/".into(),
        false => config.application.get_stage_name(),
    };

    let file = format!(
        r#"
<!DOCTYPE html>
<!-- Important: must specify -->
<html>
  <head>
    <meta charset="utf-8" />
    <!-- Important: rapi-doc uses utf8 characters -->
    <script
      type="module"
      src="https://unpkg.com/rapidoc/dist/rapidoc-min.js"
    ></script>
  </head>
  <body>
    <rapi-doc spec-url="{stage_name}api-doc/openapi.json"> </rapi-doc>
  </body>
</html>
    "#,
    );

    Ok(Html(file))
}

#[debug_handler]
pub async fn openapi_json(
    State(openapi): State<Arc<utoipa::openapi::OpenApi>>,
) -> Result<impl IntoResponse, AppError> {
    Ok((axum::http::StatusCode::OK, Json(openapi)))
}
