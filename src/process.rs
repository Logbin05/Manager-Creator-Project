use std::{io, path::Path, process::Command};

pub fn run(program: &str, dir: &Path, args: &[&str]) -> io::Result<()> {
    let output = Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => io::Error::other(format!(
                "{program} not found. Is it installed and included in the PATH?"
            )),
            _ => e,
        })?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let msg = if stderr.is_empty() {
            format!("{program} exited with {}", output.status)
        } else {
            stderr
        };
        Err(io::Error::other(msg))
    }
}
