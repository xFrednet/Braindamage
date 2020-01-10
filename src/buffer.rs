use std::io;
use crate::{BufType, ARRAY_SIZE};
use std::num::Wrapping;
use std::fmt::{Debug, Formatter, Error};
use std::cmp::{min};

pub struct BrainfuckBuffer {
    buffer: Vec<BufType>,
    pub index: usize
}

impl BrainfuckBuffer {
    pub fn inc_index(&mut self, value: usize) {
        self.index = (Wrapping(self.index) + Wrapping(value)).0 % crate::ARRAY_SIZE;
    }

    pub fn dec_index(&mut self, value: usize) {
        self.index = (Wrapping(self.index) - Wrapping(value)).0 % crate::ARRAY_SIZE;
    }

    pub fn inc_value(&mut self, value: BufType) {
        self.buffer[self.index] = (Wrapping(self.buffer[self.index]) + Wrapping(value)).0;
    }

    pub fn dec_value(&mut self, value: BufType) {
        self.buffer[self.index] = (Wrapping(self.buffer[self.index]) - Wrapping(value)).0;
    }

    pub fn read(&mut self) {
        let mut buffer = String::new();
        let result = io::stdin().read_line(&mut buffer);

        if result.is_ok() {
            self.buffer[self.index] = buffer.bytes().next().unwrap_or_default();
        }
    }

    pub fn write(&mut self) {
        print!("{}", char::from(self.buffer[self.index]))
    }

    pub fn get_value(&self, index: usize) -> u8 {
        // TODO Deal with high index values >= ARRAY_SIZE

        self.buffer[index]
    }
}

impl Default for BrainfuckBuffer {
    fn default() -> Self {
        let mut vec = Vec::new();
        vec.resize(crate::ARRAY_SIZE, BufType::default());

        BrainfuckBuffer {
            buffer: vec,
            index: 0
        }
    }
}

impl Debug for BrainfuckBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.write_buffer(f, 0, 4096)
    }
}

impl BrainfuckBuffer {
    fn write_buffer(&self, f: &mut Formatter<'_>, start: usize, end: usize) -> Result<(), Error> {
        const ROW_SIZE: usize = 32;
        const SEPARATOR: usize = 8;

        write!(f, "Buffer {{\n").unwrap();

        let start = min(start - (start % ROW_SIZE), ARRAY_SIZE);
        let end = min(
            if end % ROW_SIZE == 0 { end } else { end + ROW_SIZE - (end % ROW_SIZE) },
            ARRAY_SIZE);

        let mut row_start = start;
        while row_start < end {
            write!(f, "  {:#08X}:    ", row_start).unwrap();

            let mut ascii = String::new();
            for i in 0..ROW_SIZE {
                let value = self.get_value(row_start + i);
                write!(f, "{:02X} ", value).unwrap();

                // Save the ascii char for the output
                let char_value = char::from(value);
                if char_value.is_alphanumeric() {
                    ascii.push(char_value);
                } else {
                    ascii.push('.');
                }

                // Print extra space for readability
                if (i + 1) % SEPARATOR == 0 {
                    write!(f, " ").unwrap();
                    ascii.push(' ');
                }
            }

            write!(f, "    {}\n", ascii).unwrap();

            row_start += ROW_SIZE;
        }

        write!(f, "}}")
    }
}