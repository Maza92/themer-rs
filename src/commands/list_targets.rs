use anyhow::Result;

use crate::config::loader::ConfigLoader;
use crate::output::output;

pub fn execute(format: Option<&str>) -> Result<()> {
    let config_loader = ConfigLoader::new()?;
    let config = config_loader.load()?;

    match format {
        Some("plain") => output_plain(&config.targets),
        Some("json") => output_json(&config.targets)?,
        Some(unknown) => {
            output::warning(&format!("Unknown format '{}', using default", unknown));
            output_default(&config.targets)
        }
        None => output_default(&config.targets),
    }

    Ok(())
}

fn output_plain(targets: &[crate::config::models::Target]) {
    for target in targets {
        println!("{}", target.name);
    }
}

fn output_json(targets: &[crate::config::models::Target]) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(targets)?);
    Ok(())
}

fn output_default(targets: &[crate::config::models::Target]) {
    output::header("Configured targets:");

    if targets.is_empty() {
        output::warning("No targets configured");
        return;
    }

    for target in targets {
        output::item(Some("Target"), &target.name, Some(&target.template));

        let mode_str = format!("Mode: {:?}", target.mode);
        output::info(&mode_str);

        if !target.output.is_empty() {
            let output_str = format!("Output: {}", target.output);
            output::info(&output_str);
        }

        if !target.reload_cmd.is_empty() {
            let reload_str = format!("Reload: {}", target.reload_cmd);
            output::info(&reload_str);
        }

        println!();
    }
}
