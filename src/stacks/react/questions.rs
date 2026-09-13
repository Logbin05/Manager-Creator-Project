use std::io;
use super::{model::{Architecture, NPM_LIBS, ReactAnswers}, manager_packet::PackageManager};

pub fn ask_pm(a: &mut ReactAnswers) -> io::Result<()> {
  let mut prompt = cliclack::select("Package manager");
  for pm in PackageManager::ALL {
    prompt = prompt.item(pm, pm.label(), pm.hint());
  }
  a.pm = prompt.initial_value(a.pm).interact()?;
  Ok(())
}

pub fn ask_arch(a: &mut ReactAnswers) -> io::Result<()> {
  let mut prompt = cliclack::select("Architecture");
  for arch in Architecture::ALL {
    prompt = prompt.item(arch, arch.label(), arch.hint());
  }
  a.arch = prompt.initial_value(a.arch).interact()?;
  Ok(())
}

pub fn ask_libs(a: &mut ReactAnswers) ->io::Result<()> {
  cliclack::log::info("Core: React, React-DOM, TypeScript, Vite")?;

  let mut prompt =cliclack::multiselect("Additional libraries");
  for lib in NPM_LIBS {
    prompt = prompt.item(*lib, lib.label, lib.hint);
  }
  a.libs = prompt.initial_values(a.libs.clone()).required(false).interact()?;
  Ok(())
}

pub fn check_name(name: &str) -> Result<(), String> {
    if name.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("npm does not allow uppercase letters in the name".into());
    }
    if name.starts_with('_') {
        return Err("npm does not allow an underscore (_) at the beginning of a name".into());
    }
    Ok(())
}