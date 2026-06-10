#[derive(Debug, Default)]
pub struct App {
    pub is_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.is_quit = true;
    }
}
