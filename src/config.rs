use dotenv::dotenv;
use ron;
use serde::{Deserialize, Serialize};
use serenity::model::id::ChannelId;
use std::env;
use std::fs;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Configuration {
    pub enabled_categories: Vec<ChannelId>,
}

// TODO: I should probably use something like toml instead of RON...
pub fn load_config() -> Configuration {
    dotenv().ok();
    let config_path = env::var("CONFIG_FILE").expect("$CONFIG_FILE must be set");
    let config_contents = fs::read_to_string(&config_path)
        .expect(&format!("Error occurred while reading {}", config_path));
    return ron::from_str(&config_contents).expect(&format!(
        "Error trying to parse config RON as Configuration.: {}",
        config_contents
    ));
}

// mostly for writing a template to disk rather than trying to figure out RON by hand
pub fn write_config(config: Option<Configuration>) {
    dotenv().ok();
    let config_path = env::var("CONFIG_FILE").expect("CONFIG_FILE must be set");
    let mut config = config.unwrap_or(Configuration::default());
    config
        .enabled_categories
        .push(ChannelId(782789175941529612));

    let _ = fs::write(config_path, ron::to_string(&config).unwrap());
}
