use utoipa::{openapi::Server, Modify, OpenApi};

use crate::configuration::get_configuration;

#[derive(OpenApi)]
#[openapi(
    modifiers(&ServerAddon),
    tags(
        (name = "Health Check", description = "Application Health Check"),
        (name = "Auth", description = "Auth Endpoints"),
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
