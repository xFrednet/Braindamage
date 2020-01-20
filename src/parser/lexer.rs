use crate::{Cell, Instruction};
use std::str::Chars;
use crate::parser::Token;
use std::marker::PhantomData;

const OP_INC_INDEX: char = '>';
const OP_DEC_INDEX: char = '<';
const OP_INC_VALUE: char = '+';
const OP_DEC_VALUE: char = '-';
const OP_IO_READ: char = ',';
const OP_IO_WRITE: char = '.';
const OP_LOOP_START: char = '[';
const OP_LOOP_END: char = ']';
const OP_FILE_IO_READ: char = ';';
const OP_FILE_IO_WRITE: char = ':';
const OP_NOOP: char = '_';

#[derive(Debug)]
pub(crate) struct Lexer<T: Cell> {
    unknown_to_noop: bool,
    phantom: PhantomData<T>,
}

impl<T> Lexer<T>
    where T: Cell
{
    pub fn new(unknown_to_noop: bool) -> Self {
        Lexer {
            unknown_to_noop,
            phantom: PhantomData
        }
    }

    pub fn lex_string(&mut self, code: &str) -> Vec<Token<T>> {
        let code = code.to_string();
        self.lex_block(&mut code.chars(), char::default())
    }

    fn lex_block(&mut self, code: &mut Chars, end: char) -> Vec<Token<T>> {
        let mut block = Vec::new();

        loop {
            let inst = code.next();
            if inst.is_none() {
                return block;
            }

            let inst = inst.unwrap();
            if inst == end {
                return block;
            }

            let lexed_inst = match inst {
                OP_INC_INDEX     => {Some(Token::Instruction(Instruction::IncreaseIndex(1)))},
                OP_DEC_INDEX     => {Some(Token::Instruction(Instruction::DecreaseIndex(1)))},
                OP_INC_VALUE     => {Some(Token::Instruction(Instruction::IncreaseValue(T::from(1u8))))},
                OP_DEC_VALUE     => {Some(Token::Instruction(Instruction::DecreaseValue(T::from(1u8))))},
                OP_IO_READ       => {Some(Token::Instruction(Instruction::IoRead))},
                OP_IO_WRITE      => {Some(Token::Instruction(Instruction::IoWrite))},
                OP_LOOP_START    => {
                    Some(Token::LoopBlock(self.lex_block(code, OP_LOOP_END)))
                },
                OP_FILE_IO_READ  => {Some(Token::Instruction(Instruction::FileIoRead))},
                OP_FILE_IO_WRITE => {Some(Token::Instruction(Instruction::FileIoWrite))},
                OP_NOOP          => {Some(Token::Instruction(Instruction::NoOp))}
                _ => {
                    if self.unknown_to_noop {
                        Some(Token::Instruction(Instruction::NoOp))
                    } else {
                        None
                    }
                }
            };

            match lexed_inst {
                None => {},
                Some(x) => {
                    block.push(x);
                },
            }
        }
    }

    pub fn flatten(&self, items: Vec<Token<T>>) -> Vec<Instruction<T>> {
        let mut instructions = Vec::new();

        for item in items {
            match item {
                Token::Instruction(x) => {instructions.push(x)},
                Token::LoopBlock(block) => {
                    let mut block = self.flatten(block);
                    let block_size = block.len() + 1;

                    instructions.push(Instruction::LoopStart(block_size));
                    instructions.append(&mut block);
                    instructions.push(Instruction::LoopEnd(block_size));
                },
            }
        }

        instructions
    }
}

impl<T> Default for Lexer<T>
    where T: Cell
{
    fn default() -> Self {
        Lexer::new(false)
    }
}
