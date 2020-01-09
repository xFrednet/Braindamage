use crate::BufType;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    NoOp,
    IncreaseIndex(usize),
    DecreaseIndex(usize),
    IncreaseValue(BufType),
    DecreaseValue(BufType),
    IoRead,
    IoWrite,
    Loop(Vec<Operation>),
}

impl Operation {

    pub fn can_join(&self, other: &Operation) -> bool {
        match (self, other) {
            (Operation::IncreaseIndex(_), Operation::IncreaseIndex(_)) |
            (Operation::DecreaseIndex(_), Operation::DecreaseIndex(_)) |
            (Operation::IncreaseValue(_), Operation::IncreaseValue(_)) |
            (Operation::DecreaseValue(_), Operation::DecreaseValue(_)) => {
                true
            },
            _ => false
        }
    }

    pub fn join(self, other: &Operation) -> Operation {
        match self {
            Operation::IncreaseIndex(x) => {Operation::IncreaseIndex(x + match other {Operation::IncreaseIndex(x) => x.clone(), _ => 0})},
            Operation::DecreaseIndex(x) => {Operation::DecreaseIndex(x + match other {Operation::DecreaseIndex(x) => x.clone(), _ => 0})},
            Operation::IncreaseValue(x)   => {Operation::IncreaseValue(x + match other {Operation::IncreaseValue(x) => x.clone(), _ => 0})},
            Operation::DecreaseValue(x)   => {Operation::DecreaseValue(x + match other {Operation::DecreaseValue(x) => x.clone(), _ => 0})},
            x => x
        }
    }

    pub fn is_loop(&self) -> bool {
        match self {
            Operation::Loop(_) => true,
            _ => false
        }
    }
}

