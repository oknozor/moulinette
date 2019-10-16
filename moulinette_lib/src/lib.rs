use config::Config;

pub fn get_config() -> MoulinetteConfig {
    let mut config = Config::new();
    config.merge(config::File::with_name("moulinette")).unwrap();

    let config = config;
    MoulinetteConfig::from(config)
}

#[derive(Debug)]
pub struct MoulinetteConfig {
    pub language: String,
    pub script: String,
}

impl From<Config> for MoulinetteConfig {
    fn from(config: Config) -> Self {
        MoulinetteConfig {
            language: config.get_str("language").unwrap(),
            script: config.get_str("script").unwrap(),
        }
    }
}
