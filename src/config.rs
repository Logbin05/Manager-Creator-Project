use serde::{Deserialize, Serialize};
use std::{
    fs, io, path::{ PathBuf}, time::{SystemTime, UNIX_EPOCH},
};

const DAY: u64 = 24 * 60 * 60;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub check_updates: bool,
    pub last_update_check: u64,
    pub projects_dir: Option<PathBuf>,
    pub default_manager: String,
    pub git_init: bool,
    pub editor: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            check_updates: true,
            last_update_check: 0,
            projects_dir: None,
            default_manager: "npm".into(),
            git_init: true,
            editor: None,
        }
    }
}

pub fn path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("mcp")
        .join("config.toml")
}

fn now() -> u64 {
  SystemTime::now()
  .duration_since(UNIX_EPOCH)
  .map(|d| d.as_secs())
  .unwrap_or(0)
}

impl Config {
    pub fn load() -> Self {
        fs::read_to_string(path())
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) -> io::Result<()> {
      let path = path();
      if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
      }
      let text = toml::to_string_pretty(self).map_err(io::Error::other)?;
      fs::write(path, text)
    }

    pub fn update_check_due(&self) -> bool {
      now().saturating_sub(self.last_update_check) >= DAY
    }

    pub fn mark_update_checked(&mut self) {
      self.last_update_check = now();
    }
}
