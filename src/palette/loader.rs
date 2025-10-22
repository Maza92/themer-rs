use anyhow::{Context, Result};
use serde::Serialize;
use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

use super::models::Palette;

pub struct PaletteLoader {
    palettes_dir: PathBuf,
}

impl PaletteLoader {
    pub fn new(config_dir: &Path) -> Self {
        Self {
            palettes_dir: config_dir.join("palettes"),
        }
    }

    pub fn load(&self, palette_name: &str) -> Result<Palette> {
        let palette_file: Cow<str> = if palette_name.ends_with(".json") {
            Cow::Borrowed(palette_name)
        } else {
            Cow::Owned(format!("{}.json", palette_name))
        };

        let palette_path = self.palettes_dir.join(palette_file.as_ref());
        let content = fs::read_to_string(&palette_path)
            .with_context(|| format!("Failed to read palette: {}", palette_path.display()))?;

        let palette: Palette =
            serde_json::from_str(&content).context("Failed to parse palette JSON")?;

        Ok(palette)
    }

    pub fn list_all(&self) -> Result<Vec<PaletteInfo>> {
        let entries = fs::read_dir(&self.palettes_dir).with_context(|| {
            format!("Failed to read directory: {}", self.palettes_dir.display())
        })?;

        let palettes: Vec<PaletteInfo> = entries
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json")
                    && let Some(filename) = path.file_stem().and_then(|s| s.to_str())
                {
                    let name = extract_palette_name(&path).ok();

                    Some(PaletteInfo {
                        filename: filename.to_string(),
                        name,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(palettes)
    }
}
fn extract_palette_name(path: &Path) -> Result<String> {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct NameOnly {
        name: String,
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;

    let name_only: NameOnly =
        serde_json::from_str(&content).context("Failed to parse name from palette JSON")?;

    Ok(name_only.name)
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PaletteInfo {
    pub filename: String,
    pub name: Option<String>,
}

impl std::fmt::Display for PaletteInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(name) => write!(f, "{} ({})", name, self.filename),
            None => write!(f, "{} (invalid)", self.filename),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_palettes() -> (TempDir, PaletteLoader) {
        let temp_dir = TempDir::new().unwrap();
        let palettes_dir = temp_dir.path().join("palettes");
        fs::create_dir(&palettes_dir).unwrap();

        let valid_palette = r#"
        {
            "name": "Test Palette",
            "colors": {
                "primary": "ffffff",
                "secondary": "000000"
            }
        }"#;
        fs::write(palettes_dir.join("test.json"), valid_palette).unwrap();

        fs::write(palettes_dir.join("invalid.json"), "{ invalid json").unwrap();

        let another_palette = r#"{
            "name": "Another Palette",
            "colors": {
                "primary": "ff0000"
            }
        }"#;
        fs::write(palettes_dir.join("another.json"), another_palette).unwrap();

        let loader = PaletteLoader::new(temp_dir.path());
        (temp_dir, loader)
    }

    #[test]
    fn test_new_creates_correct_path() {
        let temp_dir = TempDir::new().unwrap();
        let loader = PaletteLoader::new(temp_dir.path());
        assert_eq!(loader.palettes_dir, temp_dir.path().join("palettes"));
    }

    #[test]
    fn test_load_happy_path() {
        let (_temp_dir, loader) = setup_test_palettes();
        let palette = loader.load("test").unwrap();
        assert_eq!(palette.name, "Test Palette");
    }

    #[test]
    fn test_load_with_json_extension() {
        let (_temp_dir, loader) = setup_test_palettes();
        let palette = loader.load("another.json").unwrap();
        assert_eq!(palette.name, "Another Palette");
    }

    #[test]
    fn test_load_file_not_found() {
        let (_temp_dir, loader) = setup_test_palettes();
        let result = loader.load("nonexistent");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to read palette")
        );
    }

    #[test]
    fn test_load_invalid_json() {
        let (_temp_dir, loader) = setup_test_palettes();
        let result = loader.load("invalid");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to parse palette JSON")
        );
    }

    #[test]
    fn test_list_all_with_palettes() {
        let (_temp_dir, loader) = setup_test_palettes();
        let palettes = loader.list_all().unwrap();
        assert_eq!(palettes.len(), 3);

        // Check valid ones
        let test_info = palettes.iter().find(|p| p.filename == "test").unwrap();
        assert_eq!(test_info.name, Some("Test Palette".to_string()));

        let another_info = palettes.iter().find(|p| p.filename == "another").unwrap();
        assert_eq!(another_info.name, Some("Another Palette".to_string()));

        // Check invalid one
        let invalid_info = palettes.iter().find(|p| p.filename == "invalid").unwrap();
        assert_eq!(invalid_info.name, None);
    }

    #[test]
    fn test_list_all_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let palettes_dir = temp_dir.path().join("palettes");
        std::fs::create_dir(&palettes_dir).unwrap();
        let loader = PaletteLoader::new(temp_dir.path());
        let palettes = loader.list_all().unwrap();
        assert!(palettes.is_empty());
    }

    #[test]
    fn test_list_all_nonexistent_directory() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent_path = temp_dir.path().join("nonexistent");
        let loader = PaletteLoader {
            palettes_dir: nonexistent_path,
        };
        let result = loader.list_all();
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Failed to read directory")
        );
    }

    #[test]
    fn test_list_all_with_non_json_files() {
        let temp_dir = TempDir::new().unwrap();
        let palettes_dir = temp_dir.path().join("palettes");
        fs::create_dir(&palettes_dir).unwrap();
        fs::write(palettes_dir.join("not_json.txt"), "content").unwrap();
        fs::write(
            palettes_dir.join("valid.json"),
            r#"{"name": "Valid", "colors": {}}"#,
        )
        .unwrap();

        let loader = PaletteLoader::new(temp_dir.path());
        let palettes = loader.list_all().unwrap();
        assert_eq!(palettes.len(), 1);
        assert_eq!(palettes[0].filename, "valid");
    }

    #[test]
    fn test_palette_info_display() {
        let info_with_name = PaletteInfo {
            filename: "test".to_string(),
            name: Some("Test Palette".to_string()),
        };
        assert_eq!(info_with_name.to_string(), "Test Palette (test)");

        let info_without_name = PaletteInfo {
            filename: "invalid".to_string(),
            name: None,
        };
        assert_eq!(info_without_name.to_string(), "invalid (invalid)");
    }

    #[test]
    fn test_extract_palette_name() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.json");
        fs::write(&test_file, r#"{"name": "My Palette", "colors": {}}"#).unwrap();

        let name = extract_palette_name(&test_file).unwrap();
        assert_eq!(name, "My Palette");
    }
}
