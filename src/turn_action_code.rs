use std::fmt::{Display, Formatter};

#[repr(u8)]
pub enum TurnActionCode {
    None = 0,
    Fight = 1,
    Switch = 3,
    Run = 4,
    UNK5 = 5,
    Rotate = 6,
    UNK7 = 7,
    MegaEvolve = 8,
}

impl From<u8> for TurnActionCode {
    fn from(val: u8) -> Self {
        match val {
            0 => TurnActionCode::None,
            1 => TurnActionCode::Fight,
            3 => TurnActionCode::Switch,
            4 => TurnActionCode::Run,
            5 => TurnActionCode::UNK5,
            6 => TurnActionCode::Rotate,
            7 => TurnActionCode::UNK7,
            _ => TurnActionCode::MegaEvolve,
        }
    }
}

impl Display for TurnActionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TurnActionCode::None => write!(f, "None"),
            TurnActionCode::Fight => write!(f, "Fight"),
            TurnActionCode::Switch => write!(f, "Switch"),
            TurnActionCode::Run => write!(f, "Run"),
            TurnActionCode::UNK5 => write!(f, "UNK5"),
            TurnActionCode::Rotate => write!(f, "Rotate"),
            TurnActionCode::UNK7 => write!(f, "UNK7"),
            TurnActionCode::MegaEvolve => write!(f, "MegaEvolve"),
        }
    }
}
