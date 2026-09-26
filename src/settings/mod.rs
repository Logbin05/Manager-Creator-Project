mod actions;
mod render;
use std::io;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
};
use crate::{config::Config, ui::guard::TerminalGuard};

pub enum Row {
    Section(&'static str),
    Item(Item),
    Gap,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Item {
    CheckOnStart,
    CheckNow,
    ProjectsDir,
    DefaultPm,
    GitInit,
    Editor,
    Reset,
}

impl Item {
    pub fn label(self) -> &'static str {
        match self {
            Self::CheckOnStart => "Check on startup",
            Self::CheckNow => "Check now",
            Self::ProjectsDir => "Projects directory",
            Self::DefaultPm => "Default package manager",
            Self::GitInit => "Git init after creation",
            Self::Editor => "Open in editor",
            Self::Reset => "Reset to defaults",
        }
    }

    pub fn is_toggle(self) -> bool {
        matches!(self, Self::CheckOnStart | Self::GitInit | Self::DefaultPm)
    }
}

pub const ROWS: &[Row] = &[
    Row::Section("UPDATES"),
    Row::Item(Item::CheckOnStart),
    Row::Item(Item::CheckNow),
    Row::Gap,
    Row::Section("PROJECTS"),
    Row::Item(Item::ProjectsDir),
    Row::Item(Item::DefaultPm),
    Row::Item(Item::GitInit),
    Row::Item(Item::Editor),
    Row::Gap,
    Row::Item(Item::Reset),
];

fn items() -> Vec<usize> {
    ROWS.iter()
        .enumerate()
        .filter(|(_, r)| matches!(r, Row::Item(_)))
        .map(|(i, _)| i)
        .collect()
}

fn item_at(row: usize) -> Item {
    match &ROWS[row] {
        Row::Item(i) => *i,
        _ => unreachable!("cursor stands only on Row::Item"),
    }
}

enum Action {
    None,
    Redraw,
    Open(Item),
    Back,
}

pub fn run(cfg: &mut Config) -> io::Result<()> {
    let items = items();
    let mut cursor = 0usize;
    let truecolor = crate::ui::logo::supports_truecolor();

    loop {
        let action = {
            let _guard = TerminalGuard::new()?;
            let mut out = io::stdout();
            let mut dirty = true;

            loop {
                if dirty {
                    render::draw(&mut out, cfg, items[cursor], truecolor)?;
                    dirty = false;
                }

                let Event::Key(key) = event::read()? else { continue };
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match handle(key, cfg, &items, &mut cursor) {
                    Action::None => {}
                    Action::Redraw => dirty = true,
                    other => break other,
                }
            }
        };

        match action {
            Action::Open(item) => actions::open(item, cfg)?,
            Action::Back => return Ok(()),
            _ => {}
        }
    }
}

fn handle(key: KeyEvent, cfg: &mut Config, items: &[usize], cursor: &mut usize) -> Action {
    let item = item_at(items[*cursor]);

    match key.code {
        KeyCode::Up | KeyCode::Char('k') => {
            *cursor = cursor.checked_sub(1).unwrap_or(items.len() - 1);
            Action::Redraw
        }
        KeyCode::Down | KeyCode::Char('j') => {
            *cursor = (*cursor + 1) % items.len();
            Action::Redraw
        }
        KeyCode::Left | KeyCode::Right | KeyCode::Char('h') | KeyCode::Char('l')
            if item.is_toggle() =>
        {
            toggle(item, cfg);
            let _ = cfg.save();
            Action::Redraw
        }
        KeyCode::Enter | KeyCode::Char(' ') if item.is_toggle() => {
            toggle(item, cfg);
            let _ = cfg.save();
            Action::Redraw
        }
        KeyCode::Enter | KeyCode::Char(' ') => Action::Open(item),
        KeyCode::Esc | KeyCode::Char('q') => Action::Back,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Back,
        _ => Action::None,
    }
}

fn toggle(item: Item, cfg: &mut Config) {
    match item {
        Item::CheckOnStart => cfg.check_updates = !cfg.check_updates,
        Item::GitInit => cfg.git_init = !cfg.git_init,
        Item::DefaultPm => cfg.default_manager = next_manager(&cfg.default_manager),
        _ => {}
    }
}

fn next_manager(current: &str) -> String {
    let ids: Vec<&str> = crate::toolchain::TOOLS
        .iter()
        .filter(|t| t.is_manager)
        .map(|t| t.id)
        .collect();
    let i = ids.iter().position(|id| *id == current).unwrap_or(0);
    ids[(i + 1) % ids.len()].to_string()
}

pub fn value(item: Item, cfg: &Config) -> String {
    match item {
        Item::CheckOnStart => on_off(cfg.check_updates).into(),
        Item::CheckNow => format!("v{}", crate::update::CURRENT),
        Item::ProjectsDir => cfg
            .projects_dir
            .as_ref()
            .map(|p| shorten(&p.display().to_string()))
            .unwrap_or_else(|| "current directory".into()),
        Item::DefaultPm => cfg.default_manager.clone(),
        Item::GitInit => on_off(cfg.git_init).into(),
        Item::Editor => cfg.editor.clone().unwrap_or_else(|| "off".into()),
        Item::Reset => String::new(),
    }
}

fn on_off(b: bool) -> &'static str {
    if b { "on" } else { "off" }
}

pub fn shorten(path: &str) -> String {
    match dirs::home_dir() {
        Some(home) => path.replacen(&home.display().to_string(), "~", 1),
        None => path.to_string(),
    }
}