use crate::{Instruction, Cell};
use std::fmt::Debug;

pub mod token;
pub use token::Token;

mod mode;
pub use mode::ParserMode;

mod lexer;
pub(crate) use lexer::Lexer;

pub(crate) trait Processor<T> {
    fn process(code: T) -> T;
}

pub(crate) trait Postprocessor<T: Cell>: Debug{
    fn process(&self, code: Vec<Token<T>>) -> Vec<Token<T>>;
}
mod postprocessor;

#[derive(Debug)]
pub(crate) struct Parser<T: Cell> {
    mode: ParserMode,
    lexer: Lexer<T>,
    postprocessor: Vec<Box<dyn Postprocessor<T>>>
}

impl<T: Cell> Parser<T> {

    pub fn new(mode: ParserMode) -> Self {
        let lexer = Lexer::new(mode.keep_comments());
        let mut pipe = Parser {
            mode,
            lexer,
            postprocessor: Vec::new(),
        };

        pipe.setup();

        pipe
    }

    fn setup(&mut self) {
        if self.mode.aggregate_instructions() {
            self.postprocessor.push(Box::new(postprocessor::AggregateSameProcessor::create()))
        }
    }

    pub fn parse_script(&mut self, script: &str) -> Vec<Instruction<T>> {
        let mut tokens = self.lexer.lex_string(script);

        for i in 0..self.postprocessor.len() {
            tokens = self.postprocessor.get(i).unwrap().process(tokens);
        }

        self.lexer.flatten(tokens)
    }

}