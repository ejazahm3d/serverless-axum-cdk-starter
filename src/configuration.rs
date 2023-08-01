#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApplicationSettings {
    stage_name: Option<String>,
}

impl ApplicationSettings {
    pub fn get_stage_name(&self) -> String {
        match &self.stage_name {
            Some(name) => match name.is_empty() {
                true => "".into(),
                false => format!("/{}/", name),
            },
            None => "".into(),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    //
    // let configuration_directory = base_path.join("configuration");

    let settings = config::Config::builder()
        // .add_source(config::File::from(
        //     configuration_directory.join("base.yaml"),
        // ))
        // .add_source(config::File::from(
        //     configuration_directory.join("local.yaml"),
        // ))
        // // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<Settings>()
}
