#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub(crate) enum Color {
    White,
    Black,
    None,
}

impl Color {
    pub(crate) fn to_string(&self) -> String {
        return if *self == Color::None {
            "".to_owned()
        } else if *self == Color::White {
            "w".to_owned()
        } else {
            "b".to_owned()
        };
    }

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
