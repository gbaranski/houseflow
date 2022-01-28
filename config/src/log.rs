const ENV_VAR: &str = "HOUSEFLOW_LOG";

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub hide_timestamp: bool,
}

pub fn init() {
    init_with_config(Config::default());
}

pub fn init_with_config(config: Config) {
    use std::str::FromStr;
    use tracing::Level;

    let env_filter = match std::env::var(ENV_VAR) {
        Ok(env) => env,
        Err(std::env::VarError::NotPresent) => "info".to_string(),
        Err(std::env::VarError::NotUnicode(_)) => panic!(
            "{} environment variable is not valid unicode and can't be read",
            ENV_VAR
        ),
    };
    let level = Level::from_str(&env_filter)
        .unwrap_or_else(|err| panic!("invalid `{}` environment variable {}", ENV_VAR, err));

    let Config { hide_timestamp } = config;

    if hide_timestamp {
        tracing_subscriber::fmt()
            .with_max_level(level)
            .without_time()
            .init()
    } else {
        tracing_subscriber::fmt().with_max_level(level).init()
    };
}
