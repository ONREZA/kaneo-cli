use anyhow::{Context, bail};
use clap::Parser;
use serde::Deserialize;
use std::io::Read;
use std::path::PathBuf;

const REPO: &str = "onreza/kaneo-cli";
const GITHUB_API: &str = "https://api.github.com/repos";
const CHECK_INTERVAL_SECS: u64 = 86400; // 24 hours

#[derive(Parser)]
pub struct UpgradeArgs {
    /// Force upgrade even if already on latest version
    #[arg(long)]
    pub force: bool,

    /// Specific version to upgrade to (e.g., v0.1.0)
    #[arg(long)]
    pub version: Option<String>,
}

#[derive(Deserialize)]
struct ReleaseInfo {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    #[serde(rename = "browser_download_url")]
    download_url: String,
}

// ---------------------------------------------------------------------------
// Self-upgrade command
// ---------------------------------------------------------------------------

pub async fn run(args: UpgradeArgs) -> anyhow::Result<()> {
    cleanup_old_files();

    let current = env!("CARGO_PKG_VERSION");
    let platform = detect_platform()?;

    eprintln!("  Current version: v{current}");
    eprintln!("  Platform: {platform}");

    let release = if let Some(ref tag) = args.version {
        let tag = if tag.starts_with('v') {
            tag.clone()
        } else {
            format!("v{tag}")
        };
        eprintln!("  Fetching release {tag}...");
        fetch_release(&tag).await?
    } else {
        eprintln!("  Checking for updates...");
        fetch_latest_release().await?
    };

    let latest = release.tag_name.trim_start_matches('v');

    if latest == current && !args.force {
        eprintln!("  Already on the latest version (v{current})");
        return Ok(());
    }

    let archive_name = format!("kaneo-{platform}.tar.gz");
    let asset = release
        .assets
        .iter()
        .find(|a| a.name == archive_name)
        .ok_or_else(|| {
            anyhow::anyhow!("no asset '{archive_name}' in release {}", release.tag_name)
        })?;

    eprintln!("  Downloading v{latest}...");
    let data = download_binary(&asset.download_url).await?;

    eprintln!("  Extracting...");
    let binary = extract_binary_from_tar_gz(&data)?;

    eprintln!("  Replacing binary...");
    replace_binary(&binary)?;

    // Update version cache so we don't nag after upgrade
    let _ = write_version_cache(latest);

    let green = console::Style::new().green().bold();
    eprintln!("\n  {} Upgraded to v{latest}", green.apply_to("✓"));

    Ok(())
}

// ---------------------------------------------------------------------------
// Version check notification (non-blocking)
// ---------------------------------------------------------------------------

/// Lightweight check that only reads cache (never does network).
/// Use this for instant startup checks.
pub fn check_cached_update() -> Option<String> {
    let cache = read_version_cache().ok()??;
    let current = env!("CARGO_PKG_VERSION");
    let latest = cache.latest_version.trim_start_matches('v');

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .ok()?
        .as_secs();

    // Only show if cache is reasonably fresh (within 48h)
    if now.saturating_sub(cache.checked_at) < CHECK_INTERVAL_SECS * 2 && latest != current {
        Some(latest.to_string())
    } else {
        None
    }
}

/// Spawn a background task that fetches the latest version and caches it.
/// Does not block. Call this early in main so the cache is warm for next run.
pub fn spawn_version_check() {
    tokio::spawn(async {
        if let Ok(release) = fetch_latest_release().await {
            let latest = release.tag_name.trim_start_matches('v');
            let _ = write_version_cache(latest);
        }
    });
}

/// Returns true if we should do a background version check (cache is stale).
pub fn should_check_version() -> bool {
    let Ok(Some(cache)) = read_version_cache() else {
        return true;
    };
    let Ok(now) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) else {
        return false;
    };
    now.as_secs().saturating_sub(cache.checked_at) >= CHECK_INTERVAL_SECS
}

// ---------------------------------------------------------------------------
// Version cache (JSON file next to config)
// ---------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize)]
struct VersionCache {
    latest_version: String,
    checked_at: u64,
}

fn cache_path() -> anyhow::Result<PathBuf> {
    let dir = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("no config dir"))?;
    Ok(dir.join("kaneo").join("version-cache.json"))
}

fn read_version_cache() -> anyhow::Result<Option<VersionCache>> {
    let path = cache_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let data = std::fs::read_to_string(&path)?;
    Ok(Some(serde_json::from_str(&data)?))
}

