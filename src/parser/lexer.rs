use crate::{Cell, Instruction};

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

pub(crate) struct Lexer<T: Cell> {
    instructions: Vec<Instruction<T>>,
    loop_stack: Vec<usize>,
    unknown_to_noop: bool
}

impl<T> Lexer<T>
    where T: Cell
{
    pub fn new(unknown_to_noop: bool) -> Self {
        Lexer {
            instructions: Vec::new(),
            loop_stack: Vec::new(),
            unknown_to_noop
        }
    }

    pub fn lex_string(&mut self, code: &str) {
        let mut index = 0;
        for inst in code.chars() {
            match self.lex_char(inst, index) {
                None => {},
                Some(x) => {
                    self.instructions.push(x);
                    index += 1;
                },
            }
        }
    }

    pub fn lex_char(&mut self, inst: char, index: usize) -> Option<Instruction<T>> {
        match inst {
            OP_INC_INDEX     => {Some(Instruction::IncreaseIndex(1))},
            OP_DEC_INDEX     => {Some(Instruction::DecreaseIndex(1))},
            OP_INC_VALUE     => {Some(Instruction::IncreaseValue(T::from(1u8)))},
            OP_DEC_VALUE     => {Some(Instruction::DecreaseValue(T::from(1u8)))},
            OP_IO_READ       => {Some(Instruction::IoRead)},
            OP_IO_WRITE      => {Some(Instruction::IoWrite)},

            OP_LOOP_START    => {
                self.loop_stack.push(index);
                Some(Instruction::LoopStart{loop_size: 0, size_set: false})
            },
            OP_LOOP_END      => {
                let start = match self.loop_stack.pop() {
                    Some(start) => {
                        match self.instructions.get_mut(start).unwrap() {
                            Instruction::LoopStart { ref mut loop_size, ref mut size_set} => {
                                *loop_size = index - start;
                                *size_set = true;
                            },
                            _ => {},
                        };

                        start
                    }
                    None => {
                        println!("The Closing bracket at index {} has no partner.", index);
                        0
                    },
                };

                Some(Instruction::LoopEnd {loop_size: index - start})
            },
            OP_FILE_IO_READ  => {Some(Instruction::FileIoRead)},
            OP_FILE_IO_WRITE => {Some(Instruction::FileIoWrite)},
            OP_NOOP          => {Some(Instruction::NoOp)}
            _ => {
                if self.unknown_to_noop {
                    Some(Instruction::NoOp)
                } else {
                    None
                }
            }
        }
    }

    pub fn finish(self) -> Vec<Instruction<T>> {
        for index in self.loop_stack {
            println!("The block starting at {} was not closed.", index);
        }

        self.instructions
    }
}

impl<T> Default for Lexer<T>
    where T: Cell
{
    fn default() -> Self {
        Lexer::new(false)
    }
}
