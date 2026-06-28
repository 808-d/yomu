use crate::app::App;
use clap::Parser;
use ratatui::widgets::Paragraph;
use ratatui::{DefaultTerminal, Frame};
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
    loop {
        terminal.draw(render)?;
        if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
            if key.code == crossterm::event::KeyCode::Char(Key::Quit.char()) {
                break Ok(());
            }
        }
    }
}

fn render(frame: &mut Frame) {
    let path = Path::new("/home/duc/Documents/epubs/また同じ夢を見ていた - 住野よる.epub");
    let result = crate::epub::epub::load(path);

    let debug_text = format!("{:#?}", result);
    frame.render_widget(Paragraph::new(debug_text), frame.area());
}
