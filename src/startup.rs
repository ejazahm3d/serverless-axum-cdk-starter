use std::sync::Arc;

use axum::{
    extract::FromRef,
    http::{header, Method},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;

use crate::{
    configuration::Settings,
    openapi::ApiDoc,
    routes::{auth, rapidoc},
};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Settings>,
    pub openapi: Arc<utoipa::openapi::OpenApi>,
}

impl FromRef<AppState> for Arc<Settings> {
    fn from_ref(input: &AppState) -> Self {
        input.config.clone()
    }
}

impl FromRef<AppState> for Arc<utoipa::openapi::OpenApi> {
    fn from_ref(input: &AppState) -> Self {
        input.openapi.clone()
    }
}

pub fn make_app(config: Settings) -> Router {
    let openapi = ApiDoc::openapi();

    let auth_routes = Router::new()
        .route("/signup", post(auth::sign_up))
        .route("/login", post(auth::login))
        .route("/logout", post(auth::logout))
        .route("/current", get(auth::current_user));

    let rapidoc_routes = Router::new()
        .route("/api-doc/openapi.json", get(rapidoc::openapi_json))
        .route("/rapidoc/*path", get(rapidoc::rapidoc));

    let cors_layer = CorsLayer::default()
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_headers([header::ACCEPT, header::CONTENT_TYPE]);

    Router::new()
        .route("/api/health-check", get(crate::routes::health_check))
        .nest("/api", Router::new().nest("/auth", auth_routes))
        .nest("", rapidoc_routes)
        .with_state(AppState {
            config: Arc::new(config),
            openapi: Arc::new(openapi),
        })
        .layer(cors_layer)
}
