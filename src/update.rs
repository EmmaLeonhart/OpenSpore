//! Self-update mechanism for the Clawling CLI.
//!
//! Checks GitHub releases for newer versions and offers to download + replace
//! the running binary. Respects a config file at `~/.clawling/config.toml`
//! where users can set `auto_update_check = false` to opt out.

use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};

const GITHUB_RELEASES_API: &str =
    "https://api.github.com/repos/EmmaLeonhart/OpenSpore/releases/latest";

/// The current version from Cargo.toml, baked in at compile time.
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Information about an available update.
pub struct UpdateInfo {
    pub current: Version,
    pub latest: Version,
    pub asset_url: String,
    pub asset_name: String,
}

/// GitHub release API response (only the fields we need).
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
}

/// User config stored at `~/.clawling/config.toml`.
#[derive(Debug, Deserialize, Default)]
struct ClawlingConfig {
    #[serde(default = "default_true")]
    auto_update_check: bool,
}

fn default_true() -> bool {
    true
}

/// Load the config file, returning defaults if it doesn't exist or can't be parsed.
fn load_config() -> ClawlingConfig {
    let config_path = config_path();
    match config_path {
        Some(path) if path.exists() => {
            if let Ok(contents) = fs::read_to_string(&path) {
                toml::from_str(&contents).unwrap_or_default()
            } else {
                ClawlingConfig::default()
            }
        }
        _ => ClawlingConfig::default(),
    }
}

/// Return the path to `~/.clawling/config.toml`.
fn config_path() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join(".clawling").join("config.toml"))
}

/// Determine the expected asset name for the current platform.
fn platform_asset_name() -> Result<String> {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        anyhow::bail!("Unsupported OS for self-update");
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        anyhow::bail!("Unsupported architecture for self-update");
    };

    let ext = if cfg!(target_os = "windows") {
        "zip"
    } else {
        "tar.gz"
    };

    Ok(format!("clawling-{os}-{arch}.{ext}"))
}

/// Parse a version from a release tag like "v0.2.0" or "0.2.0".
fn parse_tag_version(tag: &str) -> Result<Version> {
    let cleaned = tag.strip_prefix('v').unwrap_or(tag);
    Version::parse(cleaned).context(format!("Failed to parse version from tag: {tag}"))
}

/// Check GitHub for a newer release. Returns `None` if already up to date.
pub async fn check_for_update() -> Result<Option<UpdateInfo>> {
    let current = Version::parse(CURRENT_VERSION)
        .context("Failed to parse current version from Cargo.toml")?;

    let client = reqwest::Client::new();
    let response = client
        .get(GITHUB_RELEASES_API)
        .header("User-Agent", "Clawling")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .context("Failed to reach GitHub releases API")?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        // No releases exist yet
        return Ok(None);
    }

    if !response.status().is_success() {
        anyhow::bail!("GitHub API returned status {}", response.status());
    }

    let release: GitHubRelease = response
        .json()
        .await
        .context("Failed to parse GitHub release JSON")?;

    let latest = parse_tag_version(&release.tag_name)?;

    if latest <= current {
        return Ok(None);
    }

    let expected_asset = platform_asset_name()?;

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == expected_asset)
        .context(format!(
            "Release {} exists but has no asset for this platform (expected {expected_asset})",
            release.tag_name
        ))?;

    Ok(Some(UpdateInfo {
        current,
        latest,
        asset_url: asset.browser_download_url.clone(),
        asset_name: asset.name.clone(),
    }))
}

/// Download the release asset and replace the running binary.
pub async fn download_and_replace(info: &UpdateInfo) -> Result<()> {
    let current_exe = std::env::current_exe().context("Could not determine current executable path")?;

    println!(
        "Downloading {} (v{} -> v{})...",
        info.asset_name, info.current, info.latest
    );

    // Download the asset
    let client = reqwest::Client::new();
    let response = client
        .get(&info.asset_url)
        .header("User-Agent", "Clawling")
        .send()
        .await
        .context("Failed to download release asset")?;

    if !response.status().is_success() {
        anyhow::bail!(
            "Failed to download asset: HTTP {}",
            response.status()
        );
    }

    let bytes = response
        .bytes()
        .await
        .context("Failed to read release asset body")?;

    println!("Downloaded {} bytes.", bytes.len());

    // Extract the binary from the archive
    let new_binary = extract_binary(&bytes, &info.asset_name)?;

    // Replace the current binary
    replace_binary(&current_exe, &new_binary)?;

    println!("Updated successfully to v{}!", info.latest);
    println!("Restart clawling to use the new version.");

    Ok(())
}

