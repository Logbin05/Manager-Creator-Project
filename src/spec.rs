use crate::stacks::{react::ReactAnswers, rust::RustAnswers, Stack};

#[derive(Debug, Default)]
pub struct ProjectSpec {
    pub stack: Stack,
    pub name: String,
    pub rust: RustAnswers,
    pub react: ReactAnswers,
}

impl ProjectSpec {
    pub fn summary(&self) -> String {
        match self.stack {
            Stack::RustBackend => self.rust.summary(),
            Stack::React => self.react.summary(),
        }
    }

    pub fn run_hint(&self) -> String {
        match self.stack {
            Stack::RustBackend => format!("cd {} && cargo run", self.name),
            Stack::React => format!("cd {} && {} run dev", self.name, self.react.pm.label()),
        }
    }
}