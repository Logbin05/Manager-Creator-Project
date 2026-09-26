use crate::{
    process::run,
    templates::structures::{CWD, Lib, Manifest, Phase, Step, Template},
    toolchain::{self, Tool},
};
use ::include_dir::Dir;
use std::{fs, io, path::Path, str};

#[derive(Debug, Default)]
pub struct TemplateAnswer {
    pub manager: String,
    pub layout: usize,
    pub libs: Vec<usize>,
}

impl TemplateAnswer {
    pub fn manager_tool(&self) -> Option<&'static Tool> {
        toolchain::find(&self.manager)
    }

    pub fn selected_libs<'a>(&'a self, m: &'a Manifest) -> impl Iterator<Item = &'a Lib> {
        self.libs.iter().filter_map(|i| m.libs.get(*i))
    }

    pub fn summary(&self, tpl: &Template) -> String {
        let m = &tpl.manifest;
        let libs: Vec<&str> = self.selected_libs(m).map(|l| l.label.as_str()).collect();
        let libs = if libs.is_empty() {
            "—".into()
        } else {
            libs.join(", ")
        };

        let mut out = String::new();
        if let Some(t) = self.manager_tool() {
            out.push_str(&format!("{:<9}{}\n", "Tool", t.label));
        }

        if let Some(t) = m.layouts.get(self.layout) {
            out.push_str(&format!("{:<9}{}\n", "Layout", t.label));
        }

        out.push_str(&format!("{:<9}{}\n", "Libs", libs));
        out
    }
}

fn resolve(spec: &str, manager: Option<&'static Tool>) -> io::Result<(&'static str, Vec<String>)> {
    let need = || io::Error::other(format!("step needs a manager but none was chosen: {spec}"));

    match spec {
        "{{manager}}" => Ok((manager.ok_or_else(need)?.bin(), Vec::new())),
        "{{manager.runner}}" => {
            let t = manager.ok_or_else(need)?;
            let runner = t
                .runner()
                .ok_or_else(|| io::Error::other(format!("{} has no runner", t.label)))?;
            Ok((
                runner,
                t.runner_flags.iter().map(|s| s.to_string()).collect(),
            ))
        }
        id => {
            let t = toolchain::find(id)
                .ok_or_else(|| io::Error::other(format!("unknown tool '{id}'")))?;
            Ok((t.bin(), Vec::new()))
        }
    }
}

fn subst(s: &str, name: &str, manager: Option<&'static Tool>) -> String {
    s.replace("{{name}}", name)
        .replace("{{manager}}", manager.map(|t| t.bin()).unwrap_or(""))
}

fn exec(
    step: &Step,
    dir: &Path,
    name: &str,
    manager: Option<&'static Tool>,
    packages: &[&str],
) -> io::Result<()> {
    let (program, mut args) = resolve(&step.program, manager)?;

    for arg in &step.args {
        if arg == "{{packages}}" {
            args.extend(packages.iter().map(|p| p.to_string()));
        } else {
            args.push(subst(arg, name, manager));
        }
    }

    let cwd = match step.cwd() {
        CWD::Parent => Path::new("."),
        CWD::Project => dir,
    };
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    cliclack::log::step(format!("{program} {}", refs.join(" ")))?;
    run(program, cwd, &refs)
}

fn copy_dir(tpl: &Template, src: &Path, dest: &Path, name: &str) -> io::Result<()> {
    let Some(dir) = tpl.dir().get_dir(src) else {
        return Ok(());
    };
    copy_recursive(dir, src, dest, name)
}

fn copy_recursive(dir: &Dir<'_>, strip: &Path, dest: &Path, name: &str) -> io::Result<()> {
    for file in dir.files() {
        let rel = file.path().strip_prefix(strip).map_err(io::Error::other)?;
        let out = dest.join(rel);
        if let Some(parent) = out.parent() {
            fs::create_dir_all(parent)?;
        }

        match file.contents_utf8() {
            Some(text) => fs::write(out, text.replace("{{name}}", name))?,
            None => fs::write(out, file.contents())?,
        }
    }
    for sub in dir.dirs() {
        copy_recursive(sub, strip, dest, name)?;
    }
    Ok(())
}

fn remove_path(dir: &Path, rel: &str) -> io::Result<()> {
    if rel.contains("..") || Path::new(rel).is_absolute() {
        return Err(io::Error::other(format!("unsafe path in remove: {rel}")));
    }
    let path = dir.join(rel);
    match path.metadata() {
        Ok(m) if m.is_dir() => fs::remove_dir_all(path),
        Ok(_) => fs::remove_file(path),
        Err(_) => Ok(()),
    }
}

pub fn generate(tpl: &Template, name: &str, a: &TemplateAnswer) -> io::Result<()> {
    let m = &tpl.manifest;
    let manager = a.manager_tool();
    let dir = Path::new(name);

    toolchain::ensure_installed(&m.required_tools(manager)).map_err(io::Error::other)?;

    let layout = m.layouts.get(a.layout);
    let layout_steps: &[Step] = layout.map(|l| l.steps.as_slice()).unwrap_or(&[]);

    let create_steps: Vec<&Step> = m
        .steps
        .iter()
        .chain(layout_steps)
        .filter(|s| s.phase == Phase::Create)
        .collect();
    for step in &create_steps {
        exec(step, dir, name, manager, &[])?;
    }
    if create_steps.is_empty() {
        fs::create_dir_all(dir)?;
    }

    for rel in m
        .remove
        .iter()
        .chain(layout.map(|l| l.remove.as_slice()).unwrap_or(&[]))
    {
        remove_path(dir, rel)?;
    }

    copy_dir(tpl, &tpl.root.join("files"), dir, name)?;
    if let Some(l) = layout {
        let src = tpl.root.join("layouts").join(&l.id);
        if tpl.dir().get_dir(&src).is_some() {
            cliclack::log::step(format!("layout: {}", l.label))?;
            copy_dir(tpl, &src, dir, name)?;
        }
    }

    for step in m
        .steps
        .iter()
        .chain(layout_steps)
        .filter(|s| s.phase == Phase::Setup)
    {
        exec(step, dir, name, manager, &[])?;
    }

    let libs: Vec<&Lib> = a.selected_libs(m).collect();
    if !libs.is_empty() {
        for step in m.steps.iter().filter(|s| s.phase == Phase::Libs) {
            if step.each {
                for lib in &libs {
                    let args: Vec<&str> = lib.packages.iter().map(String::as_str).collect();
                    exec(step, dir, name, manager, &args)?;
                }
            } else {
                let args: Vec<&str> = libs
                    .iter()
                    .flat_map(|l| l.packages.iter().map(String::as_str))
                    .collect();
                exec(step, dir, name, manager, &args)?;
            }
        }
    }
    Ok(())
}


pub fn run_hint(tpl: &Template, name: &str, a: &TemplateAnswer) -> String {
    let layout_hint = tpl
        .manifest
        .layouts
        .get(a.layout)
        .map(|l| l.run_hint.as_str())
        .unwrap_or("");
    let hint = if layout_hint.is_empty() { tpl.manifest.run_hint.as_str() } else { layout_hint };

    if hint.is_empty() {
        format!("cd {name}")
    } else {
        format!("cd {name} && {}", subst(hint, name, a.manager_tool()))
    }
}
