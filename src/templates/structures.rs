use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Runtime {
    Node,
    Rust,
    #[default]
    None,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Program {
    Pm,
    Runner,
    Cargo,
    Git,
    Go,
    Deno,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Create,
    #[default]
    Setup,
    Libs,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CWD {
    Parent,
    Project,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CMD {
    pub program: Program,
    pub args: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub hint: String,
    #[serde(default)]
    pub run_hint: String,
    #[serde(default)]
    pub steps: Vec<Step>,
    #[serde(default)]
    pub remove: Vec<String>,
    #[serde(default)]
    pub no_libs: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lib {
    pub label: String,
    pub packages: Vec<String>,
    #[serde(default)]
    pub hint: String,
}

fn default_order() -> u32 {
    100
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    pub id: String,
    pub name: String,
    pub hint: String,
    pub author: String,
    #[serde(default = "default_order")]
    pub order: u32,
    #[serde(default)]
    pub toolchain: Toolchain,
    #[serde(default)]
    pub run_hint: String,
    #[serde(default)]
    pub remove: Vec<String>,
    #[serde(default)]
    pub steps: Vec<Step>,
    #[serde(default)]
    pub layouts: Vec<Layout>,
    #[serde(default)]
    pub libs: Vec<Lib>,
}

#[derive(Debug)]
pub struct Template {
    pub manifest: Manifest,
    pub root: PathBuf,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Toolchain {
    #[serde(default)]
    pub manager: Vec<String>,
    #[serde(default)]
    pub requires: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub program: String,
    pub args: Vec<String>,
    #[serde(default)]
    pub phase: Phase,
    #[serde(default)]
    pub cwd: Option<CWD>,
    #[serde(default)]
    pub each: bool,
}
