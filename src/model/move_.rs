use crate::model::position::Position;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Move {
    pub(crate) start: Position,
    pub(crate) end: Position,
}

impl fmt::Display for Move {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&self.start.to_string())?;
        fmt.write_str("->")?;
        fmt.write_str(&self.end.to_string())?;
        Ok(())
    }
}

impl Move {
    pub(crate) fn kingside_castlep(&self) -> bool {
        return (self.start == Position { x: 4, y: 0 } && self.end == Position { x: 6, y: 0 })
            || (self.start == Position { x: 4, y: 7 } && self.end == Position { x: 6, y: 7 });
    }

    pub(crate) fn queenside_castlep(&self) -> bool {
        return (self.start == Position { x: 4, y: 0 } && self.end == Position { x: 2, y: 0 })
            || (self.start == Position { x: 4, y: 7 } && self.end == Position { x: 2, y: 7 });
    }
}
