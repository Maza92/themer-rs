use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::config::loader::ConfigLoader;
use crate::config::models::Target;
use crate::output::output;
use crate::palette::models::{Base16, Base30, Palette};
use crate::template::engine::TemplateEngine;

pub fn execute(target_name: Option<&str>) -> Result<()> {
    let config_loader = ConfigLoader::new()?;
    let config = config_loader.load()?;

    match target_name {
        Some(name) => validate_single_target(&config_loader, &config.targets, name),
        None => validate_all_targets(&config_loader, &config.targets),
    }
}

fn validate_all_targets(config_loader: &ConfigLoader, targets: &[Target]) -> Result<()> {
    output::header("Validating all targets...");

    if targets.is_empty() {
        output::warning("No targets configured");
        return Ok(());
    }

    let mut validation_results = Vec::new();
    let config_dir = config_loader.config_dir();

    for target in targets {
        let result = validate_target_template(config_dir, target);
        validation_results.push((target.name.clone(), result));
    }

    let mut errors = Vec::new();

    for (name, result) in validation_results {
        match result {
            Ok(()) => {
                output::item(Some("✓"), &name, Some("Valid"));
            }
            Err(e) => {
                output::item(Some("✗"), &name, Some("Invalid"));
                errors.push((name, e));
            }
        }
    }

    if !errors.is_empty() {
        output::header("\nValidation Errors:");
        for (name, error) in &errors {
            output::error(&format!("{}: {}", name, error));
        }
    }

    if errors.is_empty() {
        output::success("All templates validated successfully!");
        Ok(())
    } else {
        anyhow::bail!("{} target(s) failed validation", errors.len())
    }
}

fn validate_single_target(
    config_loader: &ConfigLoader,
    targets: &[Target],
    target_name: &str,
) -> Result<()> {
    output::header(&format!("Validating target: {}", target_name));

    let target = targets
        .iter()
        .find(|t| t.name == target_name)
        .with_context(|| format!("Target '{}' not found in configuration", target_name))?;

    let config_dir = config_loader.config_dir();

    match validate_target_template(config_dir, target) {
        Ok(()) => {
            output::success(&format!("Target '{}' is valid!", target_name));
            output::item(Some("Template"), &target.template, None);
            output::item(Some("Mode"), &format!("{:?}", target.mode), None);
            output::item(
                Some("Output"),
                if target.output.is_empty() {
                    "cache"
                } else {
                    &target.output
                },
                None,
            );
            Ok(())
        }
        Err(e) => {
            output::error(&format!("Validation failed: {}", e));
            Err(e)
        }
    }
}

fn validate_target_template(config_dir: &Path, target: &Target) -> Result<()> {
    let template_path = config_dir.join("templates").join(&target.template);

    if !template_path.exists() {
        anyhow::bail!("Template file not found: {}", target.template);
    }

    let template_content = fs::read_to_string(&template_path)
        .with_context(|| format!("Failed to read template file: {}", template_path.display()))?;

    let dummy_palette = create_dummy_palette();

    let mut engine = TemplateEngine::new();
    let context = engine
        .create_context(&dummy_palette)
        .context("Failed to create template context")?;

    engine
        .render(&target.template, &template_content, &context)
        .with_context(|| format!("Template rendering failed for '{}'", target.name))?;

    Ok(())
}

fn create_dummy_palette() -> Palette {
    Palette {
        name: "validation-dummy".to_string(),
        base_16: Some(Base16 {
            base00: "000000".to_string(),
            base01: "111111".to_string(),
            base02: "222222".to_string(),
            base03: "333333".to_string(),
            base04: "444444".to_string(),
            base05: "555555".to_string(),
            base06: "666666".to_string(),
            base07: "777777".to_string(),
            base08: "888888".to_string(),
            base09: "999999".to_string(),
            base0a: "aaaaaa".to_string(),
            base0b: "bbbbbb".to_string(),
            base0c: "cccccc".to_string(),
            base0d: "dddddd".to_string(),
            base0e: "eeeeee".to_string(),
            base0f: "ffffff".to_string(),
        }),
        base_30: Some(Base30 {
            white: "ffffff".to_string(),
            darker_black: "000000".to_string(),
            black: "111111".to_string(),
            black2: "1a1a1a".to_string(),
            one_bg: "222222".to_string(),
            one_bg2: "2a2a2a".to_string(),
            one_bg3: "333333".to_string(),
            grey: "888888".to_string(),
            grey_fg: "999999".to_string(),
            grey_fg2: "aaaaaa".to_string(),
            light_grey: "bbbbbb".to_string(),
            red: "ff0000".to_string(),
            baby_pink: "ffaaaa".to_string(),
            pink: "ff00ff".to_string(),
            line: "444444".to_string(),
            green: "00ff00".to_string(),
            vibrant_green: "00ff88".to_string(),
            nord_blue: "5e81ac".to_string(),
            blue: "0000ff".to_string(),
            yellow: "ffff00".to_string(),
            sun: "ffaa00".to_string(),
            purple: "aa00ff".to_string(),
            dark_purple: "550088".to_string(),
            teal: "00ffff".to_string(),
            orange: "ff8800".to_string(),
            cyan: "00aaff".to_string(),
            lightbg: "eeeeee".to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::models::Mode;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_env() -> (TempDir, ConfigLoader) {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join("themer");
        fs::create_dir_all(config_dir.join("templates")).unwrap();

        let loader = ConfigLoader { config_dir };
        (temp_dir, loader)
    }

    #[test]
    fn test_validate_target_template_not_found() {
        let (_temp, loader) = setup_test_env();
        let target = Target {
            name: "test".to_string(),
            template: "nonexistent.tmpl".to_string(),
            output: String::new(),
            mode: Mode::Include,
            reload_cmd: String::new(),
        };

        let result = validate_target_template(loader.config_dir(), &target);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn test_validate_target_template_valid() {
        let (_temp, loader) = setup_test_env();
        let templates_dir = loader.config_dir().join("templates");

        fs::write(templates_dir.join("test.tmpl"), "color: {{ base00 }}").unwrap();

        let target = Target {
            name: "test".to_string(),
            template: "test.tmpl".to_string(),
            output: String::new(),
            mode: Mode::Include,
            reload_cmd: String::new(),
        };

        let result = validate_target_template(loader.config_dir(), &target);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_target_template_invalid_syntax() {
        let (_temp, loader) = setup_test_env();
        let templates_dir = loader.config_dir().join("templates");

        fs::write(templates_dir.join("invalid.tmpl"), "{{ unclosed_tag").unwrap();

        let target = Target {
            name: "test".to_string(),
            template: "invalid.tmpl".to_string(),
            output: String::new(),
            mode: Mode::Include,
            reload_cmd: String::new(),
        };

        let result = validate_target_template(loader.config_dir(), &target);
        assert!(result.is_err());
    }
}
