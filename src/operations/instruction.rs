use crate::Cell;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction<T: Cell> {
    NoOp,
    IncreaseIndex(usize),
    DecreaseIndex(usize),
    IncreaseValue(T),
    DecreaseValue(T),
    IoRead,
    IoWrite,
    Loop(Vec<Instruction<T>>),
}

impl<T> Instruction<T>
    where
        T: Cell
{

    pub fn can_join(&self, other: &Instruction<T>) -> bool {
        match (self, other) {
            (Instruction::IncreaseIndex(_), Instruction::IncreaseIndex(_)) |
            (Instruction::DecreaseIndex(_), Instruction::DecreaseIndex(_)) |
            (Instruction::IncreaseValue(_), Instruction::IncreaseValue(_)) |
            (Instruction::DecreaseValue(_), Instruction::DecreaseValue(_)) => {
                true
            },
            _ => false
        }
    }

    pub fn join(self, other: Instruction<T>) -> Instruction<T> {
        match (self, other) {
            (Instruction::IncreaseIndex(x), Instruction::IncreaseIndex(y)) => Instruction::IncreaseIndex(x + y),
            (Instruction::DecreaseIndex(x), Instruction::DecreaseIndex(y)) => Instruction::DecreaseIndex(x + y),
            (Instruction::IncreaseValue(x)   , Instruction::IncreaseValue(y  )) => Instruction::IncreaseValue(x + y),
            (Instruction::DecreaseValue(x)   , Instruction::DecreaseValue(y  )) => Instruction::DecreaseValue(x + y),
            (this, _) => this
        }
    }

    pub fn is_loop(&self) -> bool {
        match self {
            Instruction::Loop(_) => true,
            _ => false
        }
    }
}

