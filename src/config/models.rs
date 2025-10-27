use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub active_palette: String,
    pub targets: Vec<Target>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Target {
    pub name: String,
    pub template: String,
    pub output: String,
    pub mode: Mode,
    pub reload_cmd: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Include,
    Replace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_serialization() {
        // Arrange
        let config = Config {
            active_palette: "gruvbox".to_string(),
            targets: vec![Target {
                name: "neovim".to_string(),
                template: "nvim".to_string(),
                output: "colors.lua".to_string(),
                mode: Mode::Replace,
                reload_cmd: "echo 'reloaded'".to_string(),
            }],
        };

        // Act
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(config.active_palette, deserialized.active_palette);
        assert_eq!(config.targets.len(), deserialized.targets.len());
    }

    #[test]
    fn test_mode_serialization() {
        // Arrange
        let replace = Mode::Replace;
        let include = Mode::Include;

        // Act
        let replace_str = serde_json::to_string(&replace).unwrap();
        let include_str = serde_json::to_string(&include).unwrap();

        // Assert
        assert_eq!(replace_str, r#""replace""#);
        assert_eq!(include_str, r#""include""#);
    }

    #[test]
    fn test_mode_deserialization() {
        // Arrange
        let replace_json = r#""replace""#;
        let include_json = r#""include""#;

        // Act
        let replace: Mode = serde_json::from_str(replace_json).unwrap();
        let include: Mode = serde_json::from_str(include_json).unwrap();

        // Assert
        assert_eq!(replace, Mode::Replace);
        assert_eq!(include, Mode::Include);
    }

    #[test]
    fn test_mode_ordering() {
        // Act & Assert
        assert!(Mode::Include < Mode::Replace);
        assert_eq!(Mode::Replace, Mode::Replace);
    }

    #[test]
    fn test_empty_config() {
        // Arrange
        let config = Config {
            active_palette: String::new(),
            targets: vec![],
        };

        // Act
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(deserialized.active_palette, "");
        assert_eq!(deserialized.targets.len(), 0);
    }

    #[test]
    fn test_target_with_special_characters() {
        // Arrange
        let target = Target {
            name: "test-target_123".to_string(),
            template: "template-name".to_string(),
            output: "/absolute/path/output.conf".to_string(),
            mode: Mode::Include,
            reload_cmd: "systemctl restart service && echo 'done'".to_string(),
        };

        // Act
        let serialized = toml::to_string(&target).unwrap();
        let deserialized: Target = toml::from_str(&serialized).unwrap();

        // Assert
        assert_eq!(target.name, deserialized.name);
        assert_eq!(target.reload_cmd, deserialized.reload_cmd);
    }
}
