#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PackageManager {
    #[default]
    Npm,
    Bun,
}

impl PackageManager {
    pub const ALL: [Self; 2] = [Self::Npm, Self::Bun];

    pub fn label(self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Bun => "bun",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Npm => "comes bundled with Node.js",
            Self::Bun => "Hurry up, I need the bun installed.",
        }
    }

    pub fn bin(self) -> &'static str {
        match self {
            Self::Npm => {
                if cfg!(windows) {
                    "npx.cmd "
                } else {
                    "npx "
                }
            }
            Self::Bun => "bun",
        }
    }

    pub fn runner(self) -> &'static str {
        match self {
            Self::Npm => {
                if cfg!(windows) {
                    "npx.cmd"
                } else {
                    "npx"
                }
            }
            Self::Bun => "bunx",
        }
    }

    pub fn runner_flags(self) -> &'static [&'static str] {
        match self {
            Self::Npm => &["--yes"],
            Self::Bun => &[],
        }
    }
}
