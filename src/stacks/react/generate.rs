use super::model::ReactAnswers;
use crate::process::run;
use std::{fs, io, path::Path};

pub fn generate(name: &str, a: &ReactAnswers) -> io::Result<()> {
    let dir = Path::new(name);
    let pm = a.pm;

    let mut args = pm.runner_flags().to_vec();
    args.extend([
        "create-vite@latest",
        name,
        "--template",
        "react-ts",
        "--no-interactive",
    ]);
    cliclack::log::step(format!("{} {}", pm.runner(), args.join(" ")))?;
    run(pm.runner(), Path::new("."), &args)?;

    cliclack::log::step(format!("architecture: {}", a.arch.label()))?;
    for (file, content) in a.arch.files() {
        let path = dir.join(file);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
    }

    cliclack::log::step(format!("{} install", pm.label()))?;
    run(pm.bin(), dir, &["install"])?;

    let packages: Vec<&str> = a
        .libs
        .iter()
        .flat_map(|l| l.packages.iter().copied())
        .collect();
    if !packages.is_empty() {
        let mut args = vec!["add"];
        args.extend(packages);
        cliclack::log::step(format!("{} {}", pm.label(), args.join(" ")))?;
        run(pm.bin(), dir, &args)?;
    }

    Ok(())
}
