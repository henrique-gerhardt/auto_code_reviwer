use config::{Config, ConfigError, Environment, File};
use std::io::Write;


const CONFIG_FILE: &str = "config.json";
const ENV_PREFIX: &str = "code_reviewer";

#[derive(Debug, serde::Deserialize, serde::Serialize, Default)]
pub struct AppConfig {
    gitlab_token: String,
    github_token: String,
    openai_token: String,
}

impl AppConfig {
    pub fn save_gitlab_token(&mut self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.gitlab_token = token.to_string();
        self.save_to_file()
    }

    pub fn save_github_token(&mut self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.github_token = token.to_string();
        self.save_to_file()
    }

    pub fn save_openai_token(&mut self, token: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.openai_token = token.to_string();
        self.save_to_file()
    }

    pub fn get_gitlab_token(&self) -> Option<&str> {
        if self.gitlab_token.is_empty() {
            None
        } else {
            Some(&self.gitlab_token)
        }
    }

    pub fn get_github_token(&self) -> Option<&str> {
        if self.github_token.is_empty() {
            None
        } else {
            Some(&self.github_token)
        }
    }

    pub fn get_openai_token(&self) -> Option<&str> {
        if self.openai_token.is_empty() {
            None
        } else {
            Some(&self.openai_token)
        }
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(CONFIG_FILE)?;
        let json = serde_json::to_string_pretty(self)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}



pub fn load_config() -> Result<AppConfig, ConfigError> {

    if !std::path::Path::new(CONFIG_FILE).exists() {
        return Ok(AppConfig::default());
    }
    
    let config = Config::builder()
        .add_source(File::with_name(CONFIG_FILE))
        .add_source(Environment::with_prefix(ENV_PREFIX))
        .build()?;

    Ok(config.try_deserialize()?)
}