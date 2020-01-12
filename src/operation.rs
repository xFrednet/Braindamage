use crate::Cell;

//    +,-
//   {^.°}
//   <[@]>
//    : ;
//
// Meet BDB the BrainDamageBot he is here to make sure that
// all code is awesome and readable

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum Operation<T: Cell> {
    NoOp,
    IncreaseIndex(usize),
    DecreaseIndex(usize),
    IncreaseValue(T),
    DecreaseValue(T),
    IoRead,
    IoWrite,
    Loop(Vec<Operation<T>>),
}

impl<T> Operation<T>
    where
        T: Cell
{

    pub fn can_join(&self, other: &Operation<T>) -> bool {
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

    pub fn join(self, other: Operation<T>) -> Operation<T> {
        match (self, other) {
            (Operation::IncreaseIndex(x), Operation::IncreaseIndex(y)) => Operation::IncreaseIndex(x + y),
            (Operation::DecreaseIndex(x), Operation::DecreaseIndex(y)) => Operation::DecreaseIndex(x + y),
            (Operation::IncreaseValue(x)   , Operation::IncreaseValue(y  )) => Operation::IncreaseValue(x + y),
            (Operation::DecreaseValue(x)   , Operation::DecreaseValue(y  )) => Operation::DecreaseValue(x + y),
            (this, _) => this
        }
    }

    pub fn is_loop(&self) -> bool {
        match self {
            Operation::Loop(_) => true,
            _ => false
        }
    }
}

