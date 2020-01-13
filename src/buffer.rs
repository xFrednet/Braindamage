use crate::{ARRAY_SIZE};
use crate::cell::Cell;
use std::fmt::{Formatter, Error, Debug};
use std::cmp::min;

pub struct VecBuffer<T: Cell> {
    buffer: Vec<T>,
    pub index: usize
}

impl<T> VecBuffer<T>
    where
        T: Cell
{
    pub fn set_value(&mut self, index: usize, value: T) {
        self.buffer[index] = value;
    }
    pub fn get_value(&self, index: usize) -> T {
        self.buffer[index]
    }

    pub fn write_buffer(&self, f: &mut Formatter<'_>, start: usize, end: usize) -> Result<(), Error> {
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
                let char_value: char = value.to_char();
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

impl<T> Default for VecBuffer<T>
    where
        T: Cell
{
    fn default() -> Self {
        let mut vec: Vec<T> = Vec::new();
        vec.resize(crate::ARRAY_SIZE, T::default());

        VecBuffer {
            buffer: vec,
            index: 0
        }
    }
}

impl<T> Debug for VecBuffer<T>
    where
        T: Cell
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.write_buffer(f, 0, 128)
    }
}