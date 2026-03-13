use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Global auth config (~/.config/kaneo/config.json)
// ---------------------------------------------------------------------------

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
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
        std::fs::write(&path, data).with_context(|| format!("writing {}", path.display()))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    pub fn active_profile(&self) -> anyhow::Result<&Profile> {
        let name = self.default_profile.as_deref().unwrap_or("default");

        self.profiles
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("no profile '{name}' found. Run `kaneo login` first"))
    }
}

// ---------------------------------------------------------------------------
// Local project config (.kaneo.json — walk up from cwd to $HOME)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

const LOCAL_CONFIG_NAME: &str = ".kaneo.json";

/// Walk up from `start` to `stop` (inclusive), collecting all .kaneo.json files.
/// Returns them in priority order: closest (cwd) first, farthest (home) last.
fn find_local_configs(start: &Path, stop: &Path) -> Vec<LocalConfig> {
    let mut configs = Vec::new();
    let mut dir = start.to_path_buf();

    loop {
        let candidate = dir.join(LOCAL_CONFIG_NAME);
        if candidate.is_file()
            && let Ok(data) = std::fs::read_to_string(&candidate)
            && let Ok(cfg) = serde_json::from_str::<LocalConfig>(&data)
        {
            configs.push(cfg);
        }

        if dir == stop {
            break;
        }
        if !dir.pop() {
            break;
        }
    }

    configs
}

/// Merge local configs: first non-None value wins (closest to cwd).
fn merge_local_configs(configs: &[LocalConfig]) -> LocalConfig {
    let mut merged = LocalConfig::default();
    for cfg in configs {
        if merged.workspace.is_none() {
            merged.workspace.clone_from(&cfg.workspace);
        }
        if merged.project.is_none() {
            merged.project.clone_from(&cfg.project);
        }
        if merged.workspace.is_some() && merged.project.is_some() {
            break;
        }
    }
    merged
}

/// Load merged local config from cwd walk-up.
pub fn load_local_config() -> LocalConfig {
    let cwd = std::env::current_dir().unwrap_or_default();
    let home = dirs::home_dir().unwrap_or_default();
    let configs = find_local_configs(&cwd, &home);
    merge_local_configs(&configs)
}

/// Write a .kaneo.json in the given directory.
pub fn write_local_config(dir: &Path, config: &LocalConfig) -> anyhow::Result<()> {
    let path = dir.join(LOCAL_CONFIG_NAME);
    let data = serde_json::to_string_pretty(config)?;
    std::fs::write(&path, data).with_context(|| format!("writing {}", path.display()))
}

// ---------------------------------------------------------------------------
// Resolved context (all sources merged)
// ---------------------------------------------------------------------------

pub struct ResolvedContext {
    pub api_url: String,
    pub api_key: String,
    pub workspace_id: Option<String>,
    pub project_id: Option<String>,
}

/// Resolution priority (highest → lowest):
/// 1. CLI flags (--token, --workspace, --project)
/// 2. Environment variables (KANEO_API_KEY, KANEO_WORKSPACE, KANEO_PROJECT)
/// 3. .kaneo.json walk-up (cwd → $HOME)
/// 4. Global profile (~/.config/kaneo/config.json)
pub fn resolve_context(
    token: Option<&str>,
    api_url: Option<&str>,
    workspace: Option<&str>,
    project: Option<&str>,
) -> anyhow::Result<ResolvedContext> {
    let env_api_url = std::env::var("KANEO_API_URL").ok();
    let default_url = "https://cloud.kaneo.app";
    let local = load_local_config();

    // Helper: resolve workspace from remaining sources
    let resolve_ws = |flag: Option<&str>| -> Option<String> {
        flag.map(String::from)
            .or_else(|| std::env::var("KANEO_WORKSPACE").ok())
            .or(local.workspace.clone())
    };

    // Helper: resolve project from remaining sources
    let resolve_proj = |flag: Option<&str>| -> Option<String> {
        flag.map(String::from)
            .or_else(|| std::env::var("KANEO_PROJECT").ok())
            .or(local.project.clone())
    };

    // Explicit token flag
    if let Some(key) = token {
        let url = api_url.or(env_api_url.as_deref()).unwrap_or(default_url);
        return Ok(ResolvedContext {
            api_url: url.to_string(),
            api_key: key.to_string(),
            workspace_id: resolve_ws(workspace),
            project_id: resolve_proj(project),
        });
    }

    // Env var token
    if let Ok(key) = std::env::var("KANEO_API_KEY") {
        let url = api_url.or(env_api_url.as_deref()).unwrap_or(default_url);
        return Ok(ResolvedContext {
            api_url: url.to_string(),
            api_key: key,
            workspace_id: resolve_ws(workspace),
            project_id: resolve_proj(project),
        });
    }

    // Config file profile
    let config = AuthConfig::load()?;
    let profile = config.active_profile()?;
    Ok(ResolvedContext {
        api_url: api_url.unwrap_or(&profile.api_url).to_string(),
        api_key: profile.api_key.clone(),
        workspace_id: workspace
            .map(String::from)
            .or_else(|| std::env::var("KANEO_WORKSPACE").ok())
            .or(local.workspace)
            .or_else(|| profile.workspace_id.clone()),
        project_id: project
            .map(String::from)
            .or_else(|| std::env::var("KANEO_PROJECT").ok())
            .or(local.project)
            .or_else(|| profile.project_id.clone()),
    })
}

pub fn require_workspace(ctx: &ResolvedContext) -> anyhow::Result<&str> {
    ctx.workspace_id.as_deref().ok_or_else(|| {
        anyhow::anyhow!("workspace ID required. Use -w, KANEO_WORKSPACE, .kaneo.json, or profile")
    })
}

pub fn require_project(ctx: &ResolvedContext) -> anyhow::Result<&str> {
    ctx.project_id.as_deref().ok_or_else(|| {
        anyhow::anyhow!("project ID required. Use -p, KANEO_PROJECT, .kaneo.json, or `kaneo link`")
    })
}

fn config_path() -> anyhow::Result<PathBuf> {
    let dir =
        dirs::config_dir().ok_or_else(|| anyhow::anyhow!("cannot determine config directory"))?;
    Ok(dir.join("kaneo").join("config.json"))
}
