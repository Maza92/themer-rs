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
