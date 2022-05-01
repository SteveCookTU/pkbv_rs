use std::fmt::{Display, Formatter};

#[repr(u8)]
pub enum BattleMode {
    Link,
    Maison,
    MaisonSuper,
    BattleSpotFree,
    BattleSpotRating,
    BattleSpotSpecial,
    UNUSED,
    JP1,
    JP2,
    BAD,
}

impl Display for BattleMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BattleMode::Link => write!(f, "Link"),
            BattleMode::Maison => write!(f, "Maison"),
            BattleMode::MaisonSuper => write!(f, "Maison Super"),
            BattleMode::BattleSpotFree => write!(f, "Battle Spot Free"),
            BattleMode::BattleSpotRating => write!(f, "Battle Spot Rating"),
            BattleMode::BattleSpotSpecial => write!(f, "Battle Spot Special"),
            BattleMode::UNUSED => write!(f, "UNUSED"),
            BattleMode::JP1 => write!(f, "JP1"),
            BattleMode::JP2 => write!(f, "JP2"),
            BattleMode::BAD => write!(f, "BAD"),
        }
    }
}

impl From<u8> for BattleMode {
    fn from(val: u8) -> Self {
        match val {
            0 => BattleMode::Link,
            1 => BattleMode::Maison,
            2 => BattleMode::MaisonSuper,
            3 => BattleMode::BattleSpotFree,
            4 => BattleMode::BattleSpotRating,
            5 => BattleMode::BattleSpotSpecial,
            6 => BattleMode::UNUSED,
            7 => BattleMode::JP1,
            8 => BattleMode::JP2,
            _ => BattleMode::BAD,
        }
    }
}
