use anyhow::anyhow;
use clap::Args;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone, Args)]
pub struct Config {
    #[serde(default = "default_url")]
    pub url: String,
    #[serde(default)]
    pub exclude_allergens: Vec<String>,
    pub vegan: Option<bool>,
    pub vegetarian: Option<bool>,
    #[serde(default)]
    pub location_codes: Vec<String>,
    pub language: Option<String>,
    pub price_category: Option<String>,
}

impl Config {
    pub fn read_from_file<'a>(path: &'a str) -> anyhow::Result<Self> {
        let file_content =
            fs::read_to_string(path).map_err(|_| anyhow!(format!("Please provide the config file at '{}'\nYou can find an example at https://github.com/Draculente/speiseplan-cli", path)))?;
        let config: Config = toml::from_str(file_content.as_str())?;

        Ok(config)
    }
}

fn default_url() -> String {
    "https://speiseplan.mcloud.digital/v2".to_owned()
}
