pub mod config;
pub mod process;
pub mod settings;
pub mod spec;
pub mod templates;
pub mod toolchain;
pub mod ui;
pub mod update;
pub mod wizard;

use std::{
    io,
    sync::{Arc, Mutex},
    thread,
};
use ui::start_screen::{self, MenuItem};
pub const APP_NAME: &str = "Manager Creator Project";
use crate::{config::Config, ui::start_screen::Notice};

pub fn run() -> io::Result<()> {
    dotenvy::dotenv().ok();
    let mut cfg = Config::load();
     if let Some(dir) = &cfg.projects_dir {
        if let Err(e) = std::env::set_current_dir(dir) {
            eprintln!("warning: cannot use projects directory {}: {e}", dir.display());
            eprintln!("         creating projects in the current directory instead");
        }
    }

    let notice: Notice = Arc::new(Mutex::new(None));
    if cfg.check_updates && cfg.update_check_due() {
        cfg.mark_update_checked();
        cfg.save()?;
        let notice = Arc::clone(&notice);
        thread::spawn(move || {
            if let Ok(Some(u)) = update::check() {
                *notice.lock().unwrap() =
                    Some(format!("v{} is available · see Settings", u.version));
            }
        });
    }
    loop {
        match start_screen::run(&notice)? {
            MenuItem::NewProject => {
                let Some(spec) = wizard::run(&cfg)? else {
                    continue;
                };

                match templates::generated::generate(spec.template(), &spec.name, &spec.answers) {
                    Ok(()) => {
                        let path = std::env::current_dir()?.join(&spec.name);
                        cliclack::note(
                            "All done!",
                            format!("Project   {}\nRun    {}", path.display(), spec.run_hint()),
                        )?;
                    }
                    Err(e) => {
                        cliclack::outro_cancel(format!("Unable to create the project:\n{e}"))?
                    }
                }
                return Ok(());
            }
            MenuItem::Settings => settings::run(&mut cfg)?,
            MenuItem::Quit => return Ok(()),
            _ => {}
        }
    }
}
