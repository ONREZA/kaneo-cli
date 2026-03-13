use crate::auth::AuthConfig;
use crate::cli::{ProfileArgs, ProfileCommand};
use crate::output;

pub fn run(args: ProfileArgs, json: bool) -> anyhow::Result<()> {
    match args.command {
        ProfileCommand::List => {
            let config = AuthConfig::load()?;
            let default = config.default_profile.as_deref().unwrap_or("default");

            if json {
                output::json_output(&config.profiles);
            } else {
                if config.profiles.is_empty() {
                    output::warn(false, "No profiles configured. Run `kaneo login` first");
                    return Ok(());
                }
                let bold = console::Style::new().bold();
                let dim = console::Style::new().dim();
                let green = console::Style::new().green();
                for (name, profile) in &config.profiles {
                    let active = if name == default { " ◀" } else { "" };
                    eprintln!(
                        "  {} {} {}{}",
                        if name == default {
                            green.apply_to(name).to_string()
                        } else {
                            bold.apply_to(name).to_string()
                        },
                        dim.apply_to(&profile.api_url),
                        dim.apply_to(
                            profile
                                .workspace_id
                                .as_deref()
                                .map(|w| format!("ws:{w}"))
                                .unwrap_or_default()
                        ),
                        green.apply_to(active),
                    );
                }
            }
        }

        ProfileCommand::Use { name } => {
            let mut config = AuthConfig::load()?;
            if !config.profiles.contains_key(&name) {
                anyhow::bail!("profile '{name}' does not exist");
            }
            config.default_profile = Some(name.clone());
            config.save()?;
            output::success(json, &format!("Switched to profile '{name}'"));
        }

        ProfileCommand::Remove { name } => {
            let mut config = AuthConfig::load()?;
            if config.profiles.remove(&name).is_none() {
                anyhow::bail!("profile '{name}' does not exist");
            }
            if config.default_profile.as_deref() == Some(&name) {
                config.default_profile = config.profiles.keys().next().cloned();
            }
            config.save()?;
            output::success(json, &format!("Removed profile '{name}'"));
        }

        ProfileCommand::Current => {
            let config = AuthConfig::load()?;
            let name = config.default_profile.as_deref().unwrap_or("default");
            match config.profiles.get(name) {
                Some(profile) => {
                    if json {
                        output::json_output(&serde_json::json!({
                            "name": name,
                            "api_url": profile.api_url,
                            "workspace_id": profile.workspace_id,
                        }));
                    } else {
                        let dim = console::Style::new().dim();
                        let bold = console::Style::new().bold();
                        eprintln!("  {} {}", dim.apply_to("profile:"), bold.apply_to(name));
                        eprintln!("  {} {}", dim.apply_to("url:"), profile.api_url);
                        eprintln!(
                            "  {} {}…",
                            dim.apply_to("key:"),
                            &profile.api_key[..8.min(profile.api_key.len())]
                        );
                        if let Some(ws) = &profile.workspace_id {
                            eprintln!("  {} {ws}", dim.apply_to("workspace:"));
                        }
                    }
                }
                None => {
                    anyhow::bail!("no profile configured. Run `kaneo login` first");
                }
            }
        }

        ProfileCommand::SetWorkspace {
            workspace_id,
            profile,
        } => {
            let mut config = AuthConfig::load()?;
            let name = profile.unwrap_or_else(|| {
                config
                    .default_profile
                    .clone()
                    .unwrap_or_else(|| "default".to_string())
            });
            let p = config
                .profiles
                .get_mut(&name)
                .ok_or_else(|| anyhow::anyhow!("profile '{name}' does not exist"))?;
            p.workspace_id = Some(workspace_id.clone());
            config.save()?;
            output::success(
                json,
                &format!("Set workspace '{workspace_id}' on profile '{name}'"),
            );
        }
    }

    Ok(())
}
