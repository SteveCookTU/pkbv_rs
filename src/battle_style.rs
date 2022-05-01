use std::fmt::{Display, Formatter};

#[repr(u8)]
pub enum BattleStyle {
    Single,
    Double,
    Triple,
    Rotation,
    Multi,
}

impl Display for BattleStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BattleStyle::Single => write!(f, "Single"),
            BattleStyle::Double => write!(f, "Double"),
            BattleStyle::Triple => write!(f, "Triple"),
            BattleStyle::Rotation => write!(f, "Rotation"),
            BattleStyle::Multi => write!(f, "Multi"),
        }
    }
}

impl From<u8> for BattleStyle {
    fn from(val: u8) -> Self {
        match val {
            0 => BattleStyle::Single,
            1 => BattleStyle::Double,
            2 => BattleStyle::Triple,
            3 => BattleStyle::Rotation,
            _ => BattleStyle::Multi,
        }
    }
}
