use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::config::models::Config;

pub struct ConfigLoader {
    config_dir: PathBuf,
}

impl ConfigLoader {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("themer");
        Ok(Self { config_dir })
    }

    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    pub fn load(&self) -> Result<Config> {
        let config_path = self.config_dir.join("config.toml");
        let content = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;

        toml::from_str(&content).context("Failed to parse config.toml")
    }

    pub fn save(&self, config: &Config) -> Result<()> {
        let config_path = self.config_dir.join("config.toml");
        let content = toml::to_string_pretty(config)?;

        fs::write(&config_path, content)
            .with_context(|| format!("Failed to write {}", config_path.display()))?;

        Ok(())
    }
}
