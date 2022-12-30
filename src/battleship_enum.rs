use core::fmt;

#[derive(Copy, Clone)]
pub enum BoardPart {
    EMPTY,
    UNKNOWN,
    SHIP,
    HIT
}

pub enum ShipOrientation {
    VERTICAL,
    HORIZONTAL
}

impl fmt::Display for BoardPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoardPart::EMPTY => write!(f, " "),
            BoardPart::UNKNOWN => write!(f, "?"),
            BoardPart::SHIP => write!(f, "S"),
            BoardPart::HIT => write!(f, "X"),
        }
    }
}