/// Extract the `clawling` (or `clawling.exe`) binary from the downloaded archive.
fn extract_binary(archive_bytes: &[u8], asset_name: &str) -> Result<Vec<u8>> {
    let binary_name = if cfg!(target_os = "windows") {
        "clawling.exe"
    } else {
        "clawling"
    };

    if asset_name.ends_with(".zip") {
        extract_from_zip(archive_bytes, binary_name)
    } else if asset_name.ends_with(".tar.gz") {
        extract_from_targz(archive_bytes, binary_name)
    } else {
        anyhow::bail!("Unknown archive format: {asset_name}");
    }
}

/// Extract a file from a .zip archive.
fn extract_from_zip(data: &[u8], file_name: &str) -> Result<Vec<u8>> {
    let cursor = io::Cursor::new(data);
    let mut archive = zip::ZipArchive::new(cursor).context("Failed to open zip archive")?;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        // Match the binary by filename (may be inside a subdirectory)
        if name == file_name || name.ends_with(&format!("/{file_name}")) {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;
            return Ok(buf);
        }
    }

    anyhow::bail!("Could not find {file_name} inside zip archive");
}

/// Extract a file from a .tar.gz archive.
fn extract_from_targz(data: &[u8], file_name: &str) -> Result<Vec<u8>> {
    use std::io::Cursor;

    let cursor = Cursor::new(data);
    let gz = flate2::read::GzDecoder::new(cursor);
    let mut archive = tar::Archive::new(gz);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_path_buf();
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        if name == file_name {
            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)?;
            return Ok(buf);
        }
    }

    anyhow::bail!("Could not find {file_name} inside tar.gz archive");
}

/// Replace the running binary with new content.
/// On Windows, we can't overwrite a running exe, so we rename the old one first.
fn replace_binary(current_exe: &Path, new_binary: &[u8]) -> Result<()> {
    if cfg!(target_os = "windows") {
        // Windows strategy: rename old -> write new -> schedule cleanup
        let old_path = current_exe.with_extension("exe.old");

        // Clean up any previous .old file
        let _ = fs::remove_file(&old_path);

        // Rename the running binary out of the way
        fs::rename(current_exe, &old_path).context(
            "Failed to rename current binary. Try running as administrator.",
        )?;

        // Write the new binary in its place
        let mut file = fs::File::create(current_exe)
            .context("Failed to write new binary")?;
        file.write_all(new_binary)?;
        file.flush()?;

        println!(
            "(Old binary saved as {} — you can delete it.)",
            old_path.display()
        );
    } else {
        // Unix strategy: write to a temp file, set executable, rename (atomic on same fs)
        let temp_path = current_exe.with_extension("new");

        let mut file = fs::File::create(&temp_path)
            .context("Failed to write new binary to temp location")?;
        file.write_all(new_binary)?;
        file.flush()?;

        // Set executable permission
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = fs::Permissions::from_mode(0o755);
            fs::set_permissions(&temp_path, perms)?;
        }

        // Atomic rename
        fs::rename(&temp_path, current_exe).context("Failed to replace binary")?;
    }

    Ok(())
}

/// Prompt the user to accept an update. Returns true if they say yes.
fn prompt_update(info: &UpdateInfo) -> bool {
    println!();
    println!(
        "A new version of Clawling is available: v{} (current: v{})",
        info.latest, info.current
    );
    print!("Would you like to update? [y/N] ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().lock().read_line(&mut input).is_err() {
        return false;
    }

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Run the explicit `clawling update` command.
pub async fn run_update() -> Result<()> {
    println!(
        "Clawling v{} — checking for updates...",
        CURRENT_VERSION
    );

    match check_for_update().await {
        Ok(Some(info)) => {
            if prompt_update(&info) {
                download_and_replace(&info).await?;
            } else {
                println!("Update skipped.");
            }
        }
        Ok(None) => {
            println!("You're already running the latest version.");
        }
        Err(e) => {
            eprintln!("Update check failed: {e}");
        }
    }

    Ok(())
}

/// Check for updates during `clawling wake`, respecting config.
/// This is non-blocking in the sense that failures are silently ignored.
pub async fn maybe_check_on_wake() {
    let config = load_config();
    if !config.auto_update_check {
        return;
    }

    // Use a short timeout so we don't slow down wake if GitHub is unreachable
    match tokio::time::timeout(
        std::time::Duration::from_secs(5),
        check_for_update(),
    )
    .await
    {
        Ok(Ok(Some(info))) => {
            println!(
                "Update available: v{} -> v{} (run `clawling update` to install)",
                info.current, info.latest
            );
        }
        // All other cases: up to date, error, timeout — just stay quiet
        _ => {}
    }
}
