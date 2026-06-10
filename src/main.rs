use crate::app::App;
use crate::common::common::File;
use crate::epub::Epub;
use clap::Parser;
use crossterm::terminal;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::path::Path;
mod app;
pub mod common;
mod epub;

#[derive(Parser)]
struct Cli {
    file: String,
}

fn main() {
    let path = Path::new(
        "/home/duc/Documents/クールな女神様と一緒に住んだら、甘やかしすぎてポンコツにしてしまった件について1 (HJ文庫).epub",
    );

    let epub = Epub::default();
    let result = epub.unzip(path);
    println!("exists: {}", path.exists());
    println!("result: {:#?}", result);
}
