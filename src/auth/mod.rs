use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    #[serde(default)]
    pub profiles: BTreeMap<String, Profile>,
    pub default_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub api_url: String,
    pub api_key: String,
    pub workspace_id: Option<String>,
}

impl AuthConfig {
    pub fn load() -> anyhow::Result<Self> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(&path)
            .with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&data).with_context(|| format!("parsing {}", path.display()))
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, data)
            .with_context(|| format!("writing {}", path.display()))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    pub fn active_profile(&self) -> anyhow::Result<&Profile> {
        let name = self
            .default_profile
            .as_deref()
            .unwrap_or("default");

        self.profiles
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("no profile '{name}' found. Run `kaneo login` first"))
    }
}

pub struct ResolvedContext {
    pub api_url: String,
    pub api_key: String,
    pub workspace_id: Option<String>,
}

pub fn resolve_context(
    token: Option<&str>,
    api_url: Option<&str>,
    workspace: Option<&str>,
) -> anyhow::Result<ResolvedContext> {
    let env_api_url = std::env::var("KANEO_API_URL").ok();
    let default_url = "https://cloud.kaneo.app";

    // Explicit flags take priority
    if let Some(key) = token {
        let url = api_url
            .or(env_api_url.as_deref())
            .unwrap_or(default_url);
        return Ok(ResolvedContext {
            api_url: url.to_string(),
            api_key: key.to_string(),
            workspace_id: workspace.map(String::from),
        });
    }

    // Env vars
    if let Ok(key) = std::env::var("KANEO_API_KEY") {
        let url = api_url
            .or(env_api_url.as_deref())
            .unwrap_or(default_url);
        return Ok(ResolvedContext {
            api_url: url.to_string(),
            api_key: key,
            workspace_id: workspace
                .map(String::from)
                .or_else(|| std::env::var("KANEO_WORKSPACE").ok()),
        });
    }

    // Config file
    let config = AuthConfig::load()?;
    let profile = config.active_profile()?;
    Ok(ResolvedContext {
        api_url: api_url.unwrap_or(&profile.api_url).to_string(),
        api_key: profile.api_key.clone(),
        workspace_id: workspace
            .map(String::from)
            .or_else(|| profile.workspace_id.clone()),
    })
}

pub fn require_workspace(ctx: &ResolvedContext) -> anyhow::Result<&str> {
    ctx.workspace_id.as_deref().ok_or_else(|| {
        anyhow::anyhow!(
            "workspace ID required. Use --workspace, KANEO_WORKSPACE env, or set it in your profile"
        )
    })
}

fn config_path() -> anyhow::Result<PathBuf> {
    let dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("cannot determine config directory"))?;
    Ok(dir.join("kaneo").join("config.json"))
}
