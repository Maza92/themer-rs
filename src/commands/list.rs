use anyhow::Result;

use crate::config::loader::ConfigLoader;
use crate::output::output;
use crate::palette::loader::{PaletteInfo, PaletteLoader};

pub fn execute(format: Option<&str>) -> Result<()> {
    let config_loader = ConfigLoader::new()?;
    let palette_loader = PaletteLoader::new(config_loader.config_dir());
    let palettes = palette_loader.list_all()?;

    match format {
        Some("plain") => {
            for info in palettes {
                println!("{}", info.filename);
            }
        }
        Some("json") => {
            let infos: Vec<PaletteInfo> = palettes
                .into_iter()
                .map(|info| PaletteInfo {
                    filename: info.filename,
                    name: info.name,
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&infos)?);
        }
        _ => {
            output::header("Available palettes:");

            for info in palettes {
                output::item(Some("Palette"), &info.filename, info.name.as_deref());
            }
        }
    }

    Ok(())
}
