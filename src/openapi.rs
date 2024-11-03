use utoipa::{openapi::Server, Modify, OpenApi};

use crate::configuration::get_configuration;
#[derive(OpenApi)]
#[openapi(
    modifiers(&ServerAddon),
    paths(
        crate::routes::health_check,
        crate::routes::auth::login,
        crate::routes::auth::sign_up,
        crate::routes::auth::current_user,
        crate::routes::auth::logout,
    ),
)]
pub struct ApiDoc;

struct ServerAddon;

impl Modify for ServerAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let config = get_configuration().expect("Require config for openapi");
        let stage_name = config.application.get_stage_name();

        let uri = match stage_name.is_empty() {
            true => String::from("/api"),
            false => format!("{}api", config.application.get_stage_name()),
        };

        openapi.servers = Some(vec![Server::new(uri)])
    }
}
