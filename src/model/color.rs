#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) enum Color {
    White,
    Black,
    None,
}

impl Color {
    pub(crate) fn opponent_color(&self) -> Color {
        if *self == Color::None {
            return Color::None;
        }
        return if *self == Color::White {
            Color::Black
        } else {
            Color::White
        };
    }
}
