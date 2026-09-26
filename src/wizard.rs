use crate::{
    config::Config,
    spec::ProjectSpec,
    templates::{self},
};
use std::{
    io::{self, ErrorKind},
    path::Path,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Step {
    Template,
    Name,
    Manager,
    Layout,
    Libs,
    Confirm,
}

fn steps_for(spec: &ProjectSpec) -> Vec<Step> {
    let m = &spec.template().manifest;
    let mut steps = vec![Step::Template, Step::Name];
    if m.managers().len() > 1 {
        steps.push(Step::Manager);
    }
    if !m.layouts.is_empty() {
        steps.push(Step::Layout);
    }

    let no_libs = m
        .layouts
        .get(spec.answers.layout)
        .map(|l| l.no_libs)
        .unwrap_or(false);
    if !m.libs.is_empty() && !no_libs {
        steps.push(Step::Libs);
    }

    steps.push(Step::Confirm);
    steps
}

fn neighbour(spec: &ProjectSpec, step: Step, delta: isize) -> Option<Step> {
    let steps = steps_for(spec);
    let pos = steps.iter().position(|s| *s == step)? as isize;
    let next = pos + delta;
    (next >= 0)
        .then(|| steps.get(next as usize).copied())
        .flatten()
}

fn ask_template(spec: &mut ProjectSpec, cfg: &Config) -> io::Result<()> {
    let mut prompt = cliclack::select("What are we creating?");
    for (i, tpl) in templates::template::ALL.iter().enumerate() {
        prompt = prompt.item(i, &tpl.manifest.name, &tpl.manifest.hint);
    }

    spec.template = prompt.initial_value(spec.template).interact()?;

    let managers = spec.template().manifest.managers();
    if !managers.iter().any(|t| t.id == spec.answers.manager) {
        spec.answers.manager = managers
            .iter()
            .find(|t| t.id == cfg.default_manager)
            .or_else(|| managers.first())
            .map(|t| t.id.to_string())
            .unwrap_or_default();
    }

    spec.answers.layout = 0;
    spec.answers.libs.clear();
    Ok(())
}

fn validate_name(name: &str, npm_rules: bool) -> Result<(), String> {
    let Some(first) = name.chars().next() else {
        return Err("name cannot be empty".into());
    };
    if first.is_ascii_digit() {
        return Err("name cannot start with a digit".into());
    }
    if !name
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("only latin letters, digits, - and _".into());
    }
    if npm_rules {
        if name.chars().any(|c| c.is_ascii_uppercase()) {
            return Err("npm does not allow uppercase letters".into());
        }
        if first == '_' {
            return Err("npm does not allow a leading _".into());
        }
    }
    if Path::new(name).exists() {
        return Err(format!("./{name} already exists"));
    }
    Ok(())
}

fn ask_name(spec: &mut ProjectSpec) -> io::Result<()> {
    let npm_rules = matches!(
        spec.answers.manager.as_str(),
        "npm" | "pnpm" | "bun" | "yarn"
    );
    spec.name = cliclack::input("Project name")
        .placeholder("my-app")
        .default_input(&spec.name)
        .validate(move |name: &String| validate_name(name, npm_rules))
        .interact()?;
    Ok(())
}

fn ask_manager(spec: &mut ProjectSpec) -> io::Result<()> {
    let managers = spec.template().manifest.managers();
    let mut prompt = cliclack::select("Package manager");
    for t in &managers {
        prompt = prompt.item(t.id.to_string(), t.label, t.hint);
    }

    spec.answers.manager = prompt
        .initial_value(spec.answers.manager.clone())
        .interact()?;
    Ok(())
}

fn ask_layout(spec: &mut ProjectSpec) -> io::Result<()> {
    let m = &spec.template().manifest;
    let mut prompt = cliclack::select("Architecture");
    for (i, l) in m.layouts.iter().enumerate() {
        prompt = prompt.item(i, &l.label, &l.hint);
    }
    spec.answers.layout = prompt.initial_value(spec.answers.layout).interact()?;
    Ok(())
}

fn ask_libs(spec: &mut ProjectSpec) -> io::Result<()> {
    let m = &spec.template().manifest;
    let mut prompt = cliclack::multiselect("Additional libraries");
    for (i, l) in m.libs.iter().enumerate() {
        prompt = prompt.item(i, &l.label, &l.hint);
    }
    spec.answers.libs = prompt
        .initial_values(spec.answers.libs.clone())
        .required(false)
        .interact()?;
    Ok(())
}

fn ask_confirm(spec: &ProjectSpec) -> io::Result<bool> {
    cliclack::note(
        "Check before creating",
        format!(
            "{:<9}{}\n{:<9}{}\n{}",
            "Project",
            spec.name,
            "Template",
            spec.template().manifest.name,
            spec.summary()
        ),
    )?;
    cliclack::confirm("Create the project?").initial_value(true).interact()
}

pub fn run(cfg: &Config) -> io::Result<Option<ProjectSpec>> {
    cliclack::intro(format!("{} · new project", crate::APP_NAME))?;

    let mut spec = ProjectSpec::default();
    spec.answers.manager = cfg.default_manager.clone();
    let mut step = Step::Template;

    loop {
        let result = match step {
            Step::Template => ask_template(&mut spec, cfg),
            Step::Name => ask_name(&mut spec),
            Step::Manager => ask_manager(&mut spec),
            Step::Layout => ask_layout(&mut spec),
            Step::Libs => ask_libs(&mut spec),
            Step::Confirm => match ask_confirm(&spec) {
                Ok(true) => return Ok(Some(spec)),
                Ok(false) => {
                    step = Step::Template;
                    continue;
                }
                Err(e) => Err(e),
            },
        };

        match result {
            Ok(()) => step = neighbour(&spec, step, 1).unwrap_or(Step::Confirm),
            Err(e) if e.kind() == ErrorKind::Interrupted => match neighbour(&spec, step, -1) {
                Some(prev) => step = prev,
                None => return Ok(None),
            },
            Err(e) => return Err(e),
        }
    }
}