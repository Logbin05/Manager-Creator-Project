pub mod react;
pub mod rust;

use crate::spec::ProjectSpec;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Stack {
    #[default]
    RustBackend,
    React,
}

impl Stack {
    pub fn label(self) -> &'static str {
        match self {
            Self::RustBackend => "Rust · Backend",
            Self::React => "React · Frontend",
        }
    }
}

pub fn generate(spec: &ProjectSpec) -> io::Result<()> {
    match spec.stack {
        Stack::RustBackend => rust::generate(&spec.name, &spec.rust),
        Stack::React => react::generate(&spec.name, &spec.react),
    }
}
