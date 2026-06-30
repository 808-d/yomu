use crate::app::App;
use clap::Parser;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use std::collections::HashMap;
use std::path::Path;
mod app;
mod epub;
mod keys;
use keys::Key;
#[derive(Parser)]
struct Cli {
    file: String,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let path = Path::new("placeholder");
    let mut result = crate::epub::epub::load(path);
    loop {
        terminal.draw(|f| render(f, &mut result))?;
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == crossterm::event::KeyCode::Char(Key::Quit.char()) {
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame, epub_file: &mut HashMap<String, String>) {
    let toc_items: Vec<ListItem> = epub_file
        .keys()
        .map(|k| ListItem::new(k.as_str()))
        .collect();

    let toc_block = Block::new().borders(Borders::ALL).title("Table of content");

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(vertical[1]);

    frame.render_widget(
        Paragraph::new("Title").block(Block::new().borders(Borders::ALL)),
        vertical[0],
    );
    frame.render_widget(
        List::new(toc_items).block(toc_block).highlight_symbol("> "),
        horizontal[0],
    );
    frame.render_widget(
        Paragraph::new("Content").block(Block::new().borders(Borders::ALL)),
        horizontal[1],
    );
}
