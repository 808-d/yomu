use crate::epub::epub::load;
use clap::Parser;
use crossterm::event::{Event, KeyCode};
use indexmap::IndexMap;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use ratatui_image::StatefulImage;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use std::collections::HashMap;
use std::path::Path;
mod app;
mod epub;
#[derive(Parser)]
struct Cli {
    file: String,
}

enum Focus {
    Toc,
    Content,
}
struct AppState {
    title: String,
    epub_file: IndexMap<String, Vec<String>>,
    keys: Vec<String>,
    selected_index: usize,
    scroll: u16,
    list_state: ListState,
    focus: Focus,
    images: HashMap<String, StatefulProtocol>,
}
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let path = Path::new(
        "/home/duc/Documents/epubs/Too Many Losing Heroines! Volum - Takibi Amamori (Yu Sen takibi).epub",
    );
    let epub_file = load(path);
    let epub_file_content = epub_file.content;
    let keys: Vec<String> = epub_file_content.keys().cloned().collect();
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());

    let images: HashMap<String, StatefulProtocol> = epub_file
        .images
        .into_iter()
        .filter_map(|(name, bytes)| {
            let dyn_img = image::load_from_memory(&bytes).ok()?;
            // resize large images before converting
            let dyn_img = dyn_img.resize(800, 600, image::imageops::FilterType::Nearest);
            let proto = picker.new_resize_protocol(dyn_img);
            Some((name, proto))
        })
    .collect();
    let mut state = AppState {
        title: epub_file.title,
        epub_file: epub_file_content,
        keys,
        selected_index: 0,
        scroll: 0,
        list_state,
        focus: Focus::Toc,
        images,
    };
    loop {
        terminal.draw(|f| render(f, &mut state))?;
        if let Event::Key(key) = crossterm::event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Tab => {
                    state.focus = match state.focus {
                        Focus::Toc => Focus::Content,
                        Focus::Content => Focus::Toc,
                    };
                }
                KeyCode::Down | KeyCode::Char('j') => match state.focus {
                    Focus::Toc => {
                        let max = state.keys.len().saturating_sub(1);
                        if state.selected_index < max {
                            state.selected_index += 1;
                            state.list_state.select(Some(state.selected_index));
                            state.scroll = 0;
                        }
                    }
                    Focus::Content => {
                        state.scroll = state.scroll.saturating_add(1);
                    }
                },
                KeyCode::Up | KeyCode::Char('k') => match state.focus {
                    Focus::Toc => {
                        if state.selected_index > 0 {
                            state.selected_index -= 1;
                            state.list_state.select(Some(state.selected_index));
                            state.scroll = 0;
                        }
                    }
                    Focus::Content => {
                        state.scroll = state.scroll.saturating_sub(1);
                    }
                },
                _ => {}
            }
        }
    }
    Ok(())
}
fn render(frame: &mut Frame, state: &mut AppState) {
    let toc_items: Vec<ListItem> = state
        .keys
        .iter()
        .map(|k| ListItem::new(k.as_str()))
        .collect();
    let toc_block = Block::new()
        .borders(Borders::ALL)
        .title("Table of content")
        .border_style(match state.focus {
            Focus::Toc => Style::default().fg(Color::Yellow),
            Focus::Content => Style::default(),
        });
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(15), Constraint::Percentage(85)])
        .split(frame.area());

    frame.render_stateful_widget(
        List::new(toc_items).block(toc_block).highlight_symbol("> "),
        horizontal[0],
        &mut state.list_state,
    );

    let content_block = Block::new()
        .borders(Borders::ALL)
        .title(state.title.clone())
        .border_style(match state.focus {
            Focus::Content => Style::default().fg(Color::Yellow),
            Focus::Toc => Style::default(),
        });

    let area = horizontal[1];
    let inner = content_block.inner(area);
    frame.render_widget(content_block, area);

    let chapter_lines = state
        .keys
        .get(state.selected_index)
        .and_then(|k| state.epub_file.get(k))
        .cloned()
        .unwrap_or_default();

    let mut y = inner.y;
    for line in chapter_lines.iter().skip(state.scroll as usize) {
        if y >= inner.y + inner.height {
            break;
        }
        if let Some(src) = line
            .strip_prefix("[image: ")
                .and_then(|s| s.strip_suffix("]"))
        {
            if let Some(proto) = state.images.get_mut(src) {
                let img_area = ratatui::layout::Rect::new(inner.x, y, inner.width, 20);
                frame.render_stateful_widget(StatefulImage::default(), img_area, proto);
                y += 20;
            }
        } else {
            let line_area = ratatui::layout::Rect::new(inner.x, y, inner.width, 1);
            frame.render_widget(
                Paragraph::new(line.as_str()).wrap(Wrap { trim: true }),
                line_area,
            );
            y += 1;
        }
    }
}
