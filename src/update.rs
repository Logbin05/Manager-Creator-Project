use std::{sync::LazyLock, time::Duration};
use serde::Deserialize;

pub static REPO: LazyLock<String> = LazyLock::new(|| {
    let url = std::env::var("MCP_REPO")
        .unwrap_or_else(|_| env!("CARGO_PKG_REPOSITORY").to_string());
    url.trim_start_matches("https://github.com/")
        .trim_end_matches('/')
        .trim_end_matches(".git")
        .to_string()
});

pub const CURRENT: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub version: semver::Version,
    pub url: String,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    html_url: String,
}

pub fn check() -> Result<Option<UpdateInfo>, String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build()
        .into();

    let url = format!("https://api.github.com/repos/{}/releases/latest", *REPO);
    let mut resp = agent
        .get(&url)
        .header("User-Agent", "mcp")
        .call()
        .map_err(|e| e.to_string())?;
    let release: Release = resp.body_mut().read_json().map_err(|e| e.to_string())?;

    let latest = semver::Version::parse(release.tag_name.trim_start_matches('v'))
        .map_err(|e| e.to_string())?;
    let current = semver::Version::parse(CURRENT).map_err(|e| e.to_string())?;

    Ok((latest > current).then_some(UpdateInfo { version: latest, url: release.html_url }))
}

pub fn install_hint() -> String {
    let base = format!(
        "https://github.com/{}/releases/latest/download/manager-creator-project-installer",
        *REPO
    );
    if cfg!(windows) {
        format!("powershell -ExecutionPolicy Bypass -c \"irm {base}.ps1 | iex\"")
    } else {
        format!("curl --proto '=https' --tlsv1.2 -LsSf {base}.sh | sh")
    }
}