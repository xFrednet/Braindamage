use crate::{Cell, Instruction};

pub enum Token<T: Cell> {
    Instruction(Instruction<T>),
    LoopBlock(Vec<Token<T>>)
}

impl<T: Cell> Token<T> {
    pub fn can_join(&self, other: &Token<T>) -> bool {
        match (self, other) {
            (Token::Instruction(x), Token::Instruction(y))
            => x.can_join(y),
            _ => false
        }
    }

    pub fn join(self, other: Token<T>) -> Token<T> {
        match (self, other) {
            (Token::Instruction(x), Token::Instruction(y))
            => Token::Instruction(x.join(y)),
            _ => panic!("No Join here")
        }
    }
}