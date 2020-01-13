use crate::operations::io::BraindamageIo;

//    +,-
//   {^.°}
//   <[@]>
//    : ;

use crate::buffer::VecBuffer;
use crate::{Instruction, Cell, ARRAY_SIZE};
use crate::operations::io::console_io::ConsoleIo;
use crate::operations::io::file_io::FileIo;

pub struct Interpreter<'a, T: Cell> {
    buffer: VecBuffer<T>,
    index: usize,

    instructions: &'a Vec<Instruction<T>>,

    console_io: Box<dyn BraindamageIo<T>>,
    #[allow(dead_code)]
    file_io: Box<dyn BraindamageIo<T>>,

}

impl<'a, T> Interpreter<'a, T>
    where T: Cell
{
    const DEFAULT_FILE: &'static str = "bd_data.txt";

    pub fn new(instructions: &'a Vec<Instruction<T>>) -> Self {
        Self::new_with_io(
            instructions,
            Box::new(ConsoleIo::new()),
            Box::new(FileIo::new(&Self::DEFAULT_FILE))
        )
    }

    pub fn new_with_io(
        instructions: &'a Vec<Instruction<T>>,
        console_io: Box<dyn BraindamageIo<T>>,
        file_io: Box<dyn BraindamageIo<T>>) -> Self
    {
        Interpreter {
            buffer: VecBuffer::default(),
            index: 0,

            instructions,

            console_io,
            file_io
        }
    }

    pub fn run(&mut self) {
        self.execute(self.instructions);

        println!("{:?}", self.buffer);
    }

    fn execute(&mut self, instructions: &Vec<Instruction<T>>) {
        for inst in instructions {
            match inst {
                Instruction::NoOp => {},
                Instruction::IncreaseIndex(x) => { self.index = (self.index + *x) % ARRAY_SIZE },
                Instruction::DecreaseIndex(x) => { self.index = (self.index - *x) % ARRAY_SIZE },
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
                Instruction::Loop(instructions) => {
                    let break_value: T = T::default();
                    while self.buffer.get_value(self.index) != break_value {
                        self.execute(instructions);
                    }
                },
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
        }
    }
}
