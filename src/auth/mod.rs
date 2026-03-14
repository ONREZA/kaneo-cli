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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // --- find_local_configs ---

    #[test]
    fn find_configs_single_file() {
        let tmp = TempDir::new().unwrap();
        let cfg = LocalConfig {
            workspace: Some("ws-1".into()),
            project: Some("proj-1".into()),
        };
        fs::write(
            tmp.path().join(".kaneo.json"),
            serde_json::to_string(&cfg).unwrap(),
        )
        .unwrap();

        let configs = find_local_configs(tmp.path(), tmp.path());
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].workspace.as_deref(), Some("ws-1"));
        assert_eq!(configs[0].project.as_deref(), Some("proj-1"));
    }

    #[test]
    fn find_configs_nested_walk_up() {
        let tmp = TempDir::new().unwrap();
        let child = tmp.path().join("sub");
        fs::create_dir(&child).unwrap();

        fs::write(
            tmp.path().join(".kaneo.json"),
            r#"{"workspace":"ws-parent"}"#,
        )
        .unwrap();
        fs::write(child.join(".kaneo.json"), r#"{"project":"proj-child"}"#).unwrap();

        let configs = find_local_configs(&child, tmp.path());
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].project.as_deref(), Some("proj-child"));
        assert!(configs[0].workspace.is_none());
        assert_eq!(configs[1].workspace.as_deref(), Some("ws-parent"));
    }

    #[test]
    fn find_configs_no_files() {
        let tmp = TempDir::new().unwrap();
        let configs = find_local_configs(tmp.path(), tmp.path());
        assert!(configs.is_empty());
    }

    #[test]
    fn find_configs_invalid_json_skipped() {
        let tmp = TempDir::new().unwrap();
        fs::write(tmp.path().join(".kaneo.json"), "not json").unwrap();
        let configs = find_local_configs(tmp.path(), tmp.path());
        assert!(configs.is_empty());
    }

    #[test]
    fn find_configs_deeply_nested() {
        let tmp = TempDir::new().unwrap();
        let a = tmp.path().join("a");
        let b = a.join("b");
        let c = b.join("c");
        fs::create_dir_all(&c).unwrap();

        fs::write(tmp.path().join(".kaneo.json"), r#"{"workspace":"ws-root"}"#).unwrap();
        fs::write(b.join(".kaneo.json"), r#"{"project":"proj-b"}"#).unwrap();
        // a and c have no config

        let configs = find_local_configs(&c, tmp.path());
        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].project.as_deref(), Some("proj-b"));
        assert_eq!(configs[1].workspace.as_deref(), Some("ws-root"));
    }

    // --- merge_local_configs ---

    #[test]
    fn merge_closest_wins() {
        let configs = vec![
            LocalConfig {
                workspace: Some("ws-close".into()),
                project: Some("proj-close".into()),
            },
            LocalConfig {
                workspace: Some("ws-far".into()),
                project: Some("proj-far".into()),
            },
        ];
        let merged = merge_local_configs(&configs);
        assert_eq!(merged.workspace.as_deref(), Some("ws-close"));
        assert_eq!(merged.project.as_deref(), Some("proj-close"));
    }

    #[test]
    fn merge_fills_gaps_from_parent() {
        let configs = vec![
            LocalConfig {
                workspace: None,
                project: Some("proj-child".into()),
            },
            LocalConfig {
                workspace: Some("ws-parent".into()),
                project: None,
            },
        ];
        let merged = merge_local_configs(&configs);
        assert_eq!(merged.workspace.as_deref(), Some("ws-parent"));
        assert_eq!(merged.project.as_deref(), Some("proj-child"));
    }

    #[test]
    fn merge_empty_list() {
        let merged = merge_local_configs(&[]);
        assert!(merged.workspace.is_none());
        assert!(merged.project.is_none());
    }

    #[test]
    fn merge_all_none() {
        let configs = vec![
            LocalConfig {
                workspace: None,
                project: None,
            },
            LocalConfig {
                workspace: None,
                project: None,
            },
        ];
        let merged = merge_local_configs(&configs);
        assert!(merged.workspace.is_none());
        assert!(merged.project.is_none());
    }

    // --- LocalConfig serde ---

    #[test]
    fn local_config_partial_deserialize() {
        let cfg: LocalConfig = serde_json::from_str(r#"{"workspace":"ws-1"}"#).unwrap();
        assert_eq!(cfg.workspace.as_deref(), Some("ws-1"));
        assert!(cfg.project.is_none());
    }

    #[test]
    fn local_config_empty_deserialize() {
        let cfg: LocalConfig = serde_json::from_str("{}").unwrap();
        assert!(cfg.workspace.is_none());
        assert!(cfg.project.is_none());
    }

    #[test]
    fn local_config_skip_none_on_serialize() {
        let cfg = LocalConfig {
            workspace: Some("ws".into()),
            project: None,
        };
        let json = serde_json::to_string(&cfg).unwrap();
        assert!(json.contains("workspace"));
        assert!(!json.contains("project"));
    }

    // --- AuthConfig ---

    #[test]
    fn auth_config_default_is_empty() {
        let cfg = AuthConfig::default();
        assert!(cfg.profiles.is_empty());
        assert!(cfg.default_profile.is_none());
    }

    #[test]
    fn auth_config_active_profile_default_name() {
        let mut cfg = AuthConfig::default();
        cfg.profiles.insert(
            "default".into(),
            Profile {
                api_url: "https://example.com".into(),
                api_key: "key".into(),
                workspace_id: None,
                project_id: None,
            },
        );
        let profile = cfg.active_profile().unwrap();
        assert_eq!(profile.api_url, "https://example.com");
    }

    #[test]
    fn auth_config_active_profile_missing() {
        let cfg = AuthConfig::default();
        let err = cfg.active_profile().unwrap_err();
        assert!(err.to_string().contains("no profile"));
    }

    #[test]
    fn auth_config_active_profile_named() {
        let mut cfg = AuthConfig::default();
        cfg.default_profile = Some("work".into());
        cfg.profiles.insert(
            "work".into(),
            Profile {
                api_url: "https://work.example.com".into(),
                api_key: "work-key".into(),
                workspace_id: Some("ws-work".into()),
                project_id: None,
            },
        );
        let profile = cfg.active_profile().unwrap();
        assert_eq!(profile.api_key, "work-key");
        assert_eq!(profile.workspace_id.as_deref(), Some("ws-work"));
    }

    #[test]
    fn auth_config_roundtrip_serde() {
        let mut cfg = AuthConfig::default();
        cfg.default_profile = Some("test".into());
        cfg.profiles.insert(
            "test".into(),
            Profile {
                api_url: "https://test.com".into(),
                api_key: "secret".into(),
                workspace_id: Some("ws".into()),
                project_id: Some("proj".into()),
            },
        );
        let json = serde_json::to_string(&cfg).unwrap();
        let restored: AuthConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.default_profile.as_deref(), Some("test"));
        let p = restored.profiles.get("test").unwrap();
        assert_eq!(p.api_key, "secret");
        assert_eq!(p.project_id.as_deref(), Some("proj"));
    }

    // --- require_workspace / require_project ---

    #[test]
    fn require_workspace_present() {
        let ctx = ResolvedContext {
            api_url: String::new(),
            api_key: String::new(),
            workspace_id: Some("ws".into()),
            project_id: None,
        };
        assert_eq!(require_workspace(&ctx).unwrap(), "ws");
    }

    #[test]
    fn require_workspace_missing() {
        let ctx = ResolvedContext {
            api_url: String::new(),
            api_key: String::new(),
            workspace_id: None,
            project_id: None,
        };
        assert!(require_workspace(&ctx).is_err());
    }

    #[test]
    fn require_project_present() {
        let ctx = ResolvedContext {
            api_url: String::new(),
            api_key: String::new(),
            workspace_id: None,
            project_id: Some("proj".into()),
        };
        assert_eq!(require_project(&ctx).unwrap(), "proj");
    }

    #[test]
    fn require_project_missing() {
        let ctx = ResolvedContext {
            api_url: String::new(),
            api_key: String::new(),
            workspace_id: None,
            project_id: None,
        };
        assert!(require_project(&ctx).is_err());
    }

    // --- write_local_config + round-trip ---

    #[test]
    fn write_and_read_local_config() {
        let tmp = TempDir::new().unwrap();
        let cfg = LocalConfig {
            workspace: Some("ws-rt".into()),
            project: Some("proj-rt".into()),
        };
        write_local_config(tmp.path(), &cfg).unwrap();

        let configs = find_local_configs(tmp.path(), tmp.path());
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].workspace.as_deref(), Some("ws-rt"));
        assert_eq!(configs[0].project.as_deref(), Some("proj-rt"));
    }

    // --- resolve_context (with explicit token) ---

    #[test]
    fn resolve_context_explicit_token() {
        let ctx = resolve_context(
            Some("my-key"),
            Some("https://my.api"),
            Some("ws-flag"),
            Some("proj-flag"),
        )
        .unwrap();

        assert_eq!(ctx.api_key, "my-key");
        assert_eq!(ctx.api_url, "https://my.api");
        assert_eq!(ctx.workspace_id.as_deref(), Some("ws-flag"));
        assert_eq!(ctx.project_id.as_deref(), Some("proj-flag"));
    }

    #[test]
    fn resolve_context_token_defaults_to_cloud() {
        let ctx = resolve_context(Some("key"), None, None, None).unwrap();
        assert_eq!(ctx.api_url, "https://cloud.kaneo.app");
    }
}
