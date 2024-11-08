use figment::providers::Env;
use figment::Figment;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bind_address: String
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:3000".to_string()
        }
    }
}

impl Config {
    pub fn from_figment() -> Result<Self, figment::Error> {
        Figment::new().merge(Env::raw()).extract()
    }
}