use std::process::Command;

pub struct Tool {
    pub id: &'static str,
    bin: &'static str,
    win_bin: Option<&'static str>,
    runner: Option<&'static str>,
    pub runner_flags: &'static [&'static str],
    pub is_manager: bool,
    pub label: &'static str,
    pub hint: &'static str,
    pub install_url: &'static str,
}

#[allow(clippy::too_many_arguments)]
const fn tool(
    id: &'static str,
    bin: &'static str,
    win_bin: Option<&'static str>,
    runner: Option<&'static str>,
    runner_flags: &'static [&'static str],
    is_manager: bool,
    label: &'static str,
    hint: &'static str,
    install_url: &'static str,
) -> Tool {
    Tool {
        id,
        bin,
        win_bin,
        runner,
        runner_flags,
        is_manager,
        label,
        hint,
        install_url,
    }
}

pub const TOOLS: &[Tool] = &[
    tool(
        "cargo",
        "cargo",
        None,
        None,
        &[],
        true,
        "cargo",
        "Rust toolchain",
        "https://rustup.rs",
    ),
    tool(
        "npm",
        "npm",
        Some("npm.cmd"),
        Some("npx"),
        &["--yes"],
        true,
        "npm",
        "ships with Node.js",
        "https://nodejs.org",
    ),
    tool(
        "pnpm",
        "pnpm",
        Some("pnpm.cmd"),
        Some("pnpm"),
        &["dlx"],
        true,
        "pnpm",
        "fast, disk-efficient",
        "https://pnpm.io",
    ),
    tool(
        "bun",
        "bun",
        None,
        Some("bunx"),
        &[],
        true,
        "bun",
        "fastest, needs bun",
        "https://bun.sh",
    ),
    tool(
        "deno",
        "deno",
        None,
        None,
        &[],
        true,
        "deno",
        "TypeScript runtime",
        "https://deno.com",
    ),
    tool(
        "go",
        "go",
        None,
        None,
        &[],
        true,
        "go",
        "Go toolchain",
        "https://go.dev/dl",
    ),
    tool(
        "uv",
        "uv",
        None,
        Some("uvx"),
        &[],
        true,
        "uv",
        "fast Python manager",
        "https://docs.astral.sh/uv",
    ),
    tool(
        "poetry",
        "poetry",
        None,
        None,
        &[],
        true,
        "poetry",
        "Python, pyproject",
        "https://python-poetry.org",
    ),
    tool(
        "composer",
        "composer",
        None,
        None,
        &[],
        true,
        "composer",
        "PHP",
        "https://getcomposer.org",
    ),
    tool(
        "dotnet",
        "dotnet",
        None,
        None,
        &[],
        true,
        "dotnet",
        ".NET SDK",
        "https://dotnet.microsoft.com",
    ),
    tool(
        "git",
        "git",
        None,
        None,
        &[],
        false,
        "git",
        "version control",
        "https://git-scm.com",
    ),
];

pub fn find(id: &str) -> Option<&'static Tool> {
    TOOLS.iter().find(|t| t.id == id)
}

impl Tool {
    pub fn bin(&self) -> &'static str {
        match (cfg!(windows), self.win_bin) {
            (true, Some(w)) => w,
            _ => self.bin,
        }
    }

    pub fn runner(&self) -> Option<&'static str> {
        self.runner
    }

    fn installed(&self) -> bool {
        let r = Command::new(self.bin()).arg("--version").output();
        eprintln!("check bin={:?} -> {:?}", self.bin(), r.as_ref().map(|o| o.status));
        r.map(|o| o.status.success()).unwrap_or(false)
    }
}

pub fn ensure_installed(tools: &[&'static Tool]) -> Result<(), String> {
    let missing: Vec<&Tool> = tools.iter().copied().filter(|t| !t.installed()).collect();
    if missing.is_empty() {
        return Ok(());
    }

    let list = missing
        .iter()
        .map(|t| format!("  {} — {}", t.label, t.install_url))
        .collect::<Vec<_>>()
        .join("\n");
    Err(format!("These tools are required but not found:\n{list}"))
}
