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
        Some("preview") => output_preview(&palette_loader, &palettes)?,
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
    let names: Vec<&str> = palettes.iter().map(|p| p.filename.as_str()).collect();
    println!("{}", serde_json::to_string_pretty(&names)?);
    Ok(())
}

fn output_preview(
    palette_loader: &PaletteLoader,
    palettes: &[crate::palette::loader::PaletteInfo],
) -> Result<()> {
    let mut json_output = serde_json::Map::new();

    for palette_info in palettes {
        let display_name = palette_info
            .name
            .as_ref()
            .map(|n| format_display_name(n))
            .or_else(|| Some(format_display_name(&palette_info.filename)))
            .unwrap_or_default();

        let preview_colors = palette_loader
            .get_preview_colors(&palette_info.filename)
            .unwrap_or_default();

        let theme_data = serde_json::json!({
            "display": display_name,
            "preview": preview_colors,
        });

        json_output.insert(palette_info.filename.clone(), theme_data);
    }

    println!("{}", serde_json::to_string_pretty(&json_output)?);
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

fn format_display_name(name: &str) -> String {
    name.replace('-', " ")
        .replace('_', " ")
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
