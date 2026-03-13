use crate::auth::{AuthConfig, Profile};
use crate::cli::LoginArgs;
use crate::output;

pub fn run(args: LoginArgs, json: bool) -> anyhow::Result<()> {
    let url = match args.url {
        Some(u) => u,
        None => output::prompt_input("Kaneo API URL (e.g. https://cloud.kaneo.app)")?,
    };

    let key = match args.key {
        Some(k) => k,
        None => output::prompt_input("API key")?,
    };

    if url.is_empty() || key.is_empty() {
        anyhow::bail!("API URL and key are required");
    }

    let mut config = AuthConfig::load()?;
    config.profiles.insert(
        args.profile.clone(),
        Profile {
            api_url: url.trim_end_matches('/').to_string(),
            api_key: key,
            workspace_id: args.workspace,
            project_id: None,
        },
    );

    if config.default_profile.is_none() {
        config.default_profile = Some(args.profile.clone());
    }

    config.save()?;

    output::success(json, &format!("Profile '{}' saved", args.profile));
    Ok(())
}

pub fn logout(json: bool) -> anyhow::Result<()> {
    let path = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("cannot determine config directory"))?
        .join("kaneo")
        .join("config.json");

    if path.exists() {
        std::fs::remove_file(&path)?;
        output::success(json, "Credentials removed");
    } else {
        output::warn(json, "No credentials found");
    }
    Ok(())
}
