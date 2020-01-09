use std::io;
use crate::{BufType};

#[derive(Debug)]
pub struct BrainfuckBuffer {
    buffer: Vec<BufType>,
    index: usize
}

impl BrainfuckBuffer {
    pub fn inc_index(&mut self, value: usize) {
        self.index += value;
    }

    pub fn dec_index(&mut self, value: usize) {
        self.index -= value;
    }

    pub fn inc_value(&mut self, value: BufType) {
        self.buffer[self.index] += value;
    }

    pub fn dec_value(&mut self, value: BufType) {
        self.buffer[self.index] -= value;
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

    pub fn get_value(&self) -> u8 {
        self.buffer[self.index]
    }
}

impl Default for BrainfuckBuffer {
    fn default() -> Self {
        let mut vec = Vec::new();
        vec.resize(30_000, BufType::default());

        BrainfuckBuffer {
            buffer: vec,
            index: 0
        }
    }
}
