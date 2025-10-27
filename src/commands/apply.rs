use anyhow::{Context, Result};

use crate::config::loader::ConfigLoader;
use crate::output::output;
use crate::palette::loader::PaletteLoader;
use crate::target::processor::TargetProcessor;
use crate::template::engine::TemplateEngine;

pub fn execute(palette_name: &str) -> Result<()> {
    output::header(&format!("Applying palette: {}", palette_name));

    let config_loader = ConfigLoader::new()?;
    let mut config = config_loader.load()?;

    let palette_loader = PaletteLoader::new(config_loader.config_dir());

    let palette = palette_loader
        .load(palette_name)
        .with_context(|| format!("Palette '{}' not found", palette_name))?;

    if config.targets.is_empty() {
        output::warning("No targets configured");
        return Ok(());
    }

    let engine = TemplateEngine::new();
    let context = engine.create_context(&palette)?;
    let mut processor = TargetProcessor::new(config_loader.config_dir());

    for target in &config.targets {
        if let Err(e) = processor.process(target, &context, &palette) {
            output::error(&format!("Failed to process {}: {}", target.name, e));
        }
    }

    config.active_palette = palette_name.to_string();
    config_loader.save(&config)?;

    output::success("Theme applied successfully!");
    Ok(())
}
