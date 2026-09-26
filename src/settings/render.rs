use std::io::{self, Write};
use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor, Stylize},
    terminal::{self, Clear, ClearType},
};
use crate::{
    config::{self, Config},
    settings::{value, Item, Row, ROWS},
};

const INNER: usize = 52;
const ACCENT: Color = Color::Rgb { r: 0, g: 212, b: 255 };

pub fn draw(out: &mut impl Write, cfg: &Config, active_row: usize, truecolor: bool) -> io::Result<()> {
    let accent = if truecolor { ACCENT } else { Color::Cyan };
    let (term_w, term_h) = terminal::size()?;
    let x = term_w.saturating_sub(INNER as u16 + 2) / 2;

    let height = ROWS.len() as u16 + 7;
    let mut y = term_h.saturating_sub(height) / 2;

    queue!(out, Clear(ClearType::All))?;

    let title = " Settings ";
    let rest = INNER - title.chars().count() - 1;
    queue!(out, MoveTo(x, y), Print("╭─".dark_grey()))?;
    queue!(out, SetForegroundColor(accent), Print(title), ResetColor)?;
    queue!(out, Print("─".repeat(rest).dark_grey()), Print("╮".dark_grey()))?;
    y += 1;

    blank(out, x, y)?;
    y += 1;

    for (i, row) in ROWS.iter().enumerate() {
        queue!(out, MoveTo(x, y), Print("│".dark_grey()))?;
        match row {
            Row::Gap => queue!(out, Print(" ".repeat(INNER)))?,
            Row::Section(name) => {
                queue!(out, Print("  "), Print(format!("{name:<width$}", width = INNER - 2).dim()))?;
            }
            Row::Item(item) => draw_item(out, cfg, *item, i == active_row, accent)?,
        }
        queue!(out, Print("│".dark_grey()))?;
        y += 1;
    }

    blank(out, x, y)?;
    y += 1;

    queue!(out, MoveTo(x, y), Print(format!("├{}┤", "─".repeat(INNER)).dark_grey()))?;
    y += 1;
    let path = crate::settings::shorten(&config::path().display().to_string());
    queue!(
        out,
        MoveTo(x, y),
        Print("│".dark_grey()),
        Print(format!("  {path:<width$}", width = INNER - 2).dim()),
        Print("│".dark_grey())
    )?;
    y += 1;
    queue!(out, MoveTo(x, y), Print(format!("╰{}╯", "─".repeat(INNER)).dark_grey()))?;

    let hints = "↑↓ move · ←→ change · enter open · esc back";
    queue!(
        out,
        MoveTo(term_w.saturating_sub(hints.chars().count() as u16) / 2, y + 2),
        Print(hints.dark_grey())
    )?;

    out.flush()
}

fn draw_item(
    out: &mut impl Write,
    cfg: &Config,
    item: Item,
    active: bool,
    accent: Color,
) -> io::Result<()> {
    let value = value(item, cfg);
    let value = if item.is_toggle() && active {
        format!("‹ {value} ›")
    } else {
        value
    };

    let cursor = if active { "▸" } else { " " };
    let label = item.label();
    let gap = INNER
        .saturating_sub(6 + label.chars().count() + value.chars().count())
        .max(1);

    queue!(out, Print("  "))?;
    if active {
        queue!(out, SetForegroundColor(accent), Print(cursor), Print(" "), ResetColor)?;
        queue!(out, Print(label.bold()))?;
    } else {
        queue!(out, Print(cursor), Print(" "), Print(label))?;
    }
    queue!(out, Print(" ".repeat(gap)))?;
    if active {
        queue!(out, SetForegroundColor(accent), Print(&value), ResetColor)?;
    } else {
        queue!(out, Print(value.as_str().dim()))?;
    }
    queue!(out, Print("  "))
}

fn blank(out: &mut impl Write, x: u16, y: u16) -> io::Result<()> {
    queue!(
        out,
        MoveTo(x, y),
        Print("│".dark_grey()),
        Print(" ".repeat(INNER)),
        Print("│".dark_grey())
    )
}