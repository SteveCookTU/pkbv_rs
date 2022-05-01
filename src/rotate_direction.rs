use std::fmt::{Display, Formatter};

pub enum RotateDirection {
    None,
    Right,
    Left,
}

impl From<u8> for RotateDirection {
    fn from(val: u8) -> Self {
        match val {
            0 => RotateDirection::None,
            1 => RotateDirection::Right,
            _ => RotateDirection::Left,
        }
    }
}

impl Display for RotateDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RotateDirection::None => write!(f, "None"),
            RotateDirection::Right => write!(f, "Right"),
            RotateDirection::Left => write!(f, "Left"),
        }
    }
}
