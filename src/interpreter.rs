use crate::operations::io::BraindamageIo;

//    +,-
//   {^.°}
//   <[@]>
//    : ;

use crate::buffer::VecBuffer;
use crate::{Instruction, Cell};
use crate::operations::io::console_io::ConsoleIo;
use crate::operations::io::file_io::FileIo;
use std::num::Wrapping;

pub struct Interpreter<'a, T: Cell> {
    buffer: VecBuffer<T>,
    index: usize,

    instructions: &'a Vec<Instruction<T>>,

    console_io: Box<dyn BraindamageIo<T>>,
    file_io: Box<dyn BraindamageIo<T>>,
}

impl<'a, T> Interpreter<'a, T>
    where T: Cell
{
    const DEFAULT_FILE: &'static str = "bd_data.txt";

    pub fn new(instructions: &'a Vec<Instruction<T>>, buffer_size: usize) -> Self {
        Self::new_with_io(
            instructions,
            buffer_size,
            Box::new(ConsoleIo::new()),
            Box::new(FileIo::new(&Self::DEFAULT_FILE))
        )
    }

    pub fn new_with_io(
        instructions: &'a Vec<Instruction<T>>,
        buffer_size: usize,
        console_io: Box<dyn BraindamageIo<T>>,
        file_io: Box<dyn BraindamageIo<T>>) -> Self
    {
        Interpreter {
            buffer: VecBuffer::new(buffer_size),
            index: 0,

            instructions,

            console_io,
            file_io
        }
    }

    pub fn run(&mut self) {
        self.execute(self.instructions);
    }

    fn execute(&mut self, instructions: &Vec<Instruction<T>>) {
        let mut inst_ptr = 0;
        while inst_ptr < instructions.len() {
            let inst = instructions.get(inst_ptr).unwrap();

            match inst {
                Instruction::NoOp => {},
                Instruction::IncreaseIndex(x) => {
                    self.index = (Wrapping(self.index) + Wrapping(*x)).0 % self.buffer.size()
                },
                Instruction::DecreaseIndex(x) => {
                    self.index = (Wrapping(self.index) - Wrapping(*x)).0 % self.buffer.size()
                },
                Instruction::IncreaseValue(x) => {
                    self.buffer.set_value(
                        self.index,
                        self.buffer.get_value(self.index).add_overflow(x))
                },
                Instruction::DecreaseValue(x) => {
                    self.buffer.set_value(
                        self.index,
                        self.buffer.get_value(self.index).sub_overflow(x))
                },
                Instruction::IoRead => {
                    self.buffer.set_value(
                        self.index,
                        self.console_io.read()
                    )
                },
                Instruction::IoWrite => {
                    let value = self.buffer.get_value(self.index);
                    self.buffer.set_value(
                        self.index,
                        self.console_io.write(value)
                    )
                },
                Instruction::LoopStart(x) => {
                    let break_value: T = T::default();
                    if self.buffer.get_value(self.index) == break_value {
                        inst_ptr += *x;
                    }
                }
                Instruction::LoopEnd(x) => {
                    let break_value: T = T::default();
                    if self.buffer.get_value(self.index) != break_value {
                        inst_ptr -= *x;
                    }
                }
                Instruction::FileIoRead => {
                    self.buffer.set_value(
                        self.index,
                        self.file_io.read()
                    )
                },
                Instruction::FileIoWrite => {
                    let value = self.buffer.get_value(self.index);
                    self.buffer.set_value(
                        self.index,
                        self.file_io.write(value)
                    )
                }
            }

            inst_ptr += 1;
        }
    }

    pub fn dump_memory(&self) {
        println!("{:?}", self.buffer);
    }
}
