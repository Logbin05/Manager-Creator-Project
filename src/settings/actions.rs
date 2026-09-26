use std::{
    io::{self, ErrorKind},
    path::PathBuf,
};
use crate::{config::Config, settings::Item, update};

pub fn open(item: Item, cfg: &mut Config) -> io::Result<()> {
    match item {
        Item::CheckNow => check_now(cfg)?,
        Item::ProjectsDir => ask_projects_dir(cfg)?,
        Item::Editor => ask_editor(cfg)?,
        Item::Reset => reset(cfg)?,
        _ => {}
    }
    cfg.save()
}

fn check_now(cfg: &mut Config) -> io::Result<()> {
    cliclack::intro("Check for updates")?;
    let spinner = cliclack::spinner();
    spinner.start("Asking GitHub…");

    match update::check() {
        Ok(Some(u)) => {
            spinner.stop(format!("Update available: v{} → v{}", update::CURRENT, u.version));
            cliclack::note("How to update", format!("{}\n\nNotes: {}", update::install_hint(), u.url))?;
        }
        Ok(None) => spinner.stop(format!("You're on the latest version (v{})", update::CURRENT)),
        Err(e) => spinner.error(format!("Could not check: {e}")),
    }

    cfg.mark_update_checked();
    cliclack::outro("Press enter to go back")?;
    let _ = cliclack::confirm("Back to settings?").initial_value(true).interact();
    Ok(())
}

fn ask_projects_dir(cfg: &mut Config) -> io::Result<()> {
    cliclack::intro("Projects directory")?;
    let current = cfg
        .projects_dir
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let input = cliclack::input("Path (empty = current directory)")
        .default_input(&current)
        .required(false)
        .validate(|s: &String| {
            let s = s.trim();
            if s.is_empty() || expand_home(s).is_dir() {
                Ok(())
            } else {
                Err("directory does not exist")
            }
        })
        .interact::<String>();

    if let Some(s) = cancelable(input)? {
        let s = s.trim();
        cfg.projects_dir = (!s.is_empty()).then(|| expand_home(s));
    }
    cliclack::outro("Saved")?;
    Ok(())
}

fn ask_editor(cfg: &mut Config) -> io::Result<()> {
    cliclack::intro("Open in editor")?;
    let input = cliclack::input("Command (empty = don't open)")
        .placeholder("code")
        .default_input(cfg.editor.as_deref().unwrap_or(""))
        .required(false)
        .interact::<String>();

    if let Some(s) = cancelable(input)? {
        let s = s.trim();
        cfg.editor = (!s.is_empty()).then(|| s.to_string());
    }
    cliclack::outro("Saved")?;
    Ok(())
}

fn reset(cfg: &mut Config) -> io::Result<()> {
    cliclack::intro("Reset settings")?;
    let yes = cliclack::confirm("Reset everything to defaults?")
        .initial_value(false)
        .interact();

    if let Some(true) = cancelable(yes)? {
        *cfg = Config::default();
        cliclack::outro("Settings reset")?;
    } else {
        cliclack::outro_cancel("Cancelled")?;
    }
    Ok(())
}

fn cancelable<T>(result: io::Result<T>) -> io::Result<Option<T>> {
    match result {
        Ok(v) => Ok(Some(v)),
        Err(e) if e.kind() == ErrorKind::Interrupted => Ok(None),
        Err(e) => Err(e),
    }
}

fn expand_home(s: &str) -> PathBuf {
    match s.strip_prefix("~/") {
        Some(rest) => dirs::home_dir().unwrap_or_default().join(rest),
        None => PathBuf::from(s),
    }
}