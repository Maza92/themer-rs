use anyhow::Result;

use crate::config::loader::ConfigLoader;
use crate::output::output;
use crate::palette::loader::PaletteLoader;

pub fn execute(format: Option<&str>) -> Result<()> {
    let config_loader = ConfigLoader::new()?;
    let palette_loader = PaletteLoader::new(config_loader.config_dir());
    let palettes = palette_loader.list_all()?;

    match format {
        Some("plain") => output_plain(&palettes),
        Some("json") => output_json(&palettes)?,
        Some(unknown) => {
            output::warning(&format!("Unknown format '{}', using default", unknown));
            output_default(&palettes)
        }
        None => output_default(&palettes),
    }

    Ok(())
}

fn output_plain(palettes: &[crate::palette::loader::PaletteInfo]) {
    for info in palettes {
        println!("{}", info.filename);
    }
}

fn output_json(palettes: &[crate::palette::loader::PaletteInfo]) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(palettes)?);
    Ok(())
}

fn output_default(palettes: &[crate::palette::loader::PaletteInfo]) {
    output::header("Available palettes:");

    if palettes.is_empty() {
        output::warning("No palettes found");
        return;
    }

    for info in palettes {
        output::item(Some("Palette"), &info.filename, info.name.as_deref());
    }
}
