use std::fmt::{Display, Formatter};

pub enum MoveTarget {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _Self,
    _8,
    _9,
    _10,
    _11,
    _12,
    Counter,
}

impl Display for MoveTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MoveTarget::_0 => write!(f, "0"),
            MoveTarget::_1 => write!(f, "1"),
            MoveTarget::_2 => write!(f, "2"),
            MoveTarget::_3 => write!(f, "3"),
            MoveTarget::_4 => write!(f, "4"),
            MoveTarget::_5 => write!(f, "5"),
            MoveTarget::_6 => write!(f, "6"),
            MoveTarget::_Self => write!(f, "Self"),
            MoveTarget::_8 => write!(f, "8"),
            MoveTarget::_9 => write!(f, "9"),
            MoveTarget::_10 => write!(f, "10"),
            MoveTarget::_11 => write!(f, "11"),
            MoveTarget::_12 => write!(f, "12"),
            MoveTarget::Counter => write!(f, "Counter"),
        }
    }
}

impl From<u8> for MoveTarget {
    fn from(val: u8) -> Self {
        match val {
            0 => MoveTarget::_0,
            1 => MoveTarget::_1,
            2 => MoveTarget::_2,
            3 => MoveTarget::_3,
            4 => MoveTarget::_4,
            5 => MoveTarget::_5,
            6 => MoveTarget::_6,
            7 => MoveTarget::_Self,
            8 => MoveTarget::_8,
            9 => MoveTarget::_9,
            10 => MoveTarget::_10,
            11 => MoveTarget::_11,
            12 => MoveTarget::_12,
            _ => MoveTarget::Counter,
        }
    }
}
