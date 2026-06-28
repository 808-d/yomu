pub enum Key {
    // motion
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Top,
    Bottom,
    // actions
    Quit,
    Open,
    Back,
}

impl Key {
    pub fn char(&self) -> char {
        match self {
            Key::Up       => 'k',
            Key::Down     => 'j',
            Key::Left     => 'h',
            Key::Right    => 'l',
            Key::PageUp   => 'u',
            Key::PageDown => 'd',
            Key::Top      => 'g',
            Key::Bottom   => 'G',
            Key::Quit     => 'q',
            Key::Open     => '\n',
            Key::Back     => 'b',
        }
    }

    pub fn from_char(c: char) -> Option<Key> {
        match c {
            'k'  => Some(Key::Up),
            'j'  => Some(Key::Down),
            'h'  => Some(Key::Left),
            'l'  => Some(Key::Right),
            'u'  => Some(Key::PageUp),
            'd'  => Some(Key::PageDown),
            'g'  => Some(Key::Top),
            'G'  => Some(Key::Bottom),
            'q'  => Some(Key::Quit),
            '\n' => Some(Key::Open),
            'b'  => Some(Key::Back),
            _    => None,
        }
    }
}
