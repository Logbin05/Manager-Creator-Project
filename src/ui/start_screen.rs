use super::{guard::TerminalGuard, logo};
use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    queue,
    style::{Print, Stylize},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuItem {
    NewProject,
    FromTemplate,
    Recent,
    Settings,
    Quit,
}

const MENU: &[(char, &str, MenuItem)] = &[
    ('n', "New project", MenuItem::NewProject),
    ('t', "From template", MenuItem::FromTemplate),
    ('r', "Recent projects", MenuItem::Recent),
    ('s', "Settings", MenuItem::Settings),
    ('q', "Exit", MenuItem::Quit),
];

const BOX_INNER: usize = 32;
const MENU_H: u16 = MENU.len() as u16 + 3;
const HINTS: &[(&str, &str)] = &[("↑↓", "navigate"), ("enter", "select"), ("q", "quit")];

const BELOW_LOGO: u16 = 1 + 1 + 1 + MENU_H + 1 + 1;

#[derive(Default)]
struct Menu {
    selected: usize,
}

enum Outcome {
    Continue,
    Choose(MenuItem),
}

impl Menu {
    fn handle(&mut self, key: KeyEvent) -> Outcome {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.selected = self.selected.checked_sub(1).unwrap_or(MENU.len() - 1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.selected = (self.selected + 1) % MENU.len();
            }
            KeyCode::Enter => return Outcome::Choose(MENU[self.selected].2),
            KeyCode::Esc => return Outcome::Choose(MenuItem::Quit),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                return Outcome::Choose(MenuItem::Quit);
            }
            KeyCode::Char(c) => {
                if let Some(i) = MENU.iter().position(|(k, ..)| *k == c) {
                    return Outcome::Choose(MENU[i].2);
                }
            }
            _ => {}
        }
        Outcome::Continue
    }
}

fn render(out: &mut impl Write, menu: &Menu, truecolor: bool) -> io::Result<()> {
    let (term_w, term_h) = terminal::size()?;
    let logo = logo::pick(
        term_w.saturating_sub(4) as usize,
        term_h.saturating_sub(BELOW_LOGO) as usize,
    );

    let total_h = logo.height() as u16 + BELOW_LOGO;
    let mut y = term_h.saturating_sub(total_h) / 2;

    queue!(out, terminal::Clear(ClearType::All))?;

    logo.draw(out, center(term_w, logo.width()), y, truecolor)?;
    y += logo.height() as u16 + 1;

    let subtitle = format!("project generator · v{}", env!("CARGO_PKG_VERSION"));
    queue!(
        out,
        MoveTo(center(term_w, subtitle.chars().count()), y),
        Print(subtitle.dim())
    )?;
    y += 2;

    y = draw_menu(out, menu, center(term_w, BOX_INNER + 2), y)?;
    draw_hints(out, term_w, y + 1)?;

    out.flush()
}

fn draw_menu(out: &mut impl Write, menu: &Menu, x: u16, mut y: u16) -> io::Result<u16> {
    let line = "─".repeat(BOX_INNER);
    queue!(out, MoveTo(x, y), Print(format!("╭{line}╮").dark_grey()))?;
    y += 1;

    for (i, &(key, label, _)) in MENU.iter().enumerate() {
        if i == MENU.len() - 1 {
            queue!(out, MoveTo(x, y), Print(format!("├{line}┤").dark_grey()))?;
            y += 1;
        }

        let label = format!("{label:<23}");
        queue!(out, MoveTo(x, y), Print("│".dark_grey()))?;
        if i == menu.selected {
            queue!(out, Print("  ▸ ".cyan()), Print(label.bold()), Print(key.cyan().bold()))?;
        } else {
            queue!(out, Print("    "), Print(label), Print(key.dark_grey()))?;
        }
        queue!(out, Print("    "), Print("│".dark_grey()))?;
        y += 1;
    }

    queue!(out, MoveTo(x, y), Print(format!("╰{line}╯").dark_grey()))?;
    Ok(y + 1)
}

fn draw_hints(out: &mut impl Write, term_w: u16, y: u16) -> io::Result<()> {
    let width = HINTS
        .iter()
        .map(|(k, d)| k.chars().count() + 1 + d.chars().count())
        .sum::<usize>()
        + (HINTS.len() - 1) * 3;

    queue!(out, MoveTo(center(term_w, width), y))?;
    for (i, &(key, desc)) in HINTS.iter().enumerate() {
        if i > 0 {
            queue!(out, Print(" · ".dark_grey()))?;
        }
        queue!(out, Print(key.bold()), Print(format!(" {desc}").dark_grey()))?;
    }
    Ok(())
}

fn center(term_w: u16, content_w: usize) -> u16 {
    term_w.saturating_sub(content_w as u16) / 2
}

pub fn run() -> io::Result<MenuItem> {
    let _guard = TerminalGuard::new()?;
    let mut out = io::stdout();
    let mut menu = Menu::default();
    let truecolor = logo::supports_truecolor();

    loop {
        render(&mut out, &menu, truecolor)?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if let Outcome::Choose(item) = menu.handle(key) {
                return Ok(item);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn up_from_first_wraps_to_last() {
        let mut m = Menu::default();
        m.handle(KeyCode::Up.into());
        assert_eq!(m.selected, MENU.len() - 1);
    }

    #[test]
    fn hotkey_chooses_immediately() {
        let mut m = Menu::default();
        assert!(matches!(
            m.handle(KeyCode::Char('s').into()),
            Outcome::Choose(MenuItem::Settings)
        ));
    }
}