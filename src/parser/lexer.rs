use crate::{Cell, Instruction};
use std::str::Chars;

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

pub(crate) enum LexerItem<T: Cell> {
    Instruction(Instruction<T>),
    LoopBlock(Vec<LexerItem<T>>)
}

pub(crate) struct Lexer<T: Cell> {
    instructions: Vec<LexerItem<T>>,
    unknown_to_noop: bool
}

impl<T> Lexer<T>
    where T: Cell
{
    pub fn new(unknown_to_noop: bool) -> Self {
        Lexer {
            instructions: Vec::new(),
            unknown_to_noop
        }
    }

    pub fn lex_string(&mut self, code: &str) {
        let code = code.to_string();
        self.instructions = self.lex_block(&mut code.chars(), char::default())
    }

    pub fn lex_block(&mut self, code: &mut Chars, end: char) -> Vec<LexerItem<T>> {
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
                OP_INC_INDEX     => {Some(LexerItem::Instruction(Instruction::IncreaseIndex(1)))},
                OP_DEC_INDEX     => {Some(LexerItem::Instruction(Instruction::DecreaseIndex(1)))},
                OP_INC_VALUE     => {Some(LexerItem::Instruction(Instruction::IncreaseValue(T::from(1u8))))},
                OP_DEC_VALUE     => {Some(LexerItem::Instruction(Instruction::DecreaseValue(T::from(1u8))))},
                OP_IO_READ       => {Some(LexerItem::Instruction(Instruction::IoRead))},
                OP_IO_WRITE      => {Some(LexerItem::Instruction(Instruction::IoWrite))},
                OP_LOOP_START    => {
                    Some(LexerItem::LoopBlock(self.lex_block(code, OP_LOOP_END)))
                },
                OP_FILE_IO_READ  => {Some(LexerItem::Instruction(Instruction::FileIoRead))},
                OP_FILE_IO_WRITE => {Some(LexerItem::Instruction(Instruction::FileIoWrite))},
                OP_NOOP          => {Some(LexerItem::Instruction(Instruction::NoOp))}
                _ => {
                    if self.unknown_to_noop {
                        Some(LexerItem::Instruction(Instruction::NoOp))
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

    fn flatten(items: Vec<LexerItem<T>>) -> Vec<Instruction<T>> {
        let mut instructions = Vec::new();

        for item in items {
            match item {
                LexerItem::Instruction(x) => {instructions.push(x)},
                LexerItem::LoopBlock(block) => {
                    let mut block = Self::flatten(block);
                    let block_size = block.len() + 1;

                    instructions.push(Instruction::LoopStart(block_size));
                    instructions.append(&mut block);
                    instructions.push(Instruction::LoopEnd(block_size));
                },
            }
        }

        instructions
    }

    pub fn finish(self) -> Vec<Instruction<T>> {
        Self::flatten(self.instructions)
    }
}

impl<T> Default for Lexer<T>
    where T: Cell
{
    fn default() -> Self {
        Lexer::new(false)
    }
}
