use std::io;
use super::model::{HttpVersions, RustAnswers, EXTRA_LIBS};

pub fn ask_http(a: &mut RustAnswers) -> io::Result<()> {
    a.http = cliclack::select("HTTP version")
        .item(
            HttpVersions::H1,
            HttpVersions::H1.label(),
            "axum, the easiest way to get started",
        )
        .item(
            HttpVersions::H2,
            HttpVersions::H2.label(),
            "axum + h2c, without TLS",
        )
        .item(HttpVersions::H3, HttpVersions::H3.label(), "In development")
        .initial_value(a.http)
        .interact()?;
    Ok(())
}

pub fn ask_libs(a: &mut RustAnswers) -> io::Result<()> {
    let base: Vec<&str> = a.http.base_deps().iter().map(|d| d.name).collect();
    cliclack::log::info(format!(
        "Basic for {}: {}",
        a.http.label(),
        base.join(", ")
    ))?;

    let mut prompt = cliclack::multiselect("Additional crates");
    for (dep, hint) in EXTRA_LIBS {
        prompt = prompt.item(*dep, dep.name, *hint);
    }
    a.extra_deps = prompt
        .initial_values(a.extra_deps.clone())
        .required(false)
        .interact()?;
    Ok(())
}