use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::config::models::Config;

pub struct ConfigLoader {
    pub config_dir: PathBuf,
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

#[cfg(test)]
mod tests {
    use crate::config::models::{Mode, Target};

    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_config_dir() -> TempDir {
        TempDir::new().unwrap()
    }

    fn create_test_loader(temp_dir: &TempDir) -> ConfigLoader {
        ConfigLoader {
            config_dir: temp_dir.path().to_path_buf(),
        }
    }

    fn create_valid_config_content() -> String {
        r#"
active_palette = "nord"

[[targets]]
name = "alacritty"
template = "alacritty.tmpl"
output = "~/.config/alacritty/colors.toml"
mode = "include"
reload_cmd = "touch ~/.config/alacritty/alacritty.toml"
"#
        .to_string()
    }

    #[test]
    fn test_config_loader_new() {
        // Act
        let result = ConfigLoader::new();

        // Assert
        assert!(result.is_ok());
        let loader = result.unwrap();
        assert!(loader.config_dir.ends_with("themer"));
    }

    #[test]
    fn test_config_dir_returns_correct_path() {
        // Arrange
        let temp_dir = create_test_config_dir();
        let loader = create_test_loader(&temp_dir);

        // Act
        let config_dir = loader.config_dir();

        // Assert
        assert_eq!(config_dir, temp_dir.path());
    }

    #[test]
    fn test_load_valid_config() {
        // Arrange
        let temp_dir = create_test_config_dir();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(&config_path, create_valid_config_content()).unwrap();
        let loader = create_test_loader(&temp_dir);

        // Act
        let result = loader.load();

        // Assert
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.active_palette, "nord");
        assert_eq!(config.targets.len(), 1);
        assert_eq!(config.targets[0].name, "alacritty");
    }

    #[test]
    fn test_load_missing_config_file() {
        // Arrange
        let temp_dir = create_test_config_dir();
        let loader = create_test_loader(&temp_dir);

        // Act
        let result = loader.load();

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Failed to read"));
    }

    #[test]
    fn test_load_invalid_toml() {
        // Arrange
        let temp_dir = create_test_config_dir();
        let config_path = temp_dir.path().join("config.toml");
        fs::write(&config_path, "invalid toml content {{").unwrap();
        let loader = create_test_loader(&temp_dir);

        // Act
        let result = loader.load();

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Failed to parse"));
    }

    #[test]
    fn test_save_config() {
        // Arrange
        let temp_dir = create_test_config_dir();
        let loader = create_test_loader(&temp_dir);
        let config = Config {
            active_palette: "catppuccin".to_string(),
            targets: vec![Target {
                name: "kitty".to_string(),
                template: "kitty.tmpl".to_string(),
                output: "~/.config/kitty/colors.conf".to_string(),
                mode: Mode::Replace,
                reload_cmd: "kill -SIGUSR1 $(pgrep kitty)".to_string(),
            }],
        };

        // Act
        let result = loader.save(&config);

        // Assert
        assert!(result.is_ok());
        let config_path = temp_dir.path().join("config.toml");
        assert!(config_path.exists());

        let content = fs::read_to_string(&config_path).unwrap();
        assert!(content.contains("catppuccin"));
        assert!(content.contains("kitty"));
    }

    #[test]
    fn test_save_and_load_roundtrip() {
        // Arrange
        let temp_dir = create_test_config_dir();
        let loader = create_test_loader(&temp_dir);
        let original_config = Config {
            active_palette: "dracula".to_string(),
            targets: vec![
                Target {
                    name: "zsh".to_string(),
                    template: "zsh.tmpl".to_string(),
                    output: "~/.zshrc.colors".to_string(),
                    mode: Mode::Include,
                    reload_cmd: "source ~/.zshrc".to_string(),
                },
                Target {
                    name: "tmux".to_string(),
                    template: "tmux.tmpl".to_string(),
                    output: "~/.tmux.conf.colors".to_string(),
                    mode: Mode::Replace,
                    reload_cmd: "tmux source ~/.tmux.conf".to_string(),
                },
            ],
        };

        // Act
        loader.save(&original_config).unwrap();
        let loaded_config = loader.load().unwrap();

        // Assert
        assert_eq!(original_config.active_palette, loaded_config.active_palette);
        assert_eq!(original_config.targets.len(), loaded_config.targets.len());
        assert_eq!(
            original_config.targets[0].name,
            loaded_config.targets[0].name
        );
        assert_eq!(
            original_config.targets[1].mode,
            loaded_config.targets[1].mode
        );
    }

    #[test]
    fn test_save_empty_config() {
        // Arrange
        let temp_dir = create_test_config_dir();
        let loader = create_test_loader(&temp_dir);
        let config = Config {
            active_palette: String::new(),
            targets: vec![],
        };

        // Act
        let result = loader.save(&config);

        // Assert
        assert!(result.is_ok());
    }
}
