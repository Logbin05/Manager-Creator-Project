use std::sync::LazyLock;

use include_dir::{include_dir, Dir};

use crate::{
    templates::structures::{Manifest, Phase, Step, Template, CWD},
    toolchain::{self, Tool},
};

static DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub static ALL: LazyLock<Vec<Template>> = LazyLock::new(|| {
    let mut list = Vec::new();
    for dir in DIR.dirs() {
        if dir.get_file(dir.path().join("template.toml")).is_none() {
            continue;
        }
        match Template::load(dir) {
            Ok(t) => list.push(t),
            Err(e) => {
                if cfg!(debug_assertions) {
                    eprintln!("template {}: {e}", dir.path().display());
                }
            }
        }
    }
    list.sort_by(|a, b| {
        a.manifest
            .order
            .cmp(&b.manifest.order)
            .then(a.manifest.id.cmp(&b.manifest.id))
    });
    list
});

impl Manifest {
    pub fn managers(&self) -> Vec<&'static Tool> {
        self.toolchain
            .manager
            .iter()
            .filter_map(|id| toolchain::find(id))
            .collect()
    }

    pub fn required_tools(&self, manager: Option<&'static Tool>) -> Vec<&'static Tool> {
        let mut tools: Vec<&Tool> = manager.into_iter().collect();
        tools.extend(
            self.toolchain
                .requires
                .iter()
                .filter_map(|id| toolchain::find(id)),
        );
        for step in self.all_steps() {
            if let Some(t) = toolchain::find(&step.program) {
                tools.push(t);
            }
        }

        tools.sort_by_key(|t| t.id);
        tools.dedup_by_key(|t| t.id);
        tools
    }

    pub fn all_steps(&self) -> impl Iterator<Item = &Step> {
        self.steps
            .iter()
            .chain(self.layouts.iter().flat_map(|l| &l.steps))
    }
}

impl Template {
    fn load(dir: &Dir<'static>) -> Result<Self, String> {
        let root = dir.path().to_path_buf();
        let file = dir
            .get_file(root.join("template.toml"))
            .ok_or("template.toml not found")?;
        let text = file
            .contents_utf8()
            .ok_or("template.toml is not valid UTF-8")?;
        let manifest: Manifest = toml::from_str(text).map_err(|e| e.to_string())?;

        if manifest.schema != 1 {
            return Err(format!("unsupported schema: {}", manifest.schema));
        }

        if manifest.id != root.to_string_lossy() {
            return Err(format!("id must match the folder name ({})", root.display()));
        }

        for id in manifest
            .toolchain
            .manager
            .iter()
            .chain(&manifest.toolchain.requires)
        {
            if toolchain::find(id).is_none() {
                return Err(format!("unknown tool '{id}' — add it to the registry first"));
            }
        }

        for step in manifest.all_steps() {
            let uses_manager = step.program.starts_with("{{manager");
            let known = step.program == "{{manager}}"
                || step.program == "{{manager.runner}}"
                || toolchain::find(&step.program).is_some();

            if !known {
                return Err(format!("step uses unknown program '{}'", step.program));
            }

            if uses_manager && manifest.toolchain.manager.is_empty() {
                return Err("step uses {{manager}} but toolchain.manager is empty".into());
            }

            if step.each && step.phase != Phase::Libs {
                return Err("`each` only makes sense in the libs phase".into());
            }
        }

        let has_create = manifest.all_steps().any(|s| s.phase == Phase::Create);
        let runs_inside = manifest.all_steps().any(|s| s.cwd() == CWD::Project);
        let has_files = dir.get_dir(root.join("files")).is_some();
        if !has_create && !runs_inside && !has_files {
            return Err("template has no create step, no files and nothing runs inside".into());
        }

        if !manifest.libs.is_empty() && !manifest.all_steps().any(|s| s.phase == Phase::Libs) {
            return Err("template offers libs but has no libs step".into());
        }

        for rel in manifest
            .remove
            .iter()
            .chain(manifest.layouts.iter().flat_map(|l| &l.remove))
        {
            if rel.contains("..") || std::path::Path::new(rel).is_absolute() {
                return Err(format!("unsafe path in remove: {rel}"));
            }
        }

        for layout in &manifest.layouts {
            let has_files = dir.get_dir(root.join("layouts").join(&layout.id)).is_some();
            if !has_files && layout.steps.is_empty() {
                return Err(format!(
                    "layout '{}' has neither files nor steps",
                    layout.id
                ));
            }
        }

        Ok(Self { manifest, root })
    }

    pub fn dir(&self) -> &'static Dir<'static> {
        DIR.get_dir(&self.root).expect("template dir exists")
    }
}

impl Step {
    pub fn cwd(&self) -> CWD {
        self.cwd.unwrap_or(match self.phase {
            Phase::Create => CWD::Parent,
            _ => CWD::Project,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_template_is_valid() {
        for dir in DIR.dirs() {
            if dir.get_file(dir.path().join("template.toml")).is_none() {
                continue;
            }
            if let Err(e) = Template::load(dir) {
                panic!("{}: {e}", dir.path().display());
            }
        }
    }

    #[test]
    fn built_in_templates_exist() {
        let ids: Vec<&str> = ALL.iter().map(|t| t.manifest.id.as_str()).collect();
        assert!(ids.contains(&"rust-axum"), "loaded: {ids:?}");
        assert!(ids.contains(&"react-vite"), "loaded: {ids:?}");
        assert!(ids.contains(&"vanilla"), "loaded: {ids:?}");
    }
}