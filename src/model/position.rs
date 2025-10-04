#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct Position {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl ToString for Position {
    fn to_string(&self) -> String {
        match self.x {
            0 => return "A".to_owned() + &(self.y + 1).to_string(),
            1 => return "B".to_owned() + &(self.y + 1).to_string(),
            2 => return "C".to_owned() + &(self.y + 1).to_string(),
            3 => return "D".to_owned() + &(self.y + 1).to_string(),
            4 => return "E".to_owned() + &(self.y + 1).to_string(),
            5 => return "F".to_owned() + &(self.y + 1).to_string(),
            6 => return "G".to_owned() + &(self.y + 1).to_string(),
            7 => return "H".to_owned() + &(self.y + 1).to_string(),
            _ => return String::from(""),
        }
    }
}

impl Position {
    pub(crate) fn up(&self) -> Position {
        return Position {
            x: self.x,
            y: self.y + 1,
        };
    }

    pub(crate) fn down(&self) -> Position {
        return Position {
            x: self.x,
            y: self.y - 1,
        };
    }

    pub(crate) fn left(&self) -> Position {
        return Position {
            x: self.x - 1,
            y: self.y,
        };
    }

    pub(crate) fn right(&self) -> Position {
        return Position {
            x: self.x + 1,
            y: self.y,
        };
    }

    pub(crate) fn validp(&self) -> bool {
        return self.x >= 0 && self.x <= 7 && self.y >= 0 && self.y <= 7;
    }

    pub(crate) fn from_string(str: String) -> Position {
        let x: isize;
        match str.to_lowercase().chars().nth(0).unwrap() {
            'a' => x = 0,
            'b' => x = 1,
            'c' => x = 2,
            'd' => x = 3,
            'e' => x = 4,
            'f' => x = 5,
            'g' => x = 6,
            'h' => x = 7,
            _ => x = 0,
        }

        let y: isize = (str.chars().nth(1).unwrap().to_digit(10).unwrap() - 1)
            .try_into()
            .unwrap();

        Position { x: x, y: y }
    }
}
