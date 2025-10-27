use anyhow::{Context as AnyhowContext, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use tera::Context;

use crate::config::models::{Mode, Target};
use crate::output::output;
use crate::palette::models::Palette;
use crate::template::engine::TemplateEngine;

pub struct TargetProcessor {
    templates_dir: PathBuf,
    engine: TemplateEngine,
}

impl TargetProcessor {
    pub fn new(config_dir: &Path) -> Self {
        Self {
            templates_dir: config_dir.join("templates"),
            engine: TemplateEngine::new(),
        }
    }

    pub fn process(&mut self, target: &Target, context: &Context, palette: &Palette) -> Result<()> {
        let template_path = self.templates_dir.join(&target.template);
        let template_content = fs::read_to_string(&template_path)
            .with_context(|| format!("Failed to read template: {}", template_path.display()))?;

        let rendered = self
            .engine
            .render(&target.template, &template_content, context)
            .with_context(|| format!("Failed to render template for {}", target.name))?;

        let output_path = self.resolve_output_path(target)?;

        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::write(&output_path, rendered)
            .with_context(|| format!("Failed to write file: {}", output_path.display()))?;

        output::item(Some("→"), &target.name, None);

        if !target.reload_cmd.is_empty() {
            self.handle_reload_command(&target.reload_cmd, &target.name, &palette.name)?;
        }

        Ok(())
    }

    fn resolve_output_path(&self, target: &Target) -> Result<PathBuf> {
        match target.mode {
            Mode::Include => {
                let cache_dir = dirs::cache_dir()
                    .context("Could not find cache directory")?
                    .join("themer");

                let extension = Path::new(&target.template)
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("");

                let filename = if extension.is_empty() {
                    target.name.clone()
                } else {
                    format!("{}.{}", target.name, extension)
                };

                Ok(cache_dir.join(filename))
            }
            Mode::Replace => {
                if target.output.is_empty() {
                    anyhow::bail!(
                        "Target '{}' with mode 'Replace' requires an 'output' field",
                        target.name
                    );
                }

                Ok(PathBuf::from(
                    shellexpand::tilde(&target.output).into_owned(),
                ))
            }
        }
    }

    fn handle_reload_command(
        &self,
        reload_cmd: &str,
        target_name: &str,
        theme_name: &str,
    ) -> Result<()> {
        let command = reload_cmd.replace("{theme}", theme_name);

        if command.trim().ends_with('&') {
            output::info(&format!("Spawning background command for {}", target_name));
            self.execute_background_command(&command)?;
            output::success("Background command spawned");
        } else {
            output::info(&format!("Executing reload command for {}", target_name));
            self.execute_foreground_command(&command)?;
            output::success("Application reloaded");
        }

        Ok(())
    }

    fn execute_foreground_command(&self, command: &str) -> Result<()> {
        if command.is_empty() {
            return Ok(());
        }

        let output = Command::new("sh")
            .args(["-c", command])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output();

        match output {
            Ok(output) if output.status.success() => Ok(()),
            Ok(output) => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                output::warning(&format!("Warning executing command: {}", stderr));
                Ok(())
            }
            Err(e) => {
                output::error(&format!("Could not execute command: {}", e));
                Ok(())
            }
        }
    }

    fn execute_background_command(&self, command: &str) -> Result<()> {
        if command.is_empty() {
            return Ok(());
        }

        Command::new("sh")
            .args(["-c", command])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|| format!("Failed to spawn background command: {}", command))?;

        std::thread::sleep(Duration::from_millis(100));

        Ok(())
    }

    pub fn cache_wallpaper(&self, wallpaper_path: &Path) -> Result<()> {
        let cache_dir = dirs::cache_dir()
            .context("Could not find cache directory")?
            .join("themer");

        fs::create_dir_all(&cache_dir).with_context(|| {
            format!("Failed to create cache directory: {}", cache_dir.display())
        })?;

        let wallpaper_dest = cache_dir.join("wallpaper");

        fs::copy(wallpaper_path, &wallpaper_dest).with_context(|| {
            format!(
                "Failed to copy wallpaper to cache: {}",
                wallpaper_dest.display()
            )
        })?;

        output::item(
            Some("→"),
            "Cached wallpaper",
            Some(&wallpaper_dest.display().to_string()),
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::models::{Mode, Target};
    use std::env;

    #[test]
    fn test_resolve_output_path_include_mode() {
        let temp_dir = env::temp_dir();
        let processor = TargetProcessor::new(&temp_dir);

        let target = Target {
            name: "test".to_string(),
            template: "test.conf".to_string(),
            output: String::new(),
            mode: Mode::Include,
            reload_cmd: String::new(),
        };

        let result = processor.resolve_output_path(&target);
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("test.conf"));
    }

    #[test]
    fn test_resolve_output_path_replace_mode_empty_output() {
        let temp_dir = env::temp_dir();
        let processor = TargetProcessor::new(&temp_dir);

        let target = Target {
            name: "test".to_string(),
            template: "test.conf".to_string(),
            output: String::new(),
            mode: Mode::Replace,
            reload_cmd: String::new(),
        };

        let result = processor.resolve_output_path(&target);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("requires an 'output' field")
        );
    }

    #[test]
    fn test_handle_reload_command_with_theme_placeholder() {
        let temp_dir = env::temp_dir();
        let processor = TargetProcessor::new(&temp_dir);

        let result = processor.handle_reload_command("echo {theme}", "test", "my-theme");

        assert!(result.is_ok());
    }
}
