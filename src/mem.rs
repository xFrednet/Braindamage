use std::fmt::Write;
use std::str::FromStr;
use std::usize;

use itertools::Itertools;
use regex::Regex;

const HEAP_CELL_SIZE: usize = 3;
const STACK_CELL_SIZE: usize = 2;

#[derive(Debug, Copy, Clone)]
pub struct MemInfo {
    layout: Layout,
}

#[derive(Debug, Copy, Clone)]
enum Layout {
    Uniform(usize),
    StackMachine { heap: usize, stack: usize },
}

impl MemInfo {
    pub fn size(&self) -> usize {
        match self.layout {
            Layout::Uniform(size) => size,
            Layout::StackMachine { heap, stack } => {
                (heap * HEAP_CELL_SIZE) + HEAP_CELL_SIZE + (stack * STACK_CELL_SIZE)
            },
        }
    }

    pub fn start_pos(&self) -> usize {
        match self.layout {
            Layout::StackMachine { heap, .. } => heap * HEAP_CELL_SIZE,
            _ => 0,
        }
    }

    pub fn print_mem(&self, mem: &[u8], head_idx: usize) -> String {
        let mut view =MemView {
            mem,
            head_idx,
            layout: self.layout,
            buffer: String::new(),
        };

        view.stringify();

        view.buffer
    }
}

impl FromStr for MemInfo {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Virtual Machine Memory
        let re = Regex::new(r#"(\d*)H(\d*)S"#).unwrap();
        if let Some(caps) = re.captures(s) {
            return Ok(Self {
                layout: Layout::StackMachine {
                    heap: caps[1].parse().unwrap(),
                    stack: caps[2].parse().unwrap(),
                },
            });
        }

        // Uniform Memory
        let re = Regex::new(r#"(\d*)"#).unwrap();
        if let Some(caps) = re.captures(s) {
            return Ok(Self {
                layout: Layout::Uniform(caps[0].parse().unwrap()),
            });
        }

        Err(format!("Invalid memory config: `{s:?}`"))
    }
}

struct MemView<'a> {
    mem: &'a [u8],
    head_idx: usize,
    layout: Layout,
    buffer: String,
}

impl<'a> MemView<'a> {
    const UNIFORM_BYTES_PER_ROW: usize = 16;
    const VM_STACK_CELLS_PER_ROW: usize = 8;

    fn stringify(&mut self) {
        match self.layout {
            Layout::Uniform(size) => {
                assert_eq!(size, self.mem.len());
                let mut row_start = 0;
                while row_start < size {
                    self.uniform_mem_row(row_start);
                    row_start += Self::UNIFORM_BYTES_PER_ROW;
                }
            },
            Layout::StackMachine { heap, .. } => {
                writeln!(self.buffer, "BF Addr | BF Memory  | V Addr | V Memory | V Text").unwrap();

                let mut bf_idx = heap * HEAP_CELL_SIZE;

                // Heap

                // Home
                self.vm_mem_row(bf_idx, bf_idx + HEAP_CELL_SIZE, HEAP_CELL_SIZE, 0xffff);
                bf_idx += HEAP_CELL_SIZE;

                // Stack
                let mut stack_idx = 0;
                while bf_idx < self.mem.len() {
                    let mut bf_end = bf_idx + (Self::VM_STACK_CELLS_PER_ROW * STACK_CELL_SIZE);
                    bf_end = bf_end.min(self.mem.len());

                    self.vm_mem_row(bf_idx, bf_end, STACK_CELL_SIZE, stack_idx);
                    stack_idx += Self::VM_STACK_CELLS_PER_ROW;
                    bf_idx = bf_end;
                }
            },
        }
    }

    fn uniform_mem_row(&mut self, row_start: usize) {
        const CHUNK_SIZE: usize = 8;

        let row_end = (row_start + Self::UNIFORM_BYTES_PER_ROW).min(self.mem.len());
        let range = row_start..row_end;

        let mut bytes = String::new();
        let mut text = String::new();
        for mut chunk in &range.chunks(CHUNK_SIZE) {
            let start = chunk.next().unwrap();
            let end = chunk.last().unwrap_or(start);

            write!(bytes, "  {}", self.to_hex_str(start..=end)).unwrap();
            write!(text, "  {}", self.to_txt_str(start..=end)).unwrap();
        }

        writeln!(self.buffer, "{row_start:04X} |{bytes}  |{text}").unwrap();
    }
    
    fn vm_mem_row(&mut self, bf_start: usize, bf_end: usize, cell_size: usize, vm_start: usize) {
        let mut idx = bf_start;
        let mut vm_indices = vec![];
        let mut bf_bytes = String::new();
        while idx < bf_end {
            vm_indices.push(idx + 1);
            write!(bf_bytes, " {}", self.to_hex_str(idx..(idx + cell_size))).unwrap();
            idx += cell_size;
        }

        let vm_bytes = self.to_hex_str(vm_indices.iter().copied());
        let vm_text = self.to_txt_str(vm_indices.iter().copied());

        writeln!(self.buffer, "{bf_start:04X} |{bf_bytes}  | {vm_start:04X} | {vm_bytes} | {vm_text}").unwrap();
    }

    fn to_hex_str(&self, mem_indices: impl Iterator<Item = usize>) -> String {
        let mut bytes = String::new();
        for idx in mem_indices {
            let prefix = if idx == self.head_idx { '>' } else { ' ' };
            let value = self.mem[idx];
            write!(bytes, "{prefix}{value:02X}").unwrap();
        }
        bytes
    }

    fn to_txt_str(&self, mem_indices: impl Iterator<Item = usize>) -> String {
        let mut txt = String::new();
        for idx in mem_indices {
            let char_value = char::from(self.mem[idx]);
            if char_value.is_alphanumeric() {
                txt.push(char_value);
            } else {
                txt.push('.');
            }
        }
        txt
    }
}