fn write_version_cache(version: &str) -> anyhow::Result<()> {
    let path = cache_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let cache = VersionCache {
        latest_version: version.to_string(),
        checked_at: now,
    };
    std::fs::write(&path, serde_json::to_string(&cache)?)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// GitHub API
// ---------------------------------------------------------------------------

async fn fetch_latest_release() -> anyhow::Result<ReleaseInfo> {
    let url = format!("{GITHUB_API}/{REPO}/releases/latest");
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("user-agent", "kaneo-cli")
        .send()
        .await
        .context("fetching latest release")?;

    if !resp.status().is_success() {
        bail!("GitHub API: {}", resp.status());
    }

    resp.json().await.context("parsing release info")
}

async fn fetch_release(tag: &str) -> anyhow::Result<ReleaseInfo> {
    let url = format!("{GITHUB_API}/{REPO}/releases/tags/{tag}");
    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("user-agent", "kaneo-cli")
        .send()
        .await
        .context("fetching release")?;

    if !resp.status().is_success() {
        bail!("GitHub API: {} (tag {tag})", resp.status());
    }

    resp.json().await.context("parsing release info")
}

async fn download_binary(url: &str) -> anyhow::Result<Vec<u8>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("user-agent", "kaneo-cli")
        .send()
        .await
        .context("downloading binary")?;

    if !resp.status().is_success() {
        bail!("download failed: {}", resp.status());
    }

    let bytes = resp.bytes().await.context("reading binary")?;
    Ok(bytes.to_vec())
}

// ---------------------------------------------------------------------------
// Archive & binary replacement
// ---------------------------------------------------------------------------

fn detect_platform() -> anyhow::Result<&'static str> {
    let platform = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "linux-x64",
        ("macos", "x86_64") => "darwin-x64",
        ("macos", "aarch64") => "darwin-arm64",
        ("windows", "x86_64") => "win32-x64",
        (os, arch) => bail!("unsupported platform: {os}/{arch}"),
    };
    Ok(platform)
}

fn extract_binary_from_tar_gz(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    let gz = flate2::read::GzDecoder::new(data);
    let mut archive = tar::Archive::new(gz);

    let bin_name = if cfg!(windows) { "kaneo.exe" } else { "kaneo" };

    for entry in archive.entries().context("reading tar archive")? {
        let mut entry = entry.context("reading tar entry")?;
        let path = entry.path().context("reading entry path")?;

        if path.file_name().and_then(|n| n.to_str()) == Some(bin_name) {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf).context("extracting binary")?;
            return Ok(buf);
        }
    }

    bail!("binary '{bin_name}' not found in archive")
}

fn replace_binary(new_binary: &[u8]) -> anyhow::Result<()> {
    let current_exe = std::env::current_exe().context("getting current executable path")?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let tmp_path = current_exe.with_extension("tmp");
        std::fs::write(&tmp_path, new_binary)
            .with_context(|| format!("writing {}", tmp_path.display()))?;
        std::fs::set_permissions(&tmp_path, std::fs::Permissions::from_mode(0o755))?;
        std::fs::rename(&tmp_path, &current_exe)
            .with_context(|| format!("replacing {}", current_exe.display()))?;
    }

    #[cfg(windows)]
    {
        let new_path = current_exe.with_extension("new.exe");
        let old_path = current_exe.with_extension("old.exe");

        std::fs::write(&new_path, new_binary)
            .with_context(|| format!("writing {}", new_path.display()))?;

        // Rename current → .old, then new → current
        if old_path.exists() {
            let _ = std::fs::remove_file(&old_path);
        }
        std::fs::rename(&current_exe, &old_path)
            .with_context(|| format!("renaming {} → .old", current_exe.display()))?;
        std::fs::rename(&new_path, &current_exe)
            .with_context(|| format!("renaming .new → {}", current_exe.display()))?;

        // Try to clean up, but don't fail if the file is still locked
        let _ = std::fs::remove_file(&old_path);

        eprintln!("  Note: restart your terminal for the new version to take effect");
    }

    Ok(())
}

fn cleanup_old_files() {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };

    #[cfg(windows)]
    {
        let _ = std::fs::remove_file(exe.with_extension("old.exe"));
        let _ = std::fs::remove_file(exe.with_extension("new.exe"));
    }

    #[cfg(unix)]
    {
        let _ = std::fs::remove_file(exe.with_extension("tmp"));
    }
}
