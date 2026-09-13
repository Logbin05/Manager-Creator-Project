use super::model::{HttpVersions, RustAnswers};
use crate::process::run;
use std::{fs, io, path::Path};

const AXUM_MAIN: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/axum_main.rs.tpl"
));
const H3_STUB: &str = "fn main() {\n    println!(\"HTTP/3 template: coming soon\");\n}\n";

pub fn generate(name: &str, a: &RustAnswers) -> io::Result<()> {
    let dir = Path::new(name);

    cliclack::log::step(format!("cargo new {name}"))?;
    run("cargo", Path::new("."), &["new", name])?;

    for dep in a.all_deps() {
        let features = dep.features.join(",");
        let mut args = vec!["add", dep.name];
        if !features.is_empty() {
            args.extend(["--features", features.as_str()]);
        }
        cliclack::log::step(format!("cargo {}", args.join(" ")))?;
        run("cargo", dir, &args)?;
    }

    let main_rs = match a.http {
        HttpVersions::H1 | HttpVersions::H2 => AXUM_MAIN,
        HttpVersions::H3 => H3_STUB,
    };
    fs::write(dir.join("src/main.rs"), main_rs)
}
