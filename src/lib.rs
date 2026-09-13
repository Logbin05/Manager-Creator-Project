pub mod process;
pub mod spec;
pub mod stacks;
pub mod ui;
pub mod wizard;

use std::io;
use ui::start_screen::{self, MenuItem};
pub const APP_NAME: &str = "Manager Creator Project";

pub fn run() -> io::Result<()> {
    loop {
        match start_screen::run()? {
            MenuItem::NewProject => {
                let Some(spec) = wizard::run()? else { continue };

                match stacks::generate(&spec) {
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
            MenuItem::Quit => return Ok(()),
            _ => {}
        }
    }
}
