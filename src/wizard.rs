use std::{
    io::{self, ErrorKind},
    path::Path,
};
use crate::{
    spec::ProjectSpec,
    stacks::{react, rust, Stack},
};

#[derive(Clone, Copy)]
enum Step { Stack, Name, Http, RustLibs, Pm, Arch, ReactLibs, Confirm }

impl Step {
    fn next(self, stack: Stack) -> Step {
        match (self, stack) {
            (Step::Stack, _) => Step::Name,
            (Step::Name, Stack::RustBackend) => Step::Http,
            (Step::Name, Stack::React) => Step::Pm,
            (Step::Http, _) => Step::RustLibs,
            (Step::Pm, _) => Step::Arch,
            (Step::Arch, _) => Step::ReactLibs,
            (Step::RustLibs | Step::ReactLibs | Step::Confirm, _) => Step::Confirm,
        }
    }

    fn prev(self, stack: Stack) -> Option<Step> {
        Some(match (self, stack) {
            (Step::Stack, _) => return None,
            (Step::Name, _) => Step::Stack,
            (Step::Http | Step::Pm, _) => Step::Name,
            (Step::RustLibs, _) => Step::Http,
            (Step::Arch, _) => Step::Pm,
            (Step::ReactLibs, _) => Step::Arch,
            (Step::Confirm, Stack::RustBackend) => Step::RustLibs,
            (Step::Confirm, Stack::React) => Step::ReactLibs,
        })
    }
}

pub fn run() -> io::Result<Option<ProjectSpec>> {
    cliclack::intro(format!("{} · new project", crate::APP_NAME))?;

    let mut spec = ProjectSpec::default();
    let mut step = Step::Stack;

    loop {
        let result = match step {
            Step::Stack => ask_stack(&mut spec),
            Step::Name => ask_name(&mut spec),
            Step::Http => rust::ask_http(&mut spec.rust),
            Step::RustLibs => rust::ask_libs(&mut spec.rust),
            Step::Pm => react::ask_pm(&mut spec.react),
            Step::Arch => react::ask_arch(&mut spec.react),
            Step::ReactLibs => react::ask_libs(&mut spec.react),
            Step::Confirm => match ask_confirm(&spec) {
                Ok(true) => return Ok(Some(spec)),
                Ok(false) => {
                    step = Step::Stack;
                    continue;
                }
                Err(e) => Err(e),
            },
        };

        match result {
            Ok(()) => step = step.next(spec.stack),
            Err(e) if e.kind() == ErrorKind::Interrupted => match step.prev(spec.stack) {
                Some(prev) => step = prev,
                None => return Ok(None),
            },
            Err(e) => return Err(e),
        }
    }
}

fn ask_stack(spec: &mut ProjectSpec) -> io::Result<()> {
    spec.stack = cliclack::select("What are we creating?")
        .item(Stack::RustBackend, Stack::RustBackend.label(), "axum, HTTP/1.1–3")
        .item(Stack::React, Stack::React.label(), "Vite + TypeScript")
        .initial_value(spec.stack)
        .interact()?;
    Ok(())
}

fn ask_name(spec: &mut ProjectSpec) -> io::Result<()> {
    let stack = spec.stack;
    spec.name = cliclack::input("Project name")
        .placeholder("my-app")
        .default_input(&spec.name)
        .validate(move |name: &String| validate_name(name, stack))
        .interact()?;
    Ok(())
}

fn validate_name(name: &str, stack: Stack) -> Result<(), String> {
    let Some(first) = name.chars().next() else {
        return Err("The name cannot be empty".into());
    };
    if first.is_ascii_digit() {
        return Err("A name cannot start with a number".into());
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err("only Latin letters, numbers, - and _".into());
    }
    if stack == Stack::React {
        react::check_name(name)?;
    }
    if Path::new(name).exists() {
        return Err(format!("The ./{name} folder already exists"));
    }
    Ok(())
}

fn ask_confirm(spec: &ProjectSpec) -> io::Result<bool> {
    cliclack::note(
        "Check before creating",
        format!("Project   {}\nStack     {}\n{}", spec.name, spec.stack.label(), spec.summary()),
    )?;
    cliclack::confirm("Create a project?").initial_value(true).interact()
}